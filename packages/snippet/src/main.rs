mod utils;
pub mod bindings {
    use wasmtime::component::*;

    bindgen!({
        world: "root",
        path: "wit",
        async: true,
        with: {
            "snippet:plugin/types/optset": crate::utils::OptSet,
            "snippet:plugin/types/services": crate::utils::Services,
        },
        inline: "
            package snippet:snippet;

            world root {
                import snippet:plugin/types@0.1.0;

                export snippet:plugin/compiler@0.1.0;
                export snippet:plugin/language@0.1.0;
            }
        "
    });
}

use bindings::Root;
use cote::prelude::*;
use std::path::PathBuf;
use utils::link_component;
use wasmtime::component::*;
use wasmtime::Config;
use wasmtime::Store;
use wasmtime_wasi::ResourceTable;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::{WasiImpl, WasiView};

#[derive(Debug, Cote)]
pub struct Cli {
    #[pos()]
    compiler: PathBuf,

    #[pos()]
    lang: PathBuf,
}

pub struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
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
async fn main() -> wasmtime::Result<()> {
    let Cli { compiler, lang } = Cli::parse_env()?;
    let data = link_component(&compiler, &lang)?;

    let mut config = Config::new();
    // Instantiate the engine and store
    let engine = wasmtime::Engine::new(config.async_support(true))?;

    // Create a linker
    let mut linker = Linker::<MyState>::new(&engine);
    let closure = type_annotate::<MyState, _>(|t| WasiImpl(t));

    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    bindings::snippet::plugin::types::add_to_linker_get_host(&mut linker, closure)?;
    //bindings::snippet::c::compiler::add_to_linker(&mut linker, |a| a)?;

    // Create a store
    let mut store = Store::new(
        &engine,
        MyState {
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
        .call_support_langs(&mut store)
        .await?;

    println!("--> supports: {:?}", langs);

    Ok(())
}
