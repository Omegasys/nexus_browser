#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub filesystem: bool,
    pub network: bool,
    pub clipboard: bool,
    pub native_processes: bool,
    pub isolated_storage: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            filesystem: false,
            network: false,
            clipboard: false,
            native_processes: false,
            isolated_storage: true,
        }
    }
}

pub struct Sandbox;

impl Sandbox {
    pub fn create(
        extension_id: &str,
        config: &SandboxConfig,
    ) {
        println!(
            "Creating sandbox for {}",
            extension_id
        );

        println!(
            "Filesystem: {}",
            config.filesystem
        );

        println!(
            "Network: {}",
            config.network
        );
    }

    pub fn destroy(
        extension_id: &str,
    ) {
        println!(
            "Destroying sandbox: {}",
            extension_id
        );
    }
}
