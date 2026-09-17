use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensorType {
    Accelerometer,
    Gyroscope,
    Magnetometer,
    AmbientLight,
    Proximity,
    Motion,
    Orientation,
    DeviceMotion,
    DeviceOrientation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorAction {
    Allow,
    Prompt,
    Block,
    ReducePrecision,
}

#[derive(Debug, Clone)]
pub struct SensorProtection {
    default_action: SensorAction,
    rules: HashMap<SensorType, SensorAction>,
}

impl SensorProtection {
    pub fn new() -> Self {
        Self {
            default_action: SensorAction::Prompt,
            rules: HashMap::new(),
        }
    }

    pub fn set_default_action(&mut self, action: SensorAction) {
        self.default_action = action;
    }

    pub fn set_rule(&mut self, sensor: SensorType, action: SensorAction) {
        self.rules.insert(sensor, action);
    }

    pub fn action_for(&self, sensor: SensorType) -> SensorAction {
        self.rules
            .get(&sensor)
            .copied()
            .unwrap_or(self.default_action)
    }

    pub fn is_allowed(&self, sensor: SensorType) -> bool {
        self.action_for(sensor) == SensorAction::Allow
    }

    pub fn is_blocked(&self, sensor: SensorType) -> bool {
        self.action_for(sensor) == SensorAction::Block
    }

    pub fn should_reduce_precision(&self, sensor: SensorType) -> bool {
        self.action_for(sensor) == SensorAction::ReducePrecision
    }
}

impl Default for SensorProtection {
    fn default() -> Self {
        Self::new()
    }
}
