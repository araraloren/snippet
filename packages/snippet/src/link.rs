use std::path::Path;
use wac_graph::{types::Package, CompositionGraph, EncodeOptions};

pub fn link_component(a: &Path, b: &Path) -> wasmtime::Result<Vec<u8>> {
    let mut graph = CompositionGraph::new();
    let compiler = Package::from_file("compiler_c", None, a, graph.types_mut())?;
    let compiler = graph.register_package(compiler)?;
    let language = Package::from_file("language_c", None, b, graph.types_mut())?;
    let language = graph.register_package(language)?;
    let compiler_ins = graph.instantiate(compiler);
    let language_ins = graph.instantiate(language);

    let comp_comp = graph.alias_instance_export(compiler_ins, "snippet:plugin/compiler@0.1.0")?;

    graph.set_instantiation_argument(language_ins, "snippet:plugin/compiler@0.1.0", comp_comp)?;

    let lang_lang = graph.alias_instance_export(language_ins, "snippet:plugin/language@0.1.0")?;

    graph.export(lang_lang, "snippet:plugin/language@0.1.0")?;
    graph.export(comp_comp, "snippet:plugin/compiler@0.1.0")?;

    Ok(graph.encode(EncodeOptions::default())?)
}
