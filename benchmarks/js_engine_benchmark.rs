use std::time::{Duration, Instant};

/// A JavaScript engine benchmark metric.
#[derive(Debug, Clone)]
pub struct JsBenchmarkMetric {
    pub name: String,
    pub duration: Duration,
    pub iterations: u64,
}

impl JsBenchmarkMetric {
    pub fn operations_per_second(&self) -> f64 {
        let seconds = self.duration.as_secs_f64();

        if seconds <= 0.0 {
            return 0.0;
        }

        self.iterations as f64 / seconds
    }
}

/// JavaScript benchmark result.
#[derive(Debug, Clone)]
pub struct JsBenchmarkResult {
    pub engine_name: String,
    pub total_duration: Duration,
    pub metrics: Vec<JsBenchmarkMetric>,
    pub score: f64,
}

impl JsBenchmarkResult {
    pub fn new(engine_name: impl Into<String>) -> Self {
        Self {
            engine_name: engine_name.into(),
            total_duration: Duration::ZERO,
            metrics: Vec::new(),
            score: 0.0,
        }
    }
}

/// JavaScript engine benchmark.
///
/// The actual JavaScript execution layer is supplied by the caller. This
/// allows V8, SpiderMonkey, JavaScriptCore, or another engine to be tested
/// without coupling the benchmark system to one implementation.
#[derive(Debug, Clone)]
pub struct JsEngineBenchmark {
    reference_operations_per_second: f64,
    iterations: u64,
}

impl JsEngineBenchmark {
    pub fn new() -> Self {
        Self {
            reference_operations_per_second: 1_000_000.0,
            iterations: 100_000,
        }
    }

    pub fn with_reference(reference: f64) -> Self {
        Self {
            reference_operations_per_second: reference.max(1.0),
            iterations: 100_000,
        }
    }

    pub fn iterations(mut self, iterations: u64) -> Self {
        self.iterations = iterations.max(1);
        self
    }

    pub fn run<F>(
        &self,
        engine_name: impl Into<String>,
        benchmark_name: impl Into<String>,
        mut execute: F,
    ) -> JsBenchmarkResult
    where
        F: FnMut(),
    {
        let start = Instant::now();

        for _ in 0..self.iterations {
            execute();
        }

        let duration = start.elapsed();

        let metric = JsBenchmarkMetric {
            name: benchmark_name.into(),
            duration,
            iterations: self.iterations,
        };

        let ops_per_second = metric.operations_per_second();

        let score =
            ((ops_per_second / self.reference_operations_per_second) * 100.0)
                .min(100.0);

        JsBenchmarkResult {
            engine_name: engine_name.into(),
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

impl Default for JsEngineBenchmark {
    fn default() -> Self {
        Self::new()
    }
}
