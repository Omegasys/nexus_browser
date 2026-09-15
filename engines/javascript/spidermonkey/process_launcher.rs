//! SpiderMonkey process launcher.

use std::process::{
    Child,
    Command,
    Stdio,
};


use crate::browser_core::javascript::js_engine_api::{
    JsEngineError,
    JsEngineResult,
};



pub struct SpiderMonkeyProcessLauncher {

    executable: String,

    child: Option<Child>,

}



impl SpiderMonkeyProcessLauncher {


    pub fn new(
        executable: impl Into<String>
    ) -> JsEngineResult<Self> {


        Ok(Self {

            executable:
                executable.into(),

            child:
                None,

        })

    }



    pub fn launch(
        &mut self
    ) -> JsEngineResult<()> {


        let child =
            Command::new(
                &self.executable
            )

            .stdin(
                Stdio::piped()
            )

            .stdout(
                Stdio::piped()
            )

            .stderr(
                Stdio::piped()
            )

            .spawn()

            .map_err(
                |e|
                JsEngineError::LaunchFailed(
                    e.to_string()
                )
            )?;


        self.child =
            Some(child);


        Ok(())

    }



    pub fn terminate(
        &mut self
    ) -> JsEngineResult<()> {


        if let Some(
            mut child
        ) = self.child.take() {


            child.kill()
                .map_err(
                    |e|
                    JsEngineError::ShutdownFailed(
                        e.to_string()
                    )
                )?;

        }


        Ok(())

    }

}
