wit_bindgen::generate!({
    path: "wit",
    world: "c",
    with: {
        "snippet:plugin/types@0.1.0": generate,
        "snippet:plugin/compiler@0.1.0": generate,
    }
});

use exports::snippet::plugin::compiler::ErrorType;
use exports::snippet::plugin::compiler::Guest;
use exports::snippet::plugin::compiler::GuestCompiler;
use exports::snippet::plugin::compiler::Lang;
use exports::snippet::plugin::compiler::Mode;
use snippet::plugin::types::CmdResult;
use snippet::plugin::types::Services;

pub struct Compiler {
    storage: Services,
}

impl Compiler {
    pub fn internal_compile(
        &self,
        args: Vec<String>,
        input: Vec<String>,
    ) -> Result<CmdResult, ErrorType> {
        let bin = Self::bin()?;

        Services::invoke_cmd(&bin, &args, &input)
    }
}

impl Guest for Compiler {
    type Compiler = Compiler;

    fn bin() -> Result<Vec<u8>, ErrorType> {
        Services::find_bin("gcc")
    }

    fn support() -> Lang {
        snippet::plugin::types::Lang::C
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

    fn debug(&self) -> Result<bool, ErrorType> {
        self.storage.debug()
    }

    fn mode(&self) -> Result<Mode, ErrorType> {
        self.storage.mode()
    }

    fn set_debug(&self, debug: bool) -> Result<(), ErrorType> {
        self.storage.set_debug(debug)
    }

    fn set_mode(&self, mode: Mode) -> Result<(), ErrorType> {
        self.storage.set_mode(mode)
    }

    fn set_opt_level(&self, level: u8) -> Result<(), ErrorType> {
        if matches!(level, 0..4) {
            self.add_arg(format!("-O{}", level))
        } else {
            Err(ErrorType::InvalidOptLevel)
        }
    }

    fn set_standard(&self, std: String) -> Result<(), ErrorType> {
        if matches!(
            std.as_str(),
            "c89" | "c99" | "c11" | "c17" | "c23" | "gnu89" | "gnu99" | "gnu11" | "gnu17" | "gnu23"
        ) {
            self.add_arg(format!("-std={}", std))
        } else {
            Err(ErrorType::InvalidStandard)
        }
    }

    fn add_macro(&self, macro_: String, value: Option<String>) -> Result<(), ErrorType> {
        if let Some(value) = value {
            self.add_arg(format!("-D{}={}", macro_, value))
        } else {
            self.add_arg(format!("-D{}", macro_))
        }
    }

    fn add_include_path(&self, path: String) -> Result<(), ErrorType> {
        self.add_arg(format!("-I{}", path))
    }

    fn add_library_path(&self, path: String) -> Result<(), ErrorType> {
        self.add_arg(format!("-L{}", path))
    }

    fn link_library(&self, library: String) -> Result<(), ErrorType> {
        self.add_arg(format!("-l{}", library))
    }

    fn add_arg(&self, arg: String) -> Result<(), ErrorType> {
        self.storage.add_arg(&arg)
    }

    fn add_args(&self, args: Vec<String>) -> Result<(), ErrorType> {
        for arg in args {
            self.add_arg(arg)?;
        }
        Ok(())
    }

    fn compile_code(&self, source: Vec<String>, out: String) -> Result<String, ErrorType> {
        todo!()
    }

    fn compile_file(&self, path: String, out: String) -> Result<String, ErrorType> {
        todo!()
    }

    fn link_object(&self, objs: Vec<String>, out: String) -> Result<String, ErrorType> {
        todo!()
    }
}
