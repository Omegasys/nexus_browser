//! JavaScriptCore launcher.

use crate::browser_core::javascript::js_engine_api::{
    JsEngineError,
    JsEngineResult,
};



pub struct JavaScriptCoreProcessLauncher {

    executable: String,

}



impl JavaScriptCoreProcessLauncher {


    pub fn new(
        executable: impl Into<String>
    ) -> JsEngineResult<Self> {


        Ok(Self {

            executable:
                executable.into()

        })

    }



    pub fn executable(
        &self
    ) -> &str {

        &self.executable

    }



    pub fn terminate(
        &mut self
    ) -> JsEngineResult<()> {

        Ok(())

    }

}
