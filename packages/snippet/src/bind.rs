pub mod bindings {
    use wasmtime::component::*;

    bindgen!({
        world: "c",
        path: "wit",
        async: true,
        with: {
            "snippet:plugin/types/optset": super::OptSet,
        }
    });
}

use bindings::snippet::plugin::types::ErrorType;
use cote::prelude::ASet;
use wasmtime::component::Resource;
use wasmtime_wasi::{WasiImpl, WasiView};

#[derive(Debug, Default)]
pub struct OptSet {
    optset: ASet,
}

impl OptSet {
    pub fn add_opt<T: Into<String>>(&mut self, opt: T) -> Result<u64, ErrorType> {
        todo!()
    }
}

#[async_trait::async_trait]
impl<T: WasiView> bindings::snippet::plugin::types::HostOptset for WasiImpl<T> {
    #[doc = " Add an option to the option set"]
    async fn add_opt(&mut self, self_: Resource<OptSet>, opt: String) -> Result<u64, ErrorType> {
        let optset = self
            .table()
            .get_mut(&self_)
            .map_err(|_| ErrorType::InvalidResource)?;

        optset.add_opt(opt)
    }

    fn drop(&mut self, rep: Resource<OptSet>) -> wasmtime::Result<()> {
        self.table().delete(rep)?;
        Ok(())
    }
}
