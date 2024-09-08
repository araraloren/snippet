use std::{
    path::{Path, PathBuf},
    process::{Output, Stdio},
    str::FromStr,
};

use cote::aopt::raise_error;
use tokio::{fs::read_dir, io::AsyncWriteExt, process::Command};
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

pub async fn fmt_and_display_code(
    fmt: String,
    cat: Option<String>,
    codes: Vec<String>,
) -> cote::Result<()> {
    let (fmt_command, fmt_args) = fmt.split_once('=').unwrap_or((&fmt, ""));
    let fmt_args: Vec<_> = if fmt_args.is_empty() {
        vec![]
    } else {
        fmt_args.split(';').collect()
    };
    let input = codes.join("\n");
    let output = run_command(fmt_command, &fmt_args, &input, true)
        .await
        .map_err(|e| raise_error!("run fmt command failed: {e:?}"))?;
    let Output {
        status,
        stdout,
        stderr: _,
    } = output;

    println!("-----------------------------------");
    if status.success() {
        let stdout = String::from_utf8(stdout).map_err(|e| raise_error!("invalid utf8: {e:?}"))?;

        if let Some(cat) = cat {
            let (cat_command, cat_args) = cat.split_once('=').unwrap_or((&cat, ""));
            let cat_args: Vec<_> = if cat_args.is_empty() {
                vec![]
            } else {
                cat_args.split(';').collect()
            };

            let ret = run_command(cat_command, &cat_args, &stdout, false)
                .await
                .map_err(|e| raise_error!("run cat command failed: {e:?}"))?;

            if ret.status.success() {
                return Ok(());
            }
        }

        println!("{}", stdout);
    } else {
        for code in codes {
            println!("{}", code);
        }
    }

    Ok(())
}

pub async fn run_command(
    cmd: &str,
    args: &[&str],
    input: &str,
    stdout: bool,
) -> std::io::Result<Output> {
    let mut cmd = Command::new(cmd);

    cmd.args(args);
    if !input.is_empty() {
        cmd.stdin(Stdio::piped());
    }
    if stdout {
        cmd.stdout(Stdio::piped());
    }
    let mut child = cmd.spawn()?;

    if !input.is_empty() {
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(format!("{}\n", input).as_bytes()).await?;
        }
    }

    child.wait_with_output().await
}
