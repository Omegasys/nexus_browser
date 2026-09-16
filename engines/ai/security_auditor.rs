// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditSeverity {
    Information,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct AuditFinding {
    pub severity: AuditSeverity,
    pub component: String,
    pub description: String,
}

pub struct SecurityAuditor {
    enabled: bool,
    findings: Vec<AuditFinding>,
}

impl SecurityAuditor {
    pub fn new() -> Self {
        Self {
            enabled: true,
            findings: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn audit_component(
        &mut self,
        component: impl Into<String>,
    ) -> Result<(), String> {
        if !self.enabled {
            return Err("Security auditor is disabled".into());
        }

        let component = component.into();

        if component.is_empty() {
            return Err("Component name cannot be empty".into());
        }

        self.findings.push(AuditFinding {
            severity: AuditSeverity::Information,
            component,
            description: "Component audit completed".into(),
        });

        Ok(())
    }

    pub fn add_finding(
        &mut self,
        severity: AuditSeverity,
        component: impl Into<String>,
        description: impl Into<String>,
    ) {
        self.findings.push(AuditFinding {
            severity,
            component: component.into(),
            description: description.into(),
        });
    }

    pub fn findings(&self) -> &[AuditFinding] {
        &self.findings
    }

    pub fn clear(&mut self) {
        self.findings.clear();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for SecurityAuditor {
    fn default() -> Self {
        Self::new()
    }
}
