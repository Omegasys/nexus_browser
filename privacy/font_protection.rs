use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontProtectionMode {
    Disabled,
    RestrictEnumeration,
    Standardized,
}

#[derive(Debug, Clone)]
pub struct FontProtection {
    pub mode: FontProtectionMode,
    allowed_fonts: HashSet<String>,
}

impl FontProtection {
    pub fn new() -> Self {
        let mut allowed_fonts = HashSet::new();

        for font in [
            "Arial",
            "Helvetica",
            "Times New Roman",
            "Courier New",
            "sans-serif",
            "serif",
            "monospace",
        ] {
            allowed_fonts.insert(font.to_string());
        }

        Self {
            mode: FontProtectionMode::RestrictEnumeration,
            allowed_fonts,
        }
    }

    pub fn set_mode(&mut self, mode: FontProtectionMode) {
        self.mode = mode;
    }

    pub fn add_allowed_font(&mut self, font: impl Into<String>) {
        self.allowed_fonts.insert(font.into());
    }

    pub fn is_font_allowed(&self, font: &str) -> bool {
        match self.mode {
            FontProtectionMode::Disabled => true,
            FontProtectionMode::RestrictEnumeration
            | FontProtectionMode::Standardized => self.allowed_fonts.contains(font),
        }
    }

    pub fn enumerate_fonts(&self) -> Vec<String> {
        if self.mode == FontProtectionMode::Disabled {
            return self.allowed_fonts.iter().cloned().collect();
        }

        self.allowed_fonts.iter().cloned().collect()
    }
}

impl Default for FontProtection {
    fn default() -> Self {
        Self::new()
    }
}
