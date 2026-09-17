#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasProtectionMode {
    Disabled,
    BlockReadback,
    Prompt,
    AddNoise,
}

#[derive(Debug, Clone)]
pub struct CanvasProtection {
    pub mode: CanvasProtectionMode,
    pub noise_level: f32,
}

impl CanvasProtection {
    pub fn new() -> Self {
        Self {
            mode: CanvasProtectionMode::BlockReadback,
            noise_level: 0.01,
        }
    }

    pub fn set_mode(&mut self, mode: CanvasProtectionMode) {
        self.mode = mode;
    }

    pub fn set_noise_level(&mut self, level: f32) {
        self.noise_level = level.clamp(0.0, 1.0);
    }

    pub fn should_block_readback(&self) -> bool {
        self.mode == CanvasProtectionMode::BlockReadback
    }

    pub fn should_prompt(&self) -> bool {
        self.mode == CanvasProtectionMode::Prompt
    }

    pub fn should_add_noise(&self) -> bool {
        self.mode == CanvasProtectionMode::AddNoise
    }

    pub fn process_readback(&self, pixels: &[u8]) -> Vec<u8> {
        match self.mode {
            CanvasProtectionMode::Disabled
            | CanvasProtectionMode::BlockReadback
            | CanvasProtectionMode::Prompt => pixels.to_vec(),

            CanvasProtectionMode::AddNoise => {
                // Deterministic placeholder. A production implementation
                // should use a cryptographically secure, per-context source.
                let amount = (self.noise_level * 255.0) as u8;

                pixels
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        if index % 17 == 0 {
                            value.wrapping_add(amount)
                        } else {
                            *value
                        }
                    })
                    .collect()
            }
        }
    }
}

impl Default for CanvasProtection {
    fn default() -> Self {
        Self::new()
    }
}
