wit_bindgen::generate!({
    path: "wit",
    world: "c",
    with: {
        "snippet:plugin/types@0.1.1": generate,
        "snippet:plugin/compiler@0.1.1": generate,
    }
});

use exports::snippet::plugin::compiler::ErrorType;
use exports::snippet::plugin::compiler::Guest;
use exports::snippet::plugin::compiler::GuestCompiler;
use exports::snippet::plugin::compiler::Lang;
use exports::snippet::plugin::compiler::Mode;
use snippet::plugin::types::Binary;
use snippet::plugin::types::Object;
use snippet::plugin::types::Output;
use snippet::plugin::types::Services;
use snippet::plugin::types::Slient;
use snippet::plugin::types::Source;
use snippet::plugin::types::Target;

pub struct Compiler {
    storage: Services,
}

impl Guest for Compiler {
    type Compiler = Compiler;

    fn name() -> String {
        String::from("gcc")
    }

    fn bin() -> Result<String, ErrorType> {
        Services::find_bin("gcc")
    }

    fn support() -> Lang {
        Lang::C
    }
}

impl GuestCompiler for Compiler {
    fn new() -> Self {
        Self {
            storage: Services::new(),
        }
    }

    fn args(&self) -> Result<Vec<String>, ErrorType> {
        self.storage.args()
    }

    fn mode(&self) -> Result<Mode, ErrorType> {
        self.storage.mode()
    }

    fn enable_debug(&self) -> Result<(), ErrorType> {
        self.add_raw_arg("-g".to_string())
    }

    fn enable_warn_error(&self) -> Result<(), ErrorType> {
        self.add_raw_arg("-Wall".to_string())?;
        self.add_raw_arg("-Wextra".to_string())?;
        self.add_raw_arg("-Werror".to_string())
    }

    fn set_mode(&self, mode: Mode) -> Result<(), ErrorType> {
        self.storage.set_mode(mode)
    }

    fn set_opt_level(&self, level: u8) -> Result<(), ErrorType> {
        if matches!(level, 0..4) {
            self.add_raw_arg(format!("-O{}", level))
        } else {
            Err(ErrorType::InvalidOptLevel)
        }
    }

    fn set_standard(&self, std: String) -> Result<(), ErrorType> {
        if matches!(
            std.as_str(),
            "c89" | "c99" | "c11" | "c17" | "c23" | "gnu89" | "gnu99" | "gnu11" | "gnu17" | "gnu23"
        ) {
            self.add_raw_arg(format!("-std={}", std))
        } else {
            Err(ErrorType::InvalidStandard)
        }
    }

    fn add_macro(&self, macro_: String, value: Option<String>) -> Result<(), ErrorType> {
        if let Some(value) = value {
            self.add_raw_args(vec!["-D".to_string(), format!("{}={}", macro_, value)])
        } else {
            self.add_raw_arg(format!("-D{}", macro_))
        }
    }

    fn add_include_path(&self, path: String) -> Result<(), ErrorType> {
        self.add_raw_arg(format!("-I{}", path))
    }

    fn add_library_path(&self, path: String) -> Result<(), ErrorType> {
        self.add_raw_arg(format!("-L{}", path))
    }

    fn add_link_library(&self, library: String) -> Result<(), ErrorType> {
        self.add_raw_arg(format!("-l{}", library))
    }

    fn add_arg(&self, arg: String, long: bool) -> Result<(), ErrorType> {
        if long {
            self.storage.add_arg(&format!("--{}", arg))
        } else {
            self.storage.add_arg(&format!("-{}", arg))
        }
    }

    fn add_raw_arg(&self, arg: String) -> Result<(), ErrorType> {
        self.storage.add_arg(&arg)
    }

    fn add_raw_args(&self, args: Vec<String>) -> Result<(), ErrorType> {
        for arg in args {
            self.storage.add_arg(&arg)?;
        }
        Ok(())
    }

    fn compile_code(
        &self,
        source: Vec<String>,
        out: String,
        slient: Slient,
    ) -> Result<Target, ErrorType> {
        let bin = Self::bin()?;
        let mut args = self.storage.args()?;
        let mode = self.storage.mode()?;

        if let Some(mode) = to_compiler_mode(mode) {
            args.push(mode.to_string());
        }
        args.extend(["-o", &out, "-xc", "-"].map(String::from));

        let result = Services::invoke_cmd(&bin, &args, &source, slient)?;

        Ok(Target {
            clean: false,
            output: match mode {
                Mode::Compile => Output::Object(Object { path: out }),
                Mode::Link => Output::Binary(Binary {
                    path: out,
                    args: vec![],
                }),
                _ => Output::Source(Source { path: out }),
            },
            codes: source,
            cmd_result: result,
        })
    }

    fn compile_file(&self, path: String, out: String, slient: Slient) -> Result<Target, ErrorType> {
        let bin = Self::bin()?;
        let mut args = self.storage.args()?;
        let mode = self.storage.mode()?;

        if let Some(mode) = to_compiler_mode(mode) {
            args.push(mode.to_string());
        }
        args.extend(["-o".to_string(), out.clone(), path]);

        let result = Services::invoke_cmd(&bin, &args, &[], slient)?;

        Ok(Target {
            clean: false,
            output: match mode {
                Mode::Compile => Output::Object(Object { path: out }),
                Mode::Link => Output::Binary(Binary {
                    path: out,
                    args: vec![],
                }),
                _ => Output::Source(Source { path: out }),
            },
            codes: vec![],
            cmd_result: result,
        })
    }

    fn link_object(
        &self,
        objs: Vec<String>,
        out: String,
        slient: Slient,
    ) -> Result<Target, ErrorType> {
        let bin = Self::bin()?;
        let mut args = self.storage.args()?;
        let mode = self.storage.mode()?;

        if !matches!(mode, Mode::Link) {
            return Err(ErrorType::InvalidModeForLink);
        }

        args.extend(objs);
        args.extend(["-o".to_string(), out.clone()]);

        let result = Services::invoke_cmd(&bin, &args, &[], slient)?;

        Ok(Target {
            clean: false,
            output: Output::Binary(Binary {
                path: out,
                args: vec![],
            }),
            codes: vec![],
            cmd_result: result,
        })
    }
}

export!(Compiler);

fn to_compiler_mode(mode: Mode) -> Option<&'static str> {
    match mode {
        snippet::plugin::types::Mode::Compile => Some("-c"),
        snippet::plugin::types::Mode::Expand => Some("-E"),
        snippet::plugin::types::Mode::Assemble => Some("-S"),
        snippet::plugin::types::Mode::Link => None,
    }
}
