use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkTrackingMode {
    Disabled,
    StripKnownTracking,
    StripAggressive,
}

#[derive(Debug, Clone)]
pub struct LinkTrackingProtection {
    pub mode: LinkTrackingMode,
    tracking_parameters: HashSet<String>,
}

impl LinkTrackingProtection {
    pub fn new() -> Self {
        let mut tracking_parameters = HashSet::new();

        for parameter in [
            "utm_source",
            "utm_medium",
            "utm_campaign",
            "utm_term",
            "utm_content",
            "utm_id",
            "gclid",
            "dclid",
            "fbclid",
            "msclkid",
            "mc_cid",
            "mc_eid",
            "ref",
            "referrer",
            "affiliate",
            "aff",
        ] {
            tracking_parameters.insert(parameter.to_string());
        }

        Self {
            mode: LinkTrackingMode::StripKnownTracking,
            tracking_parameters,
        }
    }

    pub fn set_mode(&mut self, mode: LinkTrackingMode) {
        self.mode = mode;
    }

    pub fn add_parameter(&mut self, parameter: impl Into<String>) {
        self.tracking_parameters
            .insert(parameter.into().to_ascii_lowercase());
    }

    pub fn remove_tracking_parameters(&self, url: &str) -> String {
        if self.mode == LinkTrackingMode::Disabled {
            return url.to_string();
        }

        let Some((base, query)) = url.split_once('?') else {
            return url.to_string();
        };

        let mut retained = Vec::new();

        for parameter in query.split('&') {
            let name = parameter
                .split('=')
                .next()
                .unwrap_or("")
                .to_ascii_lowercase();

            let should_remove = self.tracking_parameters.contains(&name)
                || (self.mode == LinkTrackingMode::StripAggressive
                    && (name.starts_with("utm_") || name.contains("track")));

            if !should_remove && !parameter.is_empty() {
                retained.push(parameter);
            }
        }

        if retained.is_empty() {
            base.to_string()
        } else {
            format!("{}?{}", base, retained.join("&"))
        }
    }

    pub fn is_tracking_parameter(&self, parameter: &str) -> bool {
        self.tracking_parameters
            .contains(&parameter.to_ascii_lowercase())
    }
}

impl Default for LinkTrackingProtection {
    fn default() -> Self {
        Self::new()
    }
}
