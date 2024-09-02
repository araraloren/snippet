pub mod comp;
pub mod host;

use comp::find_plugins;
use comp::link_component;
use comp::Plugin;
use comp::Plugins;
use cote::prelude::*;
use host::types::Lang;
use host::Root;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tracing::debug;
use tracing::field::debug;
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

                    if let Ok(lang) = parser.find_val::<String>("lang@1") {
                        let lang = Lang::from_str(lang).map_err(|e| {
                            raise_error!("not support given language {lang}: {e:?}")
                        })?;
                        let help = parser.find_val("--help").copied().unwrap_or_default();
                        let dir = parser
                            .find_val::<String>("--path")
                            .map(|v| v.as_str())
                            .unwrap_or(".");
                        let mut args = ret.take_args().to_vec();

                        tracing::debug!(
                            "running language `{:?}`, search plugins in `{}`",
                            lang,
                            dir
                        );
                        if help {
                            args.insert(0, RawVal::from("--help"));
                        }
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
            if run_compiler(compiler, language, args.clone()).await.is_ok() {
                return Ok(());
            }
        }
    }
    Ok(())
}

pub async fn run_compiler(comp: &Plugin, lang: &Plugin, args: Vec<RawVal>) -> wasmtime::Result<()> {
    let data = link_component(&comp.path, &lang.path)?;

    let mut config = Config::new();
    // Instantiate the engine and store
    let engine = wasmtime::Engine::new(config.async_support(true))?;

    // Create a linker
    let mut linker = Linker::<State>::new(&engine);
    let closure = type_annotate::<State, _>(|t| WasiImpl(t));

    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    host::types::add_to_linker_get_host(&mut linker, closure)?;
    //bindings::snippet::c::compiler::add_to_linker(&mut linker, |a| a)?;

    // Create a store
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

    // Load component
    let lang = Component::from_binary(&engine, &data)?;
    //let compiler = Component::from_file(&engine, &compiler)?;

    // Instantiate the component
    let bindings = Root::instantiate_async(&mut store, &lang, &linker).await?;

    // Call the `greet` function
    let result = bindings
        .snippet_plugin_language()
        .call_name(&mut store)
        .await?;

    // This should print out `Greeting: [String("Hello, Alice!")]`
    println!("Greeting: {:?}", result);

    let langs = bindings
        .snippet_plugin_compiler()
        .call_support(&mut store)
        .await?;

    println!("--> supports: {:?}", langs);

    let optset = bindings
        .snippet_plugin_language()
        .call_initialize_optset(&mut store)
        .await??;

    bindings
        .snippet_plugin_language()
        .call_fill_optset(&mut store, optset)
        .await??;
    let optset = bindings
        .snippet_plugin_language()
        .call_initialize_optset(&mut store)
        .await??;
    let complier = bindings
        .snippet_plugin_compiler()
        .compiler()
        .call_constructor(&mut store)
        .await?;
    let res = bindings
        .snippet_plugin_language()
        .call_compile(&mut store, optset, complier)
        .await??;

    dbg!(res);
    Ok(())
}
