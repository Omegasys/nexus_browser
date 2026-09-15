/*
 * Nexus Browser - Servo Document
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

pub struct Document {
    id: u64,
    url: String,
    html: String,
    loaded: bool,
}

impl Document {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            url: String::new(),
            html: String::new(),
            loaded: false,
        }
    }

    pub fn load(
        &mut self,
        url: &str,
    ) -> Result<(), String> {
        if url.is_empty() {
            return Err(
                "Document URL is empty".to_string()
            );
        }

        self.url = url.to_string();
        self.loaded = true;

        Ok(())
    }

    pub fn unload(&mut self) {
        self.url.clear();
        self.html.clear();
        self.loaded = false;
    }

    pub fn set_html(
        &mut self,
        html: impl Into<String>,
    ) {
        self.html = html.into();
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn html(&self) -> &str {
        &self.html
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }
}
