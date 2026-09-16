// SPDX-License-Identifier: GPL-3.0-or-later

use super::local_model_runner::LocalModelRunner;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SummaryLength {
    Short,
    Medium,
    Long,
}

pub struct TabSummarizer {
    enabled: bool,
    summary_length: SummaryLength,
}

impl TabSummarizer {
    pub fn new() -> Self {
        Self {
            enabled: true,
            summary_length: SummaryLength::Medium,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_length(&mut self, length: SummaryLength) {
        self.summary_length = length;
    }

    pub fn summarize(
        &self,
        model: &mut LocalModelRunner,
        title: &str,
        content: &str,
    ) -> Result<String, String> {
        if !self.enabled {
            return Err("Tab summarizer is disabled".into());
        }

        if content.is_empty() {
            return Err("Tab content cannot be empty".into());
        }

        let prompt = format!(
            "Summarize the following browser tab.\n\
             Title: {}\n\
             Length: {:?}\n\
             Content:\n{}",
            title,
            self.summary_length,
            content
        );

        model.generate(&prompt)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn summary_length(&self) -> SummaryLength {
        self.summary_length
    }
}

impl Default for TabSummarizer {
    fn default() -> Self {
        Self::new()
    }
}
