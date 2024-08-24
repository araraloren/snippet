pub mod bindings {
    use wasmtime::component::*;

    bindgen!({
        world: "root",
        path: "wit",
        async: true,
        with: {
            "snippet:plugin/types/optset": super::OptSet,
            "snippet:plugin/types/services": super::Services,
        },
        inline: "
        package snippet:snippet;

        world root {
            import snippet:plugin/types@0.1.0;

            export snippet:c/compiler@0.1.0;
            export snippet:plugin/plugin@0.1.0;
        }
        "
    });
}

use bindings::snippet::plugin::types::ErrorType;
use bindings::snippet::plugin::types::Lang;
use bindings::snippet::plugin::types::Mode;
use cote::prelude::ASet;
use wasmtime::component::Resource;
use wasmtime_wasi::{WasiImpl, WasiView};

#[derive(Debug, Default)]
pub struct OptSet {
    optset: ASet,
}

#[async_trait::async_trait]
impl<T: WasiView> bindings::snippet::plugin::types::HostOptset for WasiImpl<T> {
    #[doc = " Add an option to the option set"]
    async fn add_opt(&mut self, self_: Resource<OptSet>, opt: String) -> Result<u64, ErrorType> {
        let optset = self
            .table()
            .get_mut(&self_)
            .map_err(|_| ErrorType::InvalidOptsetResource)?;

        Ok(optset
            .optset
            .add_opt(opt)
            .and_then(|v| v.run())
            .map_err(|_| ErrorType::InvalidCommand)?)
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

#[async_trait::async_trait]
impl<T: WasiView> bindings::snippet::plugin::types::HostServices for WasiImpl<T> {
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

    #[doc = " Is the compiler in debug mode?"]
    async fn debug(&mut self, self_: Resource<Services>) -> Result<bool, ErrorType> {
        let services = self
            .table()
            .get(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)?;

        Ok(services.debug)
    }

    #[doc = " Current language."]
    async fn lang(&mut self, self_: Resource<Services>) -> Result<Lang, ErrorType> {
        let services = self
            .table()
            .get(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)?;

        Ok(services.lang.clone())
    }

    #[doc = " Current arguments."]
    async fn args(&mut self, self_: Resource<Services>) -> Result<Vec<String>, ErrorType> {
        let services = self
            .table()
            .get(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)?;

        Ok(services.args.clone())
    }

    #[doc = " Current compile mode."]
    async fn mode(&mut self, self_: Resource<Services>) -> Result<Mode, ErrorType> {
        let services = self
            .table()
            .get(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)?;

        Ok(services.mode)
    }

    #[doc = " Set the language."]
    async fn set_lang(
        &mut self,
        self_: Resource<Services>,
        language: Lang,
    ) -> Result<(), ErrorType> {
        let services = self
            .table()
            .get_mut(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)?;

        services.lang = language;
        Ok(())
    }

    #[doc = " Set debug mode."]
    async fn set_debug(&mut self, self_: Resource<Services>, debug: bool) -> Result<(), ErrorType> {
        let services = self
            .table()
            .get_mut(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)?;

        services.debug = debug;
        Ok(())
    }

    #[doc = " Set the compile mode."]
    async fn set_mode(&mut self, self_: Resource<Services>, mode: Mode) -> Result<(), ErrorType> {
        let services = self
            .table()
            .get_mut(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)?;

        services.mode = mode;
        Ok(())
    }

    #[doc = " Add an argument."]
    async fn add_arg(&mut self, self_: Resource<Services>, arg: String) -> Result<(), ErrorType> {
        let services = self
            .table()
            .get_mut(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)?;

        services.args.push(arg);
        Ok(())
    }

    #[doc = " Append arguments."]
    async fn add_args(
        &mut self,
        self_: Resource<Services>,
        args: Vec<String>,
    ) -> Result<(), ErrorType> {
        let services = self
            .table()
            .get_mut(&self_)
            .map_err(|_| ErrorType::InvalidServicesResource)?;

        services.args.extend(args);
        Ok(())
    }

    #[doc = " Invoke the command"]
    async fn invoke_cmd(&mut self, bin: String, args: Vec<String>) -> Result<(), ErrorType> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<Services>) -> wasmtime::Result<()> {
        self.table().delete(rep)?;
        Ok(())
    }
}

impl<T: WasiView> bindings::snippet::plugin::types::Host for WasiImpl<T> {}

fn main() {}
