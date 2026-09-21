use std::collections::HashMap;
use std::time::{Duration, Instant};

/// CPU usage category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CpuCategory {
    BrowserKernel,
    BrowserCore,
    Rendering,
    JavaScript,
    Networking,
    Storage,
    Extensions,
    MicroVm,
    Graphics,
    DeveloperTools,
    Other,
}

/// CPU sample for a category.
#[derive(Debug, Clone)]
pub struct CpuSample {
    pub category: CpuCategory,
    pub usage_percent: f64,
    pub duration: Duration,
    pub timestamp: Instant,
}

/// Aggregated CPU statistics.
#[derive(Debug, Clone)]
pub struct CpuStatistics {
    pub average_usage_percent: f64,
    pub peak_usage_percent: f64,
    pub sample_count: usize,
    pub total_duration: Duration,
}

/// CPU profiler.
#[derive(Debug, Clone)]
pub struct CpuProfiler {
    samples: Vec<CpuSample>,
    enabled: bool,
    start_time: Option<Instant>,
}

impl CpuProfiler {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            enabled: true,
            start_time: None,
        }
    }

    pub fn start(&mut self) {
        self.enabled = true;
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.enabled = false;
    }

    pub fn is_running(&self) -> bool {
        self.enabled && self.start_time.is_some()
    }

    pub fn record(
        &mut self,
        category: CpuCategory,
        usage_percent: f64,
        duration: Duration,
    ) {
        if !self.enabled {
            return;
        }

        self.samples.push(CpuSample {
            category,
            usage_percent: usage_percent.clamp(0.0, 100.0),
            duration,
            timestamp: Instant::now(),
        });
    }

    pub fn statistics(&self) -> CpuStatistics {
        if self.samples.is_empty() {
            return CpuStatistics {
                average_usage_percent: 0.0,
                peak_usage_percent: 0.0,
                sample_count: 0,
                total_duration: Duration::ZERO,
            };
        }

        let total_duration: Duration =
            self.samples.iter().map(|sample| sample.duration).sum();

        let weighted_usage: f64 = self
            .samples
            .iter()
            .map(|sample| {
                sample.usage_percent * sample.duration.as_secs_f64()
            })
            .sum();

        let seconds = total_duration.as_secs_f64();

        let average = if seconds > 0.0 {
            weighted_usage / seconds
        } else {
            0.0
        };

        let peak = self
            .samples
            .iter()
            .map(|sample| sample.usage_percent)
            .fold(0.0, f64::max);

        CpuStatistics {
            average_usage_percent: average,
            peak_usage_percent: peak,
            sample_count: self.samples.len(),
            total_duration,
        }
    }

    pub fn category_statistics(
        &self,
    ) -> HashMap<CpuCategory, CpuStatistics> {
        let mut grouped: HashMap<CpuCategory, Vec<&CpuSample>> =
            HashMap::new();

        for sample in &self.samples {
            grouped
                .entry(sample.category)
                .or_default()
                .push(sample);
        }

        let mut result = HashMap::new();

        for (category, samples) in grouped {
            let total_duration: Duration =
                samples.iter().map(|sample| sample.duration).sum();

            let weighted_usage: f64 = samples
                .iter()
                .map(|sample| {
                    sample.usage_percent
                        * sample.duration.as_secs_f64()
                })
                .sum();

            let seconds = total_duration.as_secs_f64();

            let average = if seconds > 0.0 {
                weighted_usage / seconds
            } else {
                0.0
            };

            let peak = samples
                .iter()
                .map(|sample| sample.usage_percent)
                .fold(0.0, f64::max);

            result.insert(
                category,
                CpuStatistics {
                    average_usage_percent: average,
                    peak_usage_percent: peak,
                    sample_count: samples.len(),
                    total_duration,
                },
            );
        }

        result
    }

    pub fn samples(&self) -> &[CpuSample] {
        &self.samples
    }

    pub fn clear(&mut self) {
        self.samples.clear();
        self.start_time = None;
    }
}

impl Default for CpuProfiler {
    fn default() -> Self {
        Self::new()
    }
}
