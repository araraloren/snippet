pub mod comm;
pub mod host;

use comm::find_plugins;
use comm::link_component;
use comm::Plugins;
use cote::prelude::*;
use host::types::Binary;
use host::types::Lang;
use host::types::Object;
use host::types::Source;
use host::types::Target;
use host::Root;
use std::env::current_dir;
use std::env::current_exe;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::read_to_string;
use tokio::fs::remove_file;
use tokio::fs::File;
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

    parser.set_name("snippet");
    parser.add_opt("f;fetch=cmd: fetch plugin from github release")?;
    parser.add_opt("c=cmd: load c language plugin, generate and execute c language")?;
    parser
        .add_opt("cc;cpp;c++=cmd: load c++ language plugin, generate and execute c++ language")?;
    parser.add_opt("rs;rust=cmd: load rust language plugin, generate and execute rust language")?;
    parser.add_opt(
        "--name=s: select a compiler if the language has multiple compilers, such as --name=gcc",
    )?;
    parser.add_opt("--debug=b: display debug log message")?;
    parser.add_opt("--help=b: display help message")?;
    parser.add_opt("--path: search plugins in given path".infer::<PathBuf>())?;
    parser.add_opt("--comp: set the path of compiler plugin".infer::<PathBuf>())?;
    parser.add_opt("--lang: set the path of language plugin".infer::<PathBuf>())?;
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
                    let c_cmd = parser.find_val::<bool>("c").ok();
                    let cxx_cmd = parser.find_val::<bool>("cpp").ok();
                    let rust_cmd = parser.find_val::<bool>("rust").ok();
                    let fetch_cmd = parser.find_val::<bool>("fetch").ok();
                    let sel_comp = parser.find_val::<PathBuf>("--comp").ok();
                    let sel_lang = parser.find_val::<PathBuf>("--lang").ok();
                    let sel_name = parser.find_val::<String>("--name").map(|v| v.as_str()).ok();
                    let current_exe = current_exe()
                        .map_err(|e| raise_error!("can not get current exe directory: {e:?}"))?;
                    let plugin_directory = current_exe.parent().ok_or_else(|| {
                        raise_error!("can not find parent directory of current exe")
                    })?;
                    let plugin_dir = parser
                        .find_val::<PathBuf>("--path")
                        .map(|v| v.as_ref())
                        .unwrap_or(plugin_directory);
                    let mut args = ret.take_args().to_vec();

                    if help {
                        print_help(parser)?;
                        Ok(())
                    } else if fetch_cmd == Some(&true) {
                        args.remove(1);
                        args.remove(0);
                        let args: Vec<_> = args
                            .into_iter()
                            .map(|v| v.to_string_lossy().to_string())
                            .collect();

                        download_plugin_from_release(&args)
                            .await
                            .map_err(|e| raise_error!("can not download plugin: {e:?}"))
                    } else if c_cmd == Some(&true) {
                        tracing::debug!(
                            "running language c, search plugins in `{}`",
                            plugin_dir.display()
                        );
                        args.remove(1);
                        find_and_try(plugin_dir, Lang::C, args, sel_comp, sel_lang, sel_name).await
                    } else if cxx_cmd == Some(&true) {
                        tracing::debug!(
                            "running language c++, search plugins in `{}`",
                            plugin_dir.display()
                        );
                        args.remove(1);
                        find_and_try(plugin_dir, Lang::Cxx, args, sel_comp, sel_lang, sel_name)
                            .await
                    } else if rust_cmd == Some(&true) {
                        tracing::debug!(
                            "running language rust, search plugins in `{}`",
                            plugin_dir.display()
                        );
                        args.remove(1);
                        find_and_try(plugin_dir, Lang::Rust, args, sel_comp, sel_lang, sel_name)
                            .await
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

const PLUGINS_LIST_URL: &str =
    "https://raw.githubusercontent.com/araraloren/snippet/main/packages/snippet/plugins.ini";

pub async fn download_plugin_from_release(plugins: &[String]) -> color_eyre::Result<()> {
    let pluigns_list = reqwest::get(PLUGINS_LIST_URL).await?;
    let plugins_list = pluigns_list.text().await?;
    let current_exe = current_exe()?;
    let current_exe_dir = current_exe
        .parent()
        .ok_or_else(|| raise_error!("can not find parent directory of current exe"))?;

    tracing::debug!("download plugins: {:?}", plugins);
    for plugin in plugins {
        for line in plugins_list.lines() {
            let (name, url) = line.split_once('=').unwrap();

            if name == *plugin {
                let resp = reqwest::get(url).await?;
                let data = resp.bytes().await?;
                let path = current_exe_dir.join(format!("{}.wasm", name));

                if path.exists() {
                    println!("remove old plugin `{}`", path.display());
                    remove_file(&path).await?;
                }
                let mut out = File::options()
                    .create_new(true)
                    .write(true)
                    .open(&path)
                    .await?;

                tracing::debug!("download plugin `{}` from {}", plugin, url);
                tokio::io::copy(&mut data.as_ref(), &mut out).await?;
                println!("download plugin `{plugin}` successed");
                println!("plugin {} saved", path.display());
                break;
            } else {
                tracing::debug!("can not find plugin `{}`", plugin);
            }
        }
    }

    Ok(())
}

pub async fn find_and_try(
    dir: &Path,
    lang: Lang,
    args: Vec<RawVal>,
    selected_comp: Option<&PathBuf>,
    selected_lang: Option<&PathBuf>,
    selected_name: Option<&str>,
) -> cote::Result<()> {
    let compiler = selected_comp.map(|v| v.as_ref());
    let language = selected_lang.map(|v| v.as_ref());

    if let Some((compiler, language)) = compiler.zip(language) {
        run_compiler(compiler, language, args, lang, selected_name)
            .await
            .map_err(|e| raise_error!("run compiler failed: {e:?}"))?;
    } else {
        let mut last_error = None;
        let Plugins {
            compiler: compilers,
            language: languages,
        } = find_plugins(dir).await?;

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
        tracing::debug!("args for compiler: {args:?}");
        for compiler_plugin in &compilers {
            for language_plugin in &languages {
                let compiler = compiler.unwrap_or(&compiler_plugin.path);
                let language = language.unwrap_or(&language_plugin.path);

                tracing::debug!(
                    "run compiler: {} and language: {}",
                    compiler.display(),
                    language.display()
                );
                match run_compiler(compiler, language, args.clone(), lang, selected_name).await {
                    Ok(ret) => {
                        if ret {
                            return Ok(());
                        } else {
                            tracing::debug!("run failed...");
                        }
                    }
                    Err(e) => {
                        tracing::debug!("got error: {e:?}",);
                        last_error = Some(e);
                    }
                }
            }
        }
        if let Some(error) = last_error {
            return Err(raise_error!("failed to run compiler: {error:?}"));
        }
    }
    Ok(())
}

pub async fn run_compiler(
    compiler: &Path,
    language: &Path,
    args: Vec<RawVal>,
    lang: Lang,
    selected_name: Option<&str>,
) -> wasmtime::Result<bool> {
    let component = link_component(compiler, language)?;

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
        tracing::debug!("not a {:?} compiler", lang);
        return Ok(false);
    }

    if let Some(selected_name) = selected_name {
        if plugin
            .snippet_plugin_compiler()
            .call_name(&mut store)
            .await?
            != selected_name
        {
            tracing::debug!("not name {} selected by user", selected_name);
            return Ok(false);
        }
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

    let ret = store
        .data_mut()
        .table()
        .get_mut(&optset)?
        .parse(lang, args)?;

    if !ret {
        return Ok(false);
    }

    let inner_optset = store.data_mut().table().get(&optset)?;
    let display: bool = inner_optset
        .parser
        .find_val("-p")
        .copied()
        .unwrap_or_default();
    let fmt: Result<String, cote::Error> = inner_optset.parser.find_val("-fmt=s").cloned();
    let cat: Result<String, cote::Error> = inner_optset.parser.find_val("-cat=s").cloned();

    let complier = plugin
        .snippet_plugin_compiler()
        .compiler()
        .call_constructor(&mut store)
        .await?;
    let res = plugin
        .snippet_plugin_language()
        .call_compile(&mut store, optset, complier)
        .await??;
    let lang_fmt = plugin
        .snippet_plugin_language()
        .call_fmt(&mut store)
        .await?;

    tracing::debug!("got res => {res:?}");

    let Target {
        clean,
        output,
        codes,
        cmd_result,
    } = res;

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

                cmd.args(args);
                let mut child = cmd.spawn()?;

                let ret = child.wait().await?;

                println!();
                tracing::debug!("running ret = {:?}", ret);
            }
            host::types::Output::Source(Source { path }) => {
                println!("{}", read_to_string(path).await?);
            }
            host::types::Output::Object(Object { path: _ }) => {}
        }
    }
    if clean {
        tracing::debug!("remove target => {output:?}");
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
    if display && !codes.is_empty() {
        let fmt = fmt.unwrap_or(lang_fmt);
        let cat = cat.ok();
        tracing::debug!("display code with fmt: {fmt} and cat: {cat:?}");
        comm::fmt_and_display_code(fmt, cat, codes).await?;
    }
    Ok(true)
}
