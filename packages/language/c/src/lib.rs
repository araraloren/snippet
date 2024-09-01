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

    fn fill_optset(optset: Optset) -> Result<(), ErrorType> {
        optset.add_opt("c=cmd: execute c code")?;
        optset.add_opt("-i=s: pass -i to compiler, include given header")?;
        optset.add_opt("-I=s: pass -I to compiler, add include header search path")?;
        optset.add_opt("-D=s: pass -D to compiler, add macro definition")?;
        Ok(())
    }

    fn compile(_optset: Optset, compiler: Compiler) -> Result<CmdResult, ErrorType> {
        let codes = [
            "#include <stdio.h>",
            "int main()",
            "{",
            "printf(\"hello from wasm32-wasip1\");",
            "}",
        ]
        .map(String::from)
        .to_vec();

        compiler.compile_code(&codes, "a.out")
    }
}

export!(Language);
