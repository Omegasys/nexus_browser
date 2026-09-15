//! SpiderMonkey JavaScript engine integration.

pub mod adapter;
pub mod process_launcher;

pub use adapter::SpiderMonkeyAdapter;
pub use process_launcher::SpiderMonkeyProcessLauncher;
