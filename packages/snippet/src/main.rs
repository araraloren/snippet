pub mod comp;
pub mod host;

use comp::collect_plugins;
use comp::link_component;
use cote::prelude::*;
use host::types::Lang;
use host::Root;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
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

    parser.add_opt("c=cmd: execute c code")?;
    parser.add_opt("cxx;cpp=cmd: execute c++ code")?;
    parser.add_opt("rs;rust=cmd: execute rust code")?;
    parser.add_opt("--debug=b: display debug log message")?;
    parser.add_opt("-h;--help=b: display help message")?;
    subscriber.init();

    Ok(parser
        .run_async_mut(&mut PrePolicy::default(), |mut ret, parser| {
            let reload_handler = reload_handler.clone();

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

                    let lang = if *parser.find_val::<bool>("c")? {
                        Some(Lang::C)
                    } else if *parser.find_val::<bool>("cxx")? {
                        Some(Lang::Cxx)
                    } else if *parser.find_val::<bool>("rust")? {
                        Some(Lang::Rust)
                    } else {
                        None
                    };
                    let help: bool = *parser.find_val("--help")?;

                    tracing::debug!("running language {:?}", lang);
                    if let Some(lang) = lang {
                        let mut args = ret.take_args().to_vec();

                        if help {
                            args.insert(0, RawVal::from("--help"));
                        }
                        run_compiler(lang, args)
                            .await
                            .map_err(|e| raise_error!("can not running command: {e:?}"))?
                    }
                }
                parser.display_help(
                    env!("CARGO_PKG_AUTHORS"),
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_DESCRIPTION"),
                )?;
                Ok(())
            }
        })
        .await?)
}

pub async fn run_compiler(lang: Lang, args: Vec<RawVal>) -> wasmtime::Result<()> {
    let plugins = collect_plugins(Path::new(".")).await?;

    dbg!(&plugins);

    let data: &[u8] = todo!(); // link_component(&compiler, &lang)?;

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
