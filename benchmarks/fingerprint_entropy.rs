use std::collections::HashMap;

/// A browser fingerprint signal.
#[derive(Debug, Clone)]
pub struct FingerprintSignal {
    pub name: String,
    pub value: String,
    pub uniqueness: f64,
    pub entropy_bits: f64,
    pub randomized: bool,
    pub normalized: bool,
}

impl FingerprintSignal {
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
        uniqueness: f64,
        entropy_bits: f64,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            uniqueness: uniqueness.clamp(0.0, 1.0),
            entropy_bits: entropy_bits.max(0.0),
            randomized: false,
            normalized: false,
        }
    }
}

/// Calculated fingerprint entropy.
#[derive(Debug, Clone)]
pub struct FingerprintEntropyResult {
    pub raw_entropy_bits: f64,
    pub effective_entropy_bits: f64,
    pub normalized_entropy: f64,
    pub uniqueness_score: f64,
    pub signal_count: usize,
}

impl FingerprintEntropyResult {
    pub fn new() -> Self {
        Self {
            raw_entropy_bits: 0.0,
            effective_entropy_bits: 0.0,
            normalized_entropy: 0.0,
            uniqueness_score: 0.0,
            signal_count: 0,
        }
    }
}

impl Default for FingerprintEntropyResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Fingerprint entropy benchmark.
///
/// Lower effective entropy generally means less distinguishing information.
#[derive(Debug, Clone)]
pub struct FingerprintEntropy {
    signals: HashMap<String, FingerprintSignal>,
    entropy_reference_bits: f64,
}

impl FingerprintEntropy {
    pub fn new() -> Self {
        Self {
            signals: HashMap::new(),
            entropy_reference_bits: 32.0,
        }
    }

    pub fn with_reference_entropy(bits: f64) -> Self {
        Self {
            signals: HashMap::new(),
            entropy_reference_bits: bits.max(1.0),
        }
    }

    pub fn add_signal(&mut self, signal: FingerprintSignal) {
        self.signals.insert(signal.name.clone(), signal);
    }

    pub fn mark_randomized(&mut self, name: &str, randomized: bool) {
        if let Some(signal) = self.signals.get_mut(name) {
            signal.randomized = randomized;
        }
    }

    pub fn mark_normalized(&mut self, name: &str, normalized: bool) {
        if let Some(signal) = self.signals.get_mut(name) {
            signal.normalized = normalized;
        }
    }

    pub fn run(&self) -> FingerprintEntropyResult {
        let mut result = FingerprintEntropyResult::new();

        result.signal_count = self.signals.len();

        for signal in self.signals.values() {
            result.raw_entropy_bits += signal.entropy_bits;

            let mut effective = signal.entropy_bits;

            if signal.normalized {
                effective *= 0.5;
            }

            if signal.randomized {
                effective *= 0.25;
            }

            effective *= signal.uniqueness;

            result.effective_entropy_bits += effective;
        }

        result.normalized_entropy =
            (result.effective_entropy_bits / self.entropy_reference_bits)
                .clamp(0.0, 1.0);

        result.uniqueness_score =
            (1.0 - result.normalized_entropy) * 100.0;

        result
    }

    pub fn signals(&self) -> impl Iterator<Item = &FingerprintSignal> {
        self.signals.values()
    }

    pub fn remove_signal(&mut self, name: &str) {
        self.signals.remove(name);
    }

    pub fn clear(&mut self) {
        self.signals.clear();
    }
}

impl Default for FingerprintEntropy {
    fn default() -> Self {
        Self::new()
    }
}
