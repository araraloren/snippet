pub mod comp;
pub mod host;

use comp::find_plugins;
use comp::link_component;
use comp::Plugin;
use comp::Plugins;
use cote::prelude::*;
use host::types::Binary;
use host::types::Lang;
use host::types::Object;
use host::types::Source;
use host::types::Target;
use host::Root;
use std::env::current_dir;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs::read_to_string;
use tokio::fs::remove_file;
use tokio::process::Command;
use tracing_subscriber::filter::LevelFilter;
use wasmtime::component::*;
use wasmtime::Config;
use wasmtime::Store;
use wasmtime_wasi::ResourceTable;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::{WasiImpl, WasiView};

pub struct State {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

// NB: workaround some rustc inference - a future refactoring may make this
// obsolete.
fn type_annotate<T: WasiView, F>(val: F) -> F
where
    F: Fn(&mut T) -> WasiImpl<&mut T>,
{
    val
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let subscriber = tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_filter_reloading();
    let reload_handler = subscriber.reload_handle();
    let mut parser = Parser::<'_, ASet, ASer>::default();

    parser.add_opt("lang@1: language to be execute: c, c++, rust".infer::<Pos<String>>())?;
    parser.add_opt("--debug=b: display debug log message")?;
    parser.add_opt("--help=b: display help message")?;
    parser.add_opt("--path: search plugins in given path".infer::<PathBuf>())?;
    parser.add_opt("--comp: using given compiler plugin".infer::<PathBuf>())?;
    parser.add_opt("--lang: using given language plugin".infer::<PathBuf>())?;
    subscriber.init();

    Ok(parser
        .run_async_mut(&mut PrePolicy::default(), |mut ret, parser| {
            let reload_handler = reload_handler.clone();
            let print_help = |parser: &mut Parser<'_, ASet, ASer>| {
                parser.display_help(
                    env!("CARGO_PKG_AUTHORS"),
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_DESCRIPTION"),
                )
            };

            async move {
                if ret.status() {
                    if *parser.find_val::<bool>("--debug")? {
                        reload_handler
                            .modify(|filter| {
                                *filter = tracing_subscriber::EnvFilter::from_default_env()
                                    .add_directive(LevelFilter::DEBUG.into());
                            })
                            .map_err(|e| {
                                raise_error!("can not modify tracing subscriber: {e:?}")
                            })?;
                    }
                    let help = parser.find_val("--help").copied().unwrap_or_default();

                    if help {
                        print_help(parser)?;
                        Ok(())
                    } else if let Ok(lang) = parser.find_val::<String>("lang@1") {
                        let lang = Lang::from_str(lang).map_err(|e| {
                            raise_error!("not support given language {lang}: {e:?}")
                        })?;

                        let dir = parser
                            .find_val::<String>("--path")
                            .map(|v| v.as_str())
                            .unwrap_or(".");
                        let args = ret.take_args().to_vec();

                        tracing::debug!(
                            "running language `{:?}`, search plugins in `{}`",
                            lang,
                            dir
                        );

                        find_compiler_and_try(dir, lang, args).await
                    } else {
                        eprintln!("Which language do you want to execute: c, c++ or rust?");
                        print_help(parser)?;
                        Ok(())
                    }
                } else {
                    print_help(parser)?;
                    ret.ok()?;
                    Ok(())
                }
            }
        })
        .await?)
}

pub async fn find_compiler_and_try(dir: &str, lang: Lang, args: Vec<RawVal>) -> cote::Result<()> {
    let Plugins { compiler, language } = find_plugins(Path::new(dir)).await?;
    let compilers: Vec<_> = compiler.into_iter().filter(|v| v.lang == lang).collect();
    let languages: Vec<_> = language.into_iter().filter(|v| v.lang == lang).collect();

    if compilers.is_empty() {
        return Err(raise_error!(
            "can not find compiler support language `{:?}`",
            lang
        ));
    } else if languages.is_empty() {
        return Err(raise_error!(
            "can not find language support language `{:?}`",
            lang
        ));
    }

    for compiler in &compilers {
        for language in &languages {
            match run_compiler(compiler, language, args.clone(), lang).await {
                Ok(ret) => {
                    if ret {
                        return Ok(());
                    }
                }
                Err(e) => {
                    tracing::debug!(
                        "running compiler({}), language({}) failed: {e:?}",
                        compiler.path.display(),
                        language.path.display()
                    );
                }
            }
        }
    }
    Ok(())
}

pub async fn run_compiler(
    compiler: &Plugin,
    language: &Plugin,
    args: Vec<RawVal>,
    lang: Lang,
) -> wasmtime::Result<bool> {
    let component = link_component(&compiler.path, &language.path)?;

    let mut config = Config::new();
    let engine = wasmtime::Engine::new(config.async_support(true))?;

    let mut linker = Linker::<State>::new(&engine);
    let closure = type_annotate::<State, _>(|t| WasiImpl(t));

    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    host::types::add_to_linker_get_host(&mut linker, closure)?;

    let mut store = Store::new(
        &engine,
        State {
            ctx: WasiCtxBuilder::new()
                .inherit_stdin()
                .inherit_stdout()
                .build(),
            table: ResourceTable::new(),
        },
    );
    let component = Component::from_binary(&engine, &component)?;
    let plugin = Root::instantiate_async(&mut store, &component, &linker).await?;
    let language_lang = plugin
        .snippet_plugin_language()
        .call_name(&mut store)
        .await?;
    let compiler_lang = plugin
        .snippet_plugin_compiler()
        .call_support(&mut store)
        .await?;

    if !(language_lang == lang && compiler_lang == language_lang) {
        return Ok(false);
    }

    // initialize the optset
    let optset = plugin
        .snippet_plugin_language()
        .call_initialize_optset(&mut store)
        .await??;
    // fill the optset
    let optset = plugin
        .snippet_plugin_language()
        .call_fill_optset(&mut store, optset)
        .await??;

    let ret = store.data_mut().table().get_mut(&optset)?.parse(args)?;

    if !ret {
        return Ok(false);
    }

    let inner_optset = store.data_mut().table().get(&optset)?;
    let display: bool = inner_optset
        .parser
        .find_val("-p")
        .copied()
        .unwrap_or_default();

    let complier = plugin
        .snippet_plugin_compiler()
        .compiler()
        .call_constructor(&mut store)
        .await?;
    let res = plugin
        .snippet_plugin_language()
        .call_compile(&mut store, optset, complier)
        .await??;

    let Target {
        clean,
        output,
        codes,
        cmd_result,
    } = res;

    if display {
        println!("-----------------------------------");
        for code in codes {
            println!("{}", code);
        }
        println!("-----------------------------------");
    }
    if cmd_result.ret == 0 {
        match &output {
            host::types::Output::Binary(Binary { path, args }) => {
                tracing::debug!(
                    "running binary `{}` with args: `{:?}` in cwd `{}`",
                    path,
                    args,
                    current_dir()?.display()
                );
                let path = std::path::absolute(path)?;
                let mut cmd = Command::new(path);
                //let cmd = cmd.args(args);
                let mut child = cmd.spawn()?;

                let ret = child.wait().await?;

                tracing::debug!("running ret = {:?}", ret);
            }
            host::types::Output::Source(Source { path }) => {
                println!("{}", read_to_string(path).await?);
            }
            host::types::Output::Object(Object { path: _ }) => {}
        }
    }
    if clean {
        match &output {
            host::types::Output::Binary(Binary { path, args: _ }) => {
                remove_file(path).await?;
            }
            host::types::Output::Source(Source { path }) => {
                remove_file(path).await?;
            }
            host::types::Output::Object(Object { path }) => {
                remove_file(path).await?;
            }
        }
    }
    Ok(true)
}
