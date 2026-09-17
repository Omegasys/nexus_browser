use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceApi {
    MediaDevices,
    Camera,
    Microphone,
    Gamepad,
    Bluetooth,
    Usb,
    Serial,
    Hid,
    Clipboard,
    Geolocation,
    Battery,
    DeviceMemory,
    HardwareConcurrency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceApiAction {
    Allow,
    Prompt,
    Block,
    Spoof,
}

#[derive(Debug, Clone)]
pub struct DeviceApiProtection {
    default_action: DeviceApiAction,
    rules: HashMap<DeviceApi, DeviceApiAction>,
}

impl DeviceApiProtection {
    pub fn new() -> Self {
        Self {
            default_action: DeviceApiAction::Prompt,
            rules: HashMap::new(),
        }
    }

    pub fn set_default_action(&mut self, action: DeviceApiAction) {
        self.default_action = action;
    }

    pub fn set_rule(&mut self, api: DeviceApi, action: DeviceApiAction) {
        self.rules.insert(api, action);
    }

    pub fn action_for(&self, api: DeviceApi) -> DeviceApiAction {
        self.rules
            .get(&api)
            .copied()
            .unwrap_or(self.default_action)
    }

    pub fn is_allowed(&self, api: DeviceApi) -> bool {
        self.action_for(api) == DeviceApiAction::Allow
    }

    pub fn is_blocked(&self, api: DeviceApi) -> bool {
        self.action_for(api) == DeviceApiAction::Block
    }

    pub fn requires_prompt(&self, api: DeviceApi) -> bool {
        self.action_for(api) == DeviceApiAction::Prompt
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}

impl Default for DeviceApiProtection {
    fn default() -> Self {
        Self::new()
    }
}
