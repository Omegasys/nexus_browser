//! Memory protection subsystem.
//!
//! Provides memory boundaries for browser processes,
//! rendering engines, JavaScript engines, and MicroVMs.

#[derive(Debug, Clone)]
pub struct MemoryPolicy {


    pub max_process_memory_mb: u64,

    pub max_engine_memory_mb: u64,

    pub prevent_executable_memory: bool,

    pub guard_pages: bool,

    pub zero_memory_on_release: bool,

}



impl Default for MemoryPolicy {


    fn default() -> Self {


        Self {

            max_process_memory_mb: 4096,

            max_engine_memory_mb: 1024,

            prevent_executable_memory: true,

            guard_pages: true,

            zero_memory_on_release: true,

        }

    }

}



pub struct MemoryProtector {


    policy: MemoryPolicy,

    protected_regions: Vec<String>,

}



impl MemoryProtector {


    pub fn new() -> Self {


        Self {

            policy:
                MemoryPolicy::default(),

            protected_regions:
                Vec::new(),

        }

    }



    pub fn protect_region(
        &mut self,
        name: String
    ) {


        self.protected_regions
            .push(name);

    }



    pub fn remove_region(
        &mut self,
        name: &str
    ) {


        self.protected_regions
            .retain(
                |x| x != name
            );

    }



    pub fn is_protected(
        &self,
        name: &str
    ) -> bool {


        self.protected_regions
            .contains(
                &name.to_string()
            )

    }



    pub fn policy(
        &self
    ) -> &MemoryPolicy {

        &self.policy

    }


}


impl Default for MemoryProtector {


    fn default() -> Self {

        Self::new()

    }

}
