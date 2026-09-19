pub mod privacy_score;
pub mod fingerprint_entropy;
pub mod isolation_score;
pub mod rendering_benchmark;
pub mod js_engine_benchmark;
pub mod network_anonymity_score;

pub use privacy_score::{
    PrivacyScore,
    PrivacyScoreCategory,
    PrivacyScoreResult,
};

pub use fingerprint_entropy::{
    FingerprintEntropy,
    FingerprintSignal,
    FingerprintEntropyResult,
};

pub use isolation_score::{
    IsolationScore,
    IsolationComponent,
    IsolationScoreResult,
};

pub use rendering_benchmark::{
    RenderingBenchmark,
    RenderingBenchmarkResult,
    RenderingMetric,
};

pub use js_engine_benchmark::{
    JsEngineBenchmark,
    JsBenchmarkResult,
    JsBenchmarkMetric,
};

pub use network_anonymity_score::{
    NetworkAnonymityScore,
    NetworkAnonymityResult,
    AnonymityComponent,
};
