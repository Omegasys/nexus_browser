// Nexus Browser Blink Process Launcher
// GPL-3.0 License

use std::path::PathBuf;
use std::process::{Child, Command};


pub struct BlinkProcessLauncher {

    executable: PathBuf,

}


impl BlinkProcessLauncher {


    pub fn new(

        executable: PathBuf

    ) -> Self {

        Self {

            executable,

        }

    }


    pub fn launch(

        &self

    ) -> Result<Child, String> {

        println!(
            "Launching Blink process: {:?}",
            self.executable
        );


        Command::new(
            &self.executable
        )
        .spawn()
        .map_err(
            |error|
            format!(
                "Failed to launch Blink: {}",
                error
            )
        )

    }


    pub fn launch_sandboxed(

        &self,

        sandbox_profile: &str

    ) -> Result<Child, String> {

        println!(
            "Launching Blink with sandbox profile {}",
            sandbox_profile
        );


        Command::new(
            &self.executable
        )
        .arg(
            format!(
                "--nexus-sandbox={}",
                sandbox_profile
            )
        )
        .spawn()
        .map_err(
            |error|
            format!(
                "Failed to launch sandboxed Blink: {}",
                error
            )
        )

    }


    pub fn executable(

        &self

    ) -> &PathBuf {

        &self.executable

    }

}
