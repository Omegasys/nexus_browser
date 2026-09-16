// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackerType {
    Http,
    Https,
    Udp,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Tracker {
    pub url: String,
    pub tracker_type: TrackerType,
    pub enabled: bool,
}

pub struct TrackerManager {
    trackers: Vec<Tracker>,
}

impl TrackerManager {
    pub fn new() -> Self {
        Self {
            trackers: Vec::new(),
        }
    }

    pub fn add_tracker(&mut self, url: impl Into<String>) {
        let url = url.into();

        let tracker_type = if url.starts_with("https://") {
            TrackerType::Https
        } else if url.starts_with("http://") {
            TrackerType::Http
        } else if url.starts_with("udp://") {
            TrackerType::Udp
        } else {
            TrackerType::Unknown
        };

        self.trackers.push(Tracker {
            url,
            tracker_type,
            enabled: true,
        });
    }

    pub fn remove_tracker(&mut self, url: &str) {
        self.trackers.retain(|tracker| tracker.url != url);
    }

    pub fn set_enabled(&mut self, url: &str, enabled: bool) {
        if let Some(tracker) = self.trackers.iter_mut().find(|t| t.url == url) {
            tracker.enabled = enabled;
        }
    }

    pub fn enabled_trackers(&self) -> impl Iterator<Item = &Tracker> {
        self.trackers.iter().filter(|tracker| tracker.enabled)
    }

    pub fn trackers(&self) -> &[Tracker] {
        &self.trackers
    }
}

impl Default for TrackerManager {
    fn default() -> Self {
        Self::new()
    }
}
