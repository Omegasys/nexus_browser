/*
 * Nexus Browser - Servo Frame
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::document::Document;

pub struct Frame {
    id: u64,
    document: Document,
    active: bool,
}

impl Frame {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            document: Document::new(id),
            active: false,
        }
    }

    pub fn initialize(&mut self) {
        self.active = true;
    }

    pub fn close(&mut self) {
        self.document.unload();
        self.active = false;
    }

    pub fn navigate(
        &mut self,
        url: &str,
    ) -> Result<(), String> {
        if !self.active {
            return Err(
                "Frame is not active".to_string()
            );
        }

        self.document.load(url)
    }

    pub fn reload(&mut self) -> Result<(), String> {
        let url = self.document.url().to_string();

        if url.is_empty() {
            return Err(
                "Frame has no loaded document".to_string()
            );
        }

        self.document.load(&url)
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn document_mut(&mut self) -> &mut Document {
        &mut self.document
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
