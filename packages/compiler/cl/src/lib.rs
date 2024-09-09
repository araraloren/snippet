wit_bindgen::generate!({
    path: "wit",
    world: "cl",
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
    compiler_storage: Services,
    linker_storage: Services,
}

impl Guest for Compiler {
    type Compiler = Compiler;

    fn name() -> String {
        String::from("cl")
    }

    fn bin() -> Result<String, ErrorType> {
        Services::find_bin("cl")
    }

    fn support() -> Lang {
        Lang::C
    }
}

impl GuestCompiler for Compiler {
    fn new() -> Self {
        Self {
            compiler_storage: Services::new(),
            linker_storage: Services::new(),
        }
    }

    fn args(&self) -> Result<Vec<String>, ErrorType> {
        self.compiler_storage.args()
    }

    fn mode(&self) -> Result<Mode, ErrorType> {
        self.compiler_storage.mode()
    }

    fn enable_debug(&self) -> Result<(), ErrorType> {
        self.add_raw_arg("/ZI".to_string())
    }

    fn enable_warn_error(&self) -> Result<(), ErrorType> {
        self.add_raw_arg("/Wall".to_string())?;
        self.add_raw_arg("/WX".to_string())
    }

    fn set_mode(&self, mode: Mode) -> Result<(), ErrorType> {
        self.compiler_storage.set_mode(mode)
    }

    fn set_opt_level(&self, level: u8) -> Result<(), ErrorType> {
        if matches!(level, 0..2) {
            self.add_raw_arg(format!("/O{}", level))
        } else {
            Err(ErrorType::InvalidOptLevel)
        }
    }

    fn set_standard(&self, std: String) -> Result<(), ErrorType> {
        match std.as_str() {
            "c89" => self.add_raw_arg("/Tc".to_string()),
            "c11" | "c17" => self.add_raw_arg(format!("/std:{}", std)),
            _ => Err(ErrorType::InvalidStandard),
        }
    }

    fn add_macro(&self, macro_: String, value: Option<String>) -> Result<(), ErrorType> {
        if let Some(value) = value {
            self.add_raw_args(vec!["/D".to_string(), format!("{}={}", macro_, value)])
        } else {
            self.add_raw_args(vec!["/D".to_string(), macro_.to_string()])
        }
    }

    fn add_include_path(&self, path: String) -> Result<(), ErrorType> {
        self.add_raw_arg(format!("/I{}", path))
    }

    fn add_library_path(&self, path: String) -> Result<(), ErrorType> {
        self.linker_storage.add_arg(&format!("/LIBPATH:{}", path))
    }

    fn add_link_library(&self, library: String) -> Result<(), ErrorType> {
        self.linker_storage.add_arg(&library)
    }

    fn add_arg(&self, arg: String, _long: bool) -> Result<(), ErrorType> {
        self.compiler_storage.add_arg(&format!("/{}", arg))
    }

    fn add_raw_arg(&self, arg: String) -> Result<(), ErrorType> {
        self.compiler_storage.add_arg(&arg)
    }

    fn add_raw_args(&self, args: Vec<String>) -> Result<(), ErrorType> {
        for arg in args {
            self.compiler_storage.add_arg(&arg)?;
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
        let mut compiler_args = self.compiler_storage.args()?;
        let linker_args = self.linker_storage.args()?;
        let mode = self.compiler_storage.mode()?;

        let object_ext = match mode {
            snippet::plugin::types::Mode::Compile => ".obj",
            snippet::plugin::types::Mode::Expand => ".i",
            snippet::plugin::types::Mode::Assemble => ".asm",
            snippet::plugin::types::Mode::Link => ".obj",
        };
        let content = source.join("\n");
        let tmpdir = Services::create_tmpdir("snippet-compiler-cl")?;
        let srcfile = Services::create_tmpfile(&tmpdir, "c", content.as_bytes())?;

        Services::log_debug(&format!("generate a source file {}", &srcfile))?;
        let srcfilename = srcfile.strip_prefix(&tmpdir).unwrap();
        let srcfilename = srcfilename.replace("\\", "");
        let objectname = format!("{}\\{}", tmpdir, srcfilename.replace(".c", ".obj"));
        let exepath = format!("{}\\{}", tmpdir, &out);

        if let snippet::plugin::types::Mode::Assemble = mode {
            compiler_args.push("/FA".to_string());
            compiler_args.push("/c".to_string())
        } else if let snippet::plugin::types::Mode::Expand = mode {
            compiler_args.push("/P".to_string());
            compiler_args.push("/c".to_string())
        } else if let snippet::plugin::types::Mode::Compile = mode {
            compiler_args.push("/c".to_string())
        }
        compiler_args.push(format!("/Fo:{}", objectname));
        compiler_args.push(srcfile);
        if let snippet::plugin::types::Mode::Link = mode {
            compiler_args.push("/link".to_string());
            compiler_args.extend(linker_args);
            compiler_args.push(format!("/out:{}", exepath));
        }

        let result = Services::invoke_cmd(&bin, &compiler_args, &[], slient)?;

        Ok(Target {
            clean: false,
            output: match mode {
                Mode::Compile => Output::Object(Object { path: objectname }),
                Mode::Link => Output::Binary(Binary {
                    path: exepath,
                    args: vec![],
                }),
                _ => Output::Source(Source {
                    path: srcfilename.replace(".c", object_ext),
                }),
            },
            codes: source,
            cmd_result: result,
        })
    }

    fn compile_file(&self, path: String, out: String, slient: Slient) -> Result<Target, ErrorType> {
        let bin = Self::bin()?;
        let mut args = self.compiler_storage.args()?;
        let mode = self.compiler_storage.mode()?;

        args.push(format!("/Fo:{}", out));
        args.push("/c".to_string());
        args.push(path);

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
        let linker = "link.exe";
        let mut args = self.linker_storage.args()?;

        Services::log_debug(&format!("try to link objects {:?}", objs))?;

        args.extend(objs);
        args.push(format!("/out:{}", out.clone()));

        let result = Services::invoke_cmd(linker, &args, &[], slient)?;

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
