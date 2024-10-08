use std::fmt::Debug;
use std::path::Path;
use std::process::Stdio;
use std::str::FromStr;
use std::time::SystemTime;

use cote::aopt::ARef;
use cote::aopt::RawVal;
use cote::prelude::ASer;
use cote::prelude::ASet;
use cote::prelude::Args;
use cote::prelude::ConfigBuildInfer;
use cote::prelude::Parser;
use cote::prelude::PolicyParser;
use cote::prelude::SetValueFindExt;
use cote::FwdPolicy;
use tokio::fs::create_dir;
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
                import snippet:plugin/types@0.1.1;

                export snippet:plugin/compiler@0.1.1;
                export snippet:plugin/language@0.1.1;
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
use types::Slient;

#[derive(Debug, Default)]
pub struct OptSet {
    pub(crate) parser: Parser<'static, ASet, ASer>,
}

impl OptSet {
    pub fn new() -> Result<Self, cote::Error> {
        let mut parser = Parser::<'_, ASet, ASer>::default();

        parser.add_opt("-help=b: display help message")?;
        parser.add_opt("-S=b: compile only; do not assemble or link")?;
        parser
            .add_opt("-E=b: preprocess or expand macro only; do not compile, assemble or link")?;
        parser.add_opt("-C=b: compile and assemble, but do not link")?;
        parser.add_opt("-e=s: add code to generate, can omit the ';'")?;
        parser.add_opt("-r=b: ignore value of -e, read code from stdin")?;
        parser.add_opt("-end=s: set input code terminator used by -r")?;
        if cfg!(windows) {
            parser
                .add_opt("-o: set output file name".infer::<String>())?
                .set_value("a.exe".to_string());
        } else {
            parser.add_opt("-o=s: set output file name")?;
        }
        parser.add_opt("-p=b: display code generated by plugin")?;
        parser.add_opt("-l=s: link given library")?;
        parser.add_opt("-L=s: add link library searh path")?;
        parser.add_opt("-f=s: pass given flag -<f> to compiler")?;
        parser.add_opt("-flag=s: pass given flag --<flag> to compiler")?;
        parser.add_opt("-std=s: set language standard version")?;
        parser.add_opt("-m=s: change the main function signature")?;
        parser.add_opt("-/o=b: show output of stdout of compiler")?;
        parser.add_opt("-/e=b: don't show output of stderr of compiler")?;
        parser.add_opt("-/c=b: don't clean target generated by compiler")?;
        parser.add_opt("-arg=s: pass arguments to executeable binary")?;
        parser.add_opt("-fmt=s: format the code if `-p` set")?;
        parser.add_opt("-cat=s: set command display the code")?;
        parser.add_opt(
            "files@1..: ignore -e and -r, compile multiple files"
                .infer::<cote::prelude::Pos<String>>(),
        )?;

        Ok(Self { parser })
    }

    pub fn parse(&mut self, lang: Lang, args: Vec<RawVal>) -> Result<bool, cote::Error> {
        let mut ret = PolicyParser::parse_policy(
            &mut self.parser,
            ARef::new(Args::from(args)),
            &mut FwdPolicy::default(),
        )?;
        let print_help = |parser: &mut Parser<'_, ASet, ASer>| {
            parser.display_help(
                env!("CARGO_PKG_AUTHORS"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_DESCRIPTION"),
            )
        };

        tracing::debug!("running compiler parse => {ret:?}");
        if *self.parser.find_val::<bool>("-help")? {
            self.parser.set_name(format!(
                "snippet {}",
                match lang {
                    Lang::C => "c",
                    Lang::Cxx => "cc",
                    Lang::Rust => "rs",
                }
            ));
            print_help(&mut self.parser)?;
            Ok(false)
        } else if let Some(error) = ret.take_failure() {
            Err(error)
        } else {
            Ok(true)
        }
    }
}

pub fn inner_optset<T: WasiView>(
    impl_: &mut WasiImpl<T>,
    self_: Resource<OptSet>,
) -> Result<&mut OptSet, ErrorType> {
    impl_
        .table()
        .get_mut(&self_)
        .map_err(|_| ErrorType::InvalidOptsetResource)
}

#[async_trait::async_trait]
impl<T: WasiView> types::HostOptset for WasiImpl<T> {
    async fn new(&mut self) -> Resource<OptSet> {
        self.table().push(OptSet::default()).unwrap()
    }

    async fn default(&mut self) -> Result<Resource<OptSet>, ErrorType> {
        let optset = OptSet::new().map_err(|_| ErrorType::OptsetInitFailed)?;

        self.table()
            .push(optset)
            .map_err(|_| ErrorType::InvalidOptsetResource)
    }

    async fn add_opt(&mut self, self_: Resource<OptSet>, opt: String) -> Result<u64, ErrorType> {
        let optset = inner_optset(self, self_)?;

        Ok(optset
            .parser
            .add_opt(opt)
            .and_then(|v| v.run())
            .map_err(|_| ErrorType::InvalidOptstr)?)
    }

    async fn get_val_str(
        &mut self,
        self_: Resource<OptSet>,
        name: String,
    ) -> Result<String, ErrorType> {
        let optset = inner_optset(self, self_)?;

        optset
            .parser
            .find_val::<String>(name)
            .map_err(|_| ErrorType::OptsetAccessValueFailed)
            .cloned()
    }

    async fn get_val_bool(
        &mut self,
        self_: Resource<OptSet>,
        name: String,
    ) -> Result<bool, ErrorType> {
        let optset = inner_optset(self, self_)?;

        optset
            .parser
            .find_val::<bool>(name)
            .map_err(|_| ErrorType::OptsetAccessValueFailed)
            .copied()
    }

    async fn get_val_int(
        &mut self,
        self_: Resource<OptSet>,
        name: String,
    ) -> Result<i64, ErrorType> {
        let optset = inner_optset(self, self_)?;

        optset
            .parser
            .find_val::<i64>(name)
            .map_err(|_| ErrorType::OptsetAccessValueFailed)
            .copied()
    }

    async fn get_vals_str(
        &mut self,
        self_: Resource<OptSet>,
        name: String,
    ) -> Result<Vec<String>, ErrorType> {
        let optset = inner_optset(self, self_)?;

        optset
            .parser
            .find_vals::<String>(name)
            .map_err(|_| ErrorType::OptsetAccessValueFailed)
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

    async fn find_bin(&mut self, bin: String) -> Result<String, ErrorType> {
        which::which(bin)
            .map_err(|_| ErrorType::InvalidBinary)
            .map(|v| v.to_string_lossy().to_string())
    }

    async fn invoke_cmd(
        &mut self,
        bin: String,
        args: Vec<String>,
        stdin_lines: Vec<String>,
        slient: Slient,
    ) -> Result<CmdResult, ErrorType> {
        let bin = Path::new(&bin);
        let mut cmd = Command::new(bin);

        cmd.args(args);
        cmd.stdin(Stdio::piped());
        if slient.stdout {
            cmd.stdout(Stdio::null());
        }
        if slient.stderr {
            cmd.stderr(Stdio::null());
        }

        tracing::debug!("running command: {cmd:?}");

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

    async fn read_from_stdin(&mut self, end: String) -> Result<Vec<String>, ErrorType> {
        println!("Please input your code, make sure your code correct.");
        println!("Enter `{}` end input.", end);
        let mut rl = rustyline::DefaultEditor::new().map_err(|_| ErrorType::InitRustylineFailed)?;
        let mut lines = vec![];

        loop {
            let line = rl
                .readline(":>> ")
                .map_err(|_| ErrorType::RustylineIoFailed)?;

            if line.trim() == end {
                break;
            } else {
                lines.push(line);
            }
        }
        Ok(lines)
    }

    async fn log_debug(&mut self, msg: String) -> Result<(), ErrorType> {
        tracing::debug!("got from wasip1: {msg}");
        Ok(())
    }

    async fn create_tmpdir(&mut self, prefix: String) -> Result<String, ErrorType> {
        let path = std::env::temp_dir();
        let d = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let tmpdir = format!("{prefix}-{}", d.as_millis());
        let tmpdir = path.join(tmpdir);

        create_dir(&tmpdir).await.map_err(|e| {
            tracing::debug!("create directory {} failed: {e:?}", tmpdir.display());
            ErrorType::CreateDirFailed
        })?;

        Ok(tmpdir.to_string_lossy().to_string())
    }

    async fn create_tmpfile(
        &mut self,
        dir: String,
        ext: String,
        content: Vec<u8>,
    ) -> Result<String, ErrorType> {
        let d = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let tmpdir = Path::new(&dir);
        let name = format!("{}.{ext}", d.as_millis());
        let tmpfile = tmpdir.join(name);

        tokio::fs::write(&tmpfile, content)
            .await
            .map_err(|_| ErrorType::CreateFileFailed)?;

        Ok(tmpfile.to_string_lossy().to_string())
    }

    fn drop(&mut self, rep: Resource<Services>) -> wasmtime::Result<()> {
        self.table().delete(rep)?;
        Ok(())
    }
}

impl<T: WasiView> types::Host for WasiImpl<T> {}

impl FromStr for Lang {
    type Err = ErrorType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" => Ok(Self::C),
            "cxx" | "cpp" | "cc" => Ok(Self::Cxx),
            "rust" | "rs" => Ok(Self::Rust),
            _ => Err(ErrorType::InvalidLanguage),
        }
    }
}
