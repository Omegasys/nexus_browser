use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineSecurityState {
    Unknown,
    Staged,
    Testing,
    Limited,
    Approved,
    Trusted,
    Suspicious,
    Quarantined,
    Revoked,
}

#[derive(Debug, Clone)]
pub struct EngineSecurityRecord {
    pub engine: String,
    pub version: String,
    pub state: EngineSecurityState,
    pub signature_valid: bool,
    pub integrity_valid: bool,
    pub sandbox_required: bool,
}

#[derive(Debug)]
pub struct EngineSecurity {
    engines: HashMap<String, EngineSecurityRecord>,
}

impl EngineSecurity {
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        engine: impl Into<String>,
        version: impl Into<String>,
    ) {
        let engine = engine.into();

        self.engines.insert(
            engine.clone(),
            EngineSecurityRecord {
                engine,
                version: version.into(),
                state: EngineSecurityState::Unknown,
                signature_valid: false,
                integrity_valid: false,
                sandbox_required: true,
            },
        );
    }

    pub fn set_state(
        &mut self,
        engine: &str,
        state: EngineSecurityState,
    ) -> bool {
        let Some(record) = self.engines.get_mut(engine) else {
            return false;
        };

        record.state = state;
        true
    }

    pub fn set_signature_status(
        &mut self,
        engine: &str,
        valid: bool,
    ) -> bool {
        let Some(record) = self.engines.get_mut(engine) else {
            return false;
        };

        record.signature_valid = valid;
        true
    }

    pub fn set_integrity_status(
        &mut self,
        engine: &str,
        valid: bool,
    ) -> bool {
        let Some(record) = self.engines.get_mut(engine) else {
            return false;
        };

        record.integrity_valid = valid;
        true
    }

    pub fn approve(&mut self, engine: &str) -> bool {
        let Some(record) = self.engines.get_mut(engine) else {
            return false;
        };

        if !record.signature_valid || !record.integrity_valid {
            return false;
        }

        record.state = EngineSecurityState::Approved;
        true
    }

    pub fn trust(&mut self, engine: &str) -> bool {
        let Some(record) = self.engines.get_mut(engine) else {
            return false;
        };

        if !record.signature_valid || !record.integrity_valid {
            return false;
        }

        record.state = EngineSecurityState::Trusted;
        true
    }

    pub fn quarantine(&mut self, engine: &str) -> bool {
        self.set_state(
            engine,
            EngineSecurityState::Quarantined,
        )
    }

    pub fn revoke(&mut self, engine: &str) -> bool {
        self.set_state(
            engine,
            EngineSecurityState::Revoked,
        )
    }

    pub fn can_execute(&self, engine: &str) -> bool {
        self.engines
            .get(engine)
            .map(|record| {
                matches!(
                    record.state,
                    EngineSecurityState::Approved
                        | EngineSecurityState::Trusted
                )
                    && record.signature_valid
                    && record.integrity_valid
            })
            .unwrap_or(false)
    }

    pub fn get(&self, engine: &str) -> Option<&EngineSecurityRecord> {
        self.engines.get(engine)
    }

    pub fn engines(&self) -> impl Iterator<Item = &EngineSecurityRecord> {
        self.engines.values()
    }
}

impl Default for EngineSecurity {
    fn default() -> Self {
        Self::new()
    }
}
