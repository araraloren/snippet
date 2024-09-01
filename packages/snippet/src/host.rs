use std::ffi::OsStr;
use std::path::Path;

use cote::prelude::ASer;
use cote::prelude::ASet;
use cote::prelude::Parser;
use cote::prelude::SetValueFindExt;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use wasmtime::component::*;
use wasmtime_wasi::WasiImpl;
use wasmtime_wasi::WasiView;

bindgen!({
    world: "root",
    path: "wit",
    async: true,
    with: {
        "snippet:plugin/types/optset": crate::host::OptSet,
        "snippet:plugin/types/services": crate::host::Services,
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

pub use exports::snippet::plugin::compiler;
pub use exports::snippet::plugin::language;
pub use snippet::plugin::types;

use types::CmdResult;
use types::ErrorType;
use types::Lang;
use types::Mode;

#[derive(Debug, Default)]
pub struct OptSet {
    parser: Parser<'static, ASet, ASer>,
}

impl OptSet {
    pub fn new() -> Result<Self, cote::Error> {
        let mut parser = Parser::<'_, ASet, ASer>::default();

        parser.add_opt("-S=b: pass -S to compiler")?;
        parser.add_opt("-E=b: pass -E to compiler")?;
        parser.add_opt("-e=s: append code to generator")?;
        parser.add_opt("-r=b: ignore value of -e, read code from stdin")?;
        parser.add_opt("-end=s: set input code terminator")?;
        parser.add_opt("-o=s: set output name")?;
        parser.add_opt("-p=b: display code before display stdout")?;
        parser.add_opt("-l=s: pass -l to compiler, link given library")?;
        parser.add_opt("-L=s: pass -L to compiler, add link library searh path")?;
        parser.add_opt("-f=s: pass given flag -<f> to compiler")?;
        parser.add_opt("-flag=s: pass given flag --<flag> to compiler")?;
        parser.add_opt("-std=s: set language standard version")?;
        parser.add_opt("-c=s: select given compiler")?;
        parser.add_opt("-m=s: change the main function header")?;

        Ok(Self { parser })
    }
}

#[async_trait::async_trait]
impl<T: WasiView> types::HostOptset for WasiImpl<T> {
    async fn new(&mut self) -> Resource<OptSet> {
        self.table().push(OptSet::default()).unwrap()
    }

    async fn default(&mut self) -> Result<Resource<OptSet>, ErrorType> {
        let optset = OptSet::new().map_err(|_| ErrorType::CreateOptsetFailed)?;

        self.table()
            .push(optset)
            .map_err(|_| ErrorType::InvalidOptsetResource)
    }

    async fn add_opt(&mut self, self_: Resource<OptSet>, opt: String) -> Result<u64, ErrorType> {
        let optset = self
            .table()
            .get_mut(&self_)
            .map_err(|_| ErrorType::InvalidOptsetResource)?;

        Ok(optset
            .parser
            .add_opt(opt)
            .and_then(|v| v.run())
            .map_err(|_| ErrorType::CommandInvokeFailed)?)
    }

    async fn get_value_str(
        &mut self,
        self_: Resource<OptSet>,
        name: String,
    ) -> Result<String, ErrorType> {
        let optset = self
            .table()
            .get_mut(&self_)
            .map_err(|_| ErrorType::InvalidOptsetResource)?;

        optset
            .parser
            .find_val::<String>(name)
            .map_err(|_| ErrorType::AccessValueFailed)
            .cloned()
    }

    fn drop(&mut self, rep: Resource<OptSet>) -> wasmtime::Result<()> {
        self.table().delete(rep)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Services {
    debug: bool,
    lang: Lang,
    mode: Mode,
    args: Vec<String>,
}

impl Services {
    pub fn self_mut<T: WasiView>(
        wasi: &mut WasiImpl<T>,
        self_: Resource<Services>,
    ) -> Result<&mut Services, ErrorType> {
        wasi.table()
            .get_mut(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)
    }

    pub fn self_ref<T: WasiView>(
        wasi: &mut WasiImpl<T>,
        self_: Resource<Services>,
    ) -> Result<&Services, ErrorType> {
        wasi.table()
            .get(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)
    }
}

#[async_trait::async_trait]
impl<T: WasiView> types::HostServices for WasiImpl<T> {
    async fn new(&mut self) -> Resource<Services> {
        self.table()
            .push(Services {
                debug: false,
                lang: Lang::C,
                mode: Mode::Link,
                args: vec![],
            })
            .unwrap()
    }

    async fn debug(&mut self, self_: Resource<Services>) -> Result<bool, ErrorType> {
        Services::self_ref(self, self_).map(|v| v.debug)
    }

    async fn lang(&mut self, self_: Resource<Services>) -> Result<Lang, ErrorType> {
        Services::self_ref(self, self_).map(|v| v.lang)
    }

    async fn args(&mut self, self_: Resource<Services>) -> Result<Vec<String>, ErrorType> {
        Services::self_ref(self, self_).map(|v| v.args.clone())
    }

    async fn mode(&mut self, self_: Resource<Services>) -> Result<Mode, ErrorType> {
        Services::self_ref(self, self_).map(|v| v.mode)
    }

    async fn set_lang(
        &mut self,
        self_: Resource<Services>,
        language: Lang,
    ) -> Result<(), ErrorType> {
        Services::self_mut(self, self_).map(|v| {
            v.lang = language;
        })
    }

    async fn set_debug(&mut self, self_: Resource<Services>, debug: bool) -> Result<(), ErrorType> {
        Services::self_mut(self, self_).map(|v| {
            v.debug = debug;
        })
    }

    async fn set_mode(&mut self, self_: Resource<Services>, mode: Mode) -> Result<(), ErrorType> {
        Services::self_mut(self, self_).map(|v| {
            v.mode = mode;
        })
    }

    async fn add_arg(&mut self, self_: Resource<Services>, arg: String) -> Result<(), ErrorType> {
        Services::self_mut(self, self_).map(|v| {
            v.args.push(arg);
        })
    }

    async fn add_args(
        &mut self,
        self_: Resource<Services>,
        args: Vec<String>,
    ) -> Result<(), ErrorType> {
        Services::self_mut(self, self_).map(|v| {
            v.args.extend(args);
        })
    }

    async fn find_bin(&mut self, bin: String) -> Result<Vec<u8>, ErrorType> {
        which::which(bin)
            .map_err(|_| ErrorType::InvalidBinary)
            .map(|v| v.as_os_str().as_encoded_bytes().to_vec())
    }

    async fn invoke_cmd(
        &mut self,
        bin: Vec<u8>,
        args: Vec<String>,
        stdin_lines: Vec<String>,
    ) -> Result<CmdResult, ErrorType> {
        let bin = Path::new(unsafe { OsStr::from_encoded_bytes_unchecked(&bin) });
        let mut cmd = Command::new(bin);

        cmd.args(args);
        let mut child = cmd.spawn().map_err(|_| ErrorType::CommandSpawnFailed)?;

        match child.stdin.as_mut() {
            Some(stdin) => {
                for line in stdin_lines {
                    stdin
                        .write_all(format!("{}\n", line).as_bytes())
                        .await
                        .map_err(|_| ErrorType::CommandIoFailed)?;
                }
            }
            None => {
                if stdin_lines.is_empty() {
                    return Err(ErrorType::CommandNeedStdin);
                }
            }
        }

        child
            .wait_with_output()
            .await
            .map_err(|_| ErrorType::CommandInvokeFailed)
            .map(|v| CmdResult {
                err: v.stderr,
                out: v.stdout,
                ret: v.status.code().unwrap_or_default(),
            })
    }

    fn drop(&mut self, rep: Resource<Services>) -> wasmtime::Result<()> {
        self.table().delete(rep)?;
        Ok(())
    }
}

impl<T: WasiView> types::Host for WasiImpl<T> {}
