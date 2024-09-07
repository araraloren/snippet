use std::{
    path::{Path, PathBuf},
    process::Stdio,
    str::FromStr,
};

use cote::aopt::raise_error;
use tokio::{
    fs::read_dir,
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};
use wac_graph::{types::Package, CompositionGraph, EncodeOptions};

use crate::host::types::Lang;

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

#[derive(Debug)]
pub struct Plugin {
    pub lang: Lang,
    pub path: PathBuf,
}

#[derive(Debug, Default)]
pub struct Plugins {
    pub compiler: Vec<Plugin>,
    pub language: Vec<Plugin>,
}

// only search plugins names like `snippet-[compiler|lanuguage]-[*name*].wasm`
pub async fn find_plugins(dir: &Path) -> cote::Result<Plugins> {
    let mut plugins = Plugins::default();
    let mut read_dir = read_dir(dir)
        .await
        .map_err(|e| raise_error!("can not read directory: {e:?}"))?;

    while let Some(dir_entry) = read_dir
        .next_entry()
        .await
        .map_err(|e| raise_error!("can not read next entry: {e:?}"))?
    {
        let path = dir_entry.path();

        if let Some(name) = dir_entry.file_name().to_str() {
            if let Some(ext) = path.extension().and_then(|v| v.to_str()) {
                // only accept wasm
                if ext == "wasm" {
                    let without_ext = name
                        .strip_suffix(".wasm")
                        .ok_or_else(|| raise_error!("can not strip suffix .wasm of {name}"))?;
                    let parts: Vec<_> = without_ext.split(['_', '-']).collect();

                    if parts.len() == 3 {
                        if let Some(["snippet", ty, name]) = parts.get(0..3) {
                            let path = dir.join(path);
                            let lang = Lang::from_str(name).map_err(|e| {
                                raise_error!("not support given language `{name}`: {e:?}")
                            })?;
                            let plugin = Plugin { lang, path };

                            match *ty {
                                "compiler" => {
                                    plugins.compiler.push(plugin);
                                }
                                "language" => {
                                    plugins.language.push(plugin);
                                }
                                _ => {
                                    tracing::warn!("unsupport plugin type `{}`: {}", ty, name);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(plugins)
}

pub async fn display_codes(
    fmt: String,
    cat: Option<String>,
    codes: Vec<String>,
) -> cote::Result<()> {
    let (fmt_command, fmt_args) = fmt
        .split_once('=')
        .ok_or_else(|| raise_error!("invalid fmt string -> {fmt}"))?;
    let fmt_args: Vec<_> = fmt_args.split(';').collect();
    let mut fmt_cmd = Command::new(fmt_command);

    fmt_cmd.args(fmt_args);
    fmt_cmd.stdin(Stdio::piped());
    fmt_cmd.stdout(Stdio::piped());
    if let Ok(mut fmt_child) = fmt_cmd
        .spawn()
        .map_err(|e| raise_error!("can not spawn command: {e:?}"))
    {
        let fmt_stdin = fmt_child
            .stdin
            .as_mut()
            .ok_or_else(|| raise_error!("need write stdin to fmt command"))?;

        for line in codes {
            fmt_stdin
                .write_all(format!("{}\n", line).as_bytes())
                .await
                .map_err(|e| raise_error!("can not write to stdin of fmt: {e:?}"))?;
        }

        let output = fmt_child
            .wait()
            .await
            .map_err(|e| raise_error!("can not wait stdout: {e:?}"))?;

        if !output.success() {
            return Err(raise_error!("running fmt command failed: {output:?}"));
        }

        let mut stdout = fmt_child
            .stdout
            .ok_or_else(|| raise_error!("can not get stdout of fmt command"))?;
        let mut output = String::default();

        stdout
            .read_to_string(&mut output)
            .await
            .map_err(|e| raise_error!("can not read stdout of fmt command: {e:?}"))?;

        if let Some(cat) = cat {
            let (cat_command, cat_args) = cat
                .split_once('=')
                .ok_or_else(|| raise_error!("invalid fmt string -> {cat}"))?;
            let cat_args: Vec<_> = cat_args.split(';').collect();
            let mut cat_cmd = Command::new(cat_command);

            cat_cmd.args(cat_args);
            cat_cmd.stdin(Stdio::piped());
            if let Ok(mut cat_child) = cat_cmd.spawn() {
                let cat_stdin = cat_child
                    .stdin
                    .as_mut()
                    .ok_or_else(|| raise_error!("need write stdin to cat command"))?;

                cat_stdin
                    .write_all(output.as_bytes())
                    .await
                    .map_err(|e| raise_error!("can not write to stdin of cat: {e:?}"))?;
                return Ok(());
            }
        }
        println!("-----------------------------------");
        println!("{}", output);
        println!("-----------------------------------");
    } else {
        println!("-----------------------------------");
        for code in codes {
            println!("{}", code);
        }
        println!("-----------------------------------");
    }

    Ok(())
}
