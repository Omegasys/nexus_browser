//! Nexus Browser JavaScript subsystem.
//!
//! Provides the JavaScript engine abstraction, engine registry,
//! runtime bridge, loader, and concrete JavaScript engine adapters.

pub mod js_engine_api;
pub mod loader;
pub mod registry;
pub mod runtime_bridge;

pub mod v8;

pub use js_engine_api::{
    JsEngine,
    JsEngineCapabilities,
    JsEngineConfiguration,
    JsEngineError,
    JsEngineResult,
    JsEngineState,
};

pub use loader::JsEngineLoader;
pub use registry::JsEngineRegistry;
pub use runtime_bridge::{
    JsRuntimeBridge,
    RuntimeMessage,
    RuntimeMessageType,
};

pub use v8::V8Adapter;
