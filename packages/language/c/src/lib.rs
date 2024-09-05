wit_bindgen::generate!({
    path: "wit",
    world: "c",
    with: {
        "snippet:plugin/types@0.1.0": generate,
        "snippet:plugin/language@0.1.0": generate,
        "snippet:plugin/compiler@0.1.0": generate,
    }
});

use exports::snippet::plugin::language::ErrorType;
use exports::snippet::plugin::language::Guest;
use snippet::plugin::compiler::Compiler;
use snippet::plugin::types::CmdResult;
use snippet::plugin::types::Lang;
use snippet::plugin::types::Optset;

pub struct Language;

impl Guest for Language {
    fn name() -> Lang {
        Lang::C
    }

    fn initialize_optset() -> Result<Optset, ErrorType> {
        Optset::default()
    }

    fn fill_optset(optset: Optset) -> Result<Optset, ErrorType> {
        optset.add_opt("c=cmd: execute c code")?;
        optset.add_opt("-i=s: pass -i to compiler, include given header")?;
        optset.add_opt("-I=s: pass -I to compiler, add include header search path")?;
        optset.add_opt("-D=s: pass -D to compiler, add macro definition")?;
        Ok(optset)
    }

    fn compile(optset: Optset, compiler: Compiler) -> Result<CmdResult, ErrorType> {
        let code = optset.get_value_str("-e")?;
        tracing::debug!("got code -> {code}");
        let codes = [
            "#include <stdio.h>",
            "int main()",
            "{",
            &format!("{};", code),
            "}",
        ]
        .map(String::from)
        .to_vec();

        compiler.compile_code(&codes, "a.exe")
    }
}

export!(Language);
