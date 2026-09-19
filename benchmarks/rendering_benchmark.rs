use std::time::{Duration, Instant};

/// A rendering performance metric.
#[derive(Debug, Clone)]
pub struct RenderingMetric {
    pub name: String,
    pub duration: Duration,
    pub operations: u64,
}

impl RenderingMetric {
    pub fn operations_per_second(&self) -> f64 {
        let seconds = self.duration.as_secs_f64();

        if seconds <= 0.0 {
            return 0.0;
        }

        self.operations as f64 / seconds
    }
}

/// Rendering benchmark result.
#[derive(Debug, Clone)]
pub struct RenderingBenchmarkResult {
    pub total_duration: Duration,
    pub metrics: Vec<RenderingMetric>,
    pub score: f64,
}

impl RenderingBenchmarkResult {
    pub fn new() -> Self {
        Self {
            total_duration: Duration::ZERO,
            metrics: Vec::new(),
            score: 0.0,
        }
    }
}

impl Default for RenderingBenchmarkResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic rendering benchmark.
///
/// The benchmark intentionally operates on abstract callbacks so that the
/// actual Blink, Gecko, Servo, or other rendering implementation can supply
/// its own rendering operation.
#[derive(Debug, Clone)]
pub struct RenderingBenchmark {
    reference_operations_per_second: f64,
    iterations: u64,
}

impl RenderingBenchmark {
    pub fn new() -> Self {
        Self {
            reference_operations_per_second: 100_000.0,
            iterations: 10_000,
        }
    }

    pub fn with_reference(reference: f64) -> Self {
        Self {
            reference_operations_per_second: reference.max(1.0),
            iterations: 10_000,
        }
    }

    pub fn iterations(mut self, iterations: u64) -> Self {
        self.iterations = iterations.max(1);
        self
    }

    pub fn run<F>(&self, name: impl Into<String>, mut render: F)
        -> RenderingBenchmarkResult
    where
        F: FnMut(),
    {
        let start = Instant::now();

        for _ in 0..self.iterations {
            render();
        }

        let duration = start.elapsed();

        let metric = RenderingMetric {
            name: name.into(),
            duration,
            operations: self.iterations,
        };

        let ops_per_second = metric.operations_per_second();

        let score =
            ((ops_per_second / self.reference_operations_per_second) * 100.0)
                .min(100.0);

        RenderingBenchmarkResult {
            total_duration: duration,
            metrics: vec![metric],
            score,
        }
    }

    pub fn reference_operations_per_second(&self) -> f64 {
        self.reference_operations_per_second
    }

    pub fn iterations_count(&self) -> u64 {
        self.iterations
    }
}

impl Default for RenderingBenchmark {
    fn default() -> Self {
        Self::new()
    }
}
