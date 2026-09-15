//! JavaScriptCore adapter.

use super::process_launcher::JavaScriptCoreProcessLauncher;


use crate::browser_core::javascript::js_engine_api::{
    JsEngine,
    JsEngineCapabilities,
    JsEngineConfiguration,
    JsEngineError,
    JsEngineResult,
    JsEngineState,
};



pub struct JavaScriptCoreAdapter {

    configuration: JsEngineConfiguration,

    capabilities: JsEngineCapabilities,

    state: JsEngineState,

    launcher: Option<JavaScriptCoreProcessLauncher>,

}



impl JavaScriptCoreAdapter {


    pub fn new() -> Self {


        Self::with_configuration(

            JsEngineConfiguration {

                name:
                    "javascriptcore".to_string(),

                version:
                    "WebKit".to_string(),

                ..Default::default()

            }

        )

    }



    pub fn with_configuration(
        configuration: JsEngineConfiguration
    ) -> Self {


        Self {

            configuration,

            capabilities:
                JsEngineCapabilities::default(),

            state:
                JsEngineState::Unloaded,

            launcher:
                None,

        }

    }

}



impl Default for JavaScriptCoreAdapter {

    fn default() -> Self {

        Self::new()

    }

}



impl JsEngine for JavaScriptCoreAdapter {


    fn name(&self) -> &str {

        &self.configuration.name

    }



    fn version(&self) -> &str {

        &self.configuration.version

    }



    fn state(&self) -> JsEngineState {

        self.state

    }



    fn capabilities(&self)
        -> &JsEngineCapabilities {

        &self.capabilities

    }



    fn configuration(&self)
        -> &JsEngineConfiguration {

        &self.configuration

    }



    fn load(
        &mut self
    ) -> JsEngineResult<()> {


        self.state =
            JsEngineState::Loaded;


        Ok(())

    }



    fn start(
        &mut self
    ) -> JsEngineResult<()> {


        self.state =
            JsEngineState::Running;


        Ok(())

    }



    fn stop(
        &mut self
    ) -> JsEngineResult<()> {


        self.state =
            JsEngineState::Loaded;


        Ok(())

    }



    fn unload(
        &mut self
    ) -> JsEngineResult<()> {


        self.state =
            JsEngineState::Unloaded;


        Ok(())

    }



    fn execute(
        &mut self,
        source: &str
    ) -> JsEngineResult<String> {


        if self.state != JsEngineState::Running {

            return Err(
                JsEngineError::NotRunning
            );

        }


        Ok(
            format!(
                "JavaScriptCore execution request: {} bytes",
                source.len()
            )
        )

    }



    fn is_healthy(
        &self
    ) -> bool {


        self.state ==
            JsEngineState::Running

    }

}
