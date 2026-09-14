// Nexus Browser MicroVM System
// GPL-3.0 License

// Core VM management
pub mod microvm_manager;
pub mod vm_launcher;
pub mod vm_lifecycle;

// VM state management
pub mod vm_snapshot;
pub mod vm_restore;
pub mod vm_rollback;
pub mod vm_recovery;

// VM monitoring and security
pub mod vm_monitor;
pub mod vm_health;
pub mod vm_escape_detection;

// Specialized MicroVMs
pub mod renderer_vm;
pub mod network_vm;
pub mod dns_vm;
pub mod storage_vm;
pub mod gpu_vm;


// Re-export main interfaces

pub use microvm_manager::MicroVMManager;

pub use vm_launcher::VMLauncher;

pub use vm_lifecycle::{
    VMLifecycle,
    VMState,
};

pub use vm_snapshot::VMSnapshot;

pub use vm_restore::VMRestore;

pub use vm_rollback::VMRollback;

pub use vm_recovery::VMRecovery;

pub use vm_monitor::{
    VMMonitor,
    VMStatistics,
};

pub use vm_health::{
    VMHealth,
    HealthStatus,
};

pub use vm_escape_detection::VMEscapeDetection;

pub use renderer_vm::{
    RendererVM,
    RenderingEngine,
};

pub use network_vm::{
    NetworkVM,
    NetworkMode,
};

pub use dns_vm::{
    DNSVM,
    DNSMode,
};

pub use storage_vm::{
    StorageVM,
    StorageType,
};

pub use gpu_vm::{
    GPUVM,
    GPUMode,
};
