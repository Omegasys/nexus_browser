//! Nexus Browser threat detection system.
//!
//! Detects suspicious browser behavior,
//! abnormal engine behavior, and security events.

use std::collections::VecDeque;


#[derive(Debug, Clone)]
pub enum ThreatLevel {

    None,

    Low,

    Medium,

    High,

    Critical,

}



#[derive(Debug, Clone)]
pub enum ThreatType {

    SuspiciousScript,

    MemoryAbuse,

    SandboxViolation,

    ProcessViolation,

    NetworkAnomaly,

    EngineFailure,

    PermissionAbuse,

    Unknown,

}



#[derive(Debug, Clone)]
pub struct ThreatEvent {

    pub threat_type: ThreatType,

    pub level: ThreatLevel,

    pub source: String,

    pub description: String,

    pub timestamp: u64,

}



pub struct ThreatDetector {

    events: VecDeque<ThreatEvent>,

    monitoring: bool,

    max_events: usize,

}



impl ThreatDetector {


    pub fn new() -> Self {

        Self {

            events: VecDeque::new(),

            monitoring: false,

            max_events: 10000,

        }

    }



    pub fn start_monitoring(
        &mut self
    ) {

        self.monitoring = true;

    }



    pub fn stop_monitoring(
        &mut self
    ) {

        self.monitoring = false;

    }



    pub fn is_monitoring(
        &self
    ) -> bool {

        self.monitoring

    }



    pub fn report(
        &mut self,
        event: ThreatEvent
    ) {


        if self.events.len()
            >= self.max_events {

            self.events.pop_front();

        }


        self.events.push_back(event);

    }



    pub fn recent_events(
        &self
    ) -> Vec<ThreatEvent> {

        self.events
            .iter()
            .cloned()
            .collect()

    }



    pub fn highest_threat(
        &self
    ) -> ThreatLevel {


        if self.events.iter()
            .any(|e|
                matches!(
                    e.level,
                    ThreatLevel::Critical
                )
            ) {

            return ThreatLevel::Critical;

        }


        if self.events.iter()
            .any(|e|
                matches!(
                    e.level,
                    ThreatLevel::High
                )
            ) {

            return ThreatLevel::High;

        }


        ThreatLevel::None

    }

}


impl Default for ThreatDetector {

    fn default() -> Self {

        Self::new()

    }

}
