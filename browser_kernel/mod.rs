// Nexus Browser Kernel
// GPL-3.0 License

pub mod kernel_core;
pub mod kernel_orchestrator;

pub mod engine_abstraction_layer;
pub mod engine_registry;
pub mod engine_switcher;

pub use kernel_core::KernelCore;
pub use kernel_orchestrator::KernelOrchestrator;

pub use engine_abstraction_layer::{
    Engine,
    EngineType,
    EngineMetadata,
};

pub use engine_registry::EngineRegistry;
pub use engine_switcher::EngineSwitcher;
