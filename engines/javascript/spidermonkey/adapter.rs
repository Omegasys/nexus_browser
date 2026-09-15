//! SpiderMonkey JavaScript engine adapter.

use super::process_launcher::SpiderMonkeyProcessLauncher;

use crate::browser_core::javascript::js_engine_api::{
    JsEngine,
    JsEngineCapabilities,
    JsEngineConfiguration,
    JsEngineError,
    JsEngineResult,
    JsEngineState,
};


pub struct SpiderMonkeyAdapter {
    configuration: JsEngineConfiguration,
    capabilities: JsEngineCapabilities,
    state: JsEngineState,
    launcher: Option<SpiderMonkeyProcessLauncher>,
}


impl SpiderMonkeyAdapter {

    pub fn new() -> Self {

        Self::with_configuration(
            JsEngineConfiguration {
                name: "spidermonkey".to_string(),
                version: "128.0".to_string(),
                ..Default::default()
            }
        )
    }


    pub fn with_configuration(
        configuration: JsEngineConfiguration
    ) -> Self {

        Self {
            configuration,
            capabilities: JsEngineCapabilities {
                ecmascript: true,
                wasm: true,
                modules: true,
                workers: true,
                shared_array_buffer: true,
                bigint: true,
                jit: true,
                sandboxing: true,
                microvm: true,
                hot_reload: true,
            },

            state: JsEngineState::Unloaded,

            launcher: None,
        }
    }


    fn create_launcher(
        &self
    ) -> JsEngineResult<SpiderMonkeyProcessLauncher> {


        let executable =
            self.configuration
                .executable
                .as_deref()
                .ok_or_else(|| {

                    JsEngineError::ConfigurationError(
                        "SpiderMonkey executable not configured"
                        .to_string()
                    )

                })?;


        SpiderMonkeyProcessLauncher::new(
            executable
        )
    }
}



impl Default for SpiderMonkeyAdapter {

    fn default() -> Self {

        Self::new()

    }
}



impl JsEngine for SpiderMonkeyAdapter {


    fn name(&self) -> &str {

        &self.configuration.name

    }


    fn version(&self) -> &str {

        &self.configuration.version

    }


    fn state(&self) -> JsEngineState {

        self.state

    }


    fn capabilities(
        &self
    ) -> &JsEngineCapabilities {

        &self.capabilities

    }


    fn configuration(
        &self
    ) -> &JsEngineConfiguration {

        &self.configuration

    }



    fn load(
        &mut self
    ) -> JsEngineResult<()> {


        if self.state != JsEngineState::Unloaded {

            return Err(
                JsEngineError::AlreadyLoaded
            );

        }


        self.state =
            JsEngineState::Loaded;


        Ok(())

    }



    fn start(
        &mut self
    ) -> JsEngineResult<()> {


        if self.state != JsEngineState::Loaded {

            return Err(
                JsEngineError::InvalidState
            );

        }


        self.state =
            JsEngineState::Starting;



        if self.configuration.executable.is_some() {

            self.launcher =
                Some(
                    self.create_launcher()?
                );

        }



        self.state =
            JsEngineState::Running;


        Ok(())

    }



    fn stop(
        &mut self
    ) -> JsEngineResult<()> {


        if let Some(
            launcher
        ) = self.launcher.as_mut() {

            launcher.terminate()?;

        }


        self.state =
            JsEngineState::Loaded;


        Ok(())

    }



    fn unload(
        &mut self
    ) -> JsEngineResult<()> {


        self.launcher = None;


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
                "SpiderMonkey execution request: {} bytes",
                source.len()
            )
        )

    }



    fn is_healthy(
        &self
    ) -> bool {


        matches!(
            self.state,

            JsEngineState::Loaded |
            JsEngineState::Running

        )

    }

}
