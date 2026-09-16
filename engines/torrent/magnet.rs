// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MagnetLink {
    pub info_hash: String,
    pub display_name: Option<String>,
    pub trackers: Vec<String>,
    pub parameters: HashMap<String, String>,
}

impl MagnetLink {
    pub fn parse(uri: &str) -> Result<Self, String> {
        if !uri.starts_with("magnet:?") {
            return Err("Invalid magnet URI".into());
        }

        let query = &uri["magnet:?".len()..];

        let mut info_hash = None;
        let mut display_name = None;
        let mut trackers = Vec::new();
        let mut parameters = HashMap::new();

        for item in query.split('&') {
            let mut parts = item.splitn(2, '=');

            let key = parts.next().unwrap_or("");
            let value = parts.next().unwrap_or("");

            match key {
                "xt" if value.starts_with("urn:btih:") => {
                    info_hash = Some(value["urn:btih:".len()..].to_string());
                }
                "dn" => {
                    display_name = Some(value.to_string());
                }
                "tr" => {
                    trackers.push(value.to_string());
                }
                _ => {
                    parameters.insert(key.to_string(), value.to_string());
                }
            }
        }

        let info_hash = info_hash.ok_or_else(|| {
            "Magnet link does not contain a BitTorrent info hash".to_string()
        })?;

        Ok(Self {
            info_hash,
            display_name,
            trackers,
            parameters,
        })
    }

    pub fn to_uri(&self) -> String {
        let mut uri = format!("magnet:?xt=urn:btih:{}", self.info_hash);

        if let Some(name) = &self.display_name {
            uri.push_str("&dn=");
            uri.push_str(name);
        }

        for tracker in &self.trackers {
            uri.push_str("&tr=");
            uri.push_str(tracker);
        }

        uri
    }
}
