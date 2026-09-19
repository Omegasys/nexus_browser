use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Ethernet,
    Wifi,
    Cellular,
    Loopback,
    Tunnel,
    Virtual,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceState {
    Down,
    Up,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub interface_type: InterfaceType,
    pub state: InterfaceState,
    pub ipv4_addresses: Vec<String>,
    pub ipv6_addresses: Vec<String>,
    pub default_route: bool,
}

impl NetworkInterface {
    pub fn new(name: impl Into<String>, interface_type: InterfaceType) -> Self {
        Self {
            name: name.into(),
            interface_type,
            state: InterfaceState::Unknown,
            ipv4_addresses: Vec::new(),
            ipv6_addresses: Vec::new(),
            default_route: false,
        }
    }

    pub fn is_usable(&self) -> bool {
        self.state == InterfaceState::Up
    }
}

#[derive(Debug, Default)]
pub struct InterfaceManager {
    interfaces: HashMap<String, NetworkInterface>,
    preferred_interface: Option<String>,
}

impl InterfaceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, interface: NetworkInterface) {
        self.interfaces.insert(interface.name.clone(), interface);
    }

    pub fn remove(&mut self, name: &str) {
        self.interfaces.remove(name);

        if self.preferred_interface.as_deref() == Some(name) {
            self.preferred_interface = None;
        }
    }

    pub fn set_state(&mut self, name: &str, state: InterfaceState) -> bool {
        if let Some(interface) = self.interfaces.get_mut(name) {
            interface.state = state;
            true
        } else {
            false
        }
    }

    pub fn set_preferred(&mut self, name: impl Into<String>) -> bool {
        let name = name.into();

        if self.interfaces.contains_key(&name) {
            self.preferred_interface = Some(name);
            true
        } else {
            false
        }
    }

    pub fn clear_preferred(&mut self) {
        self.preferred_interface = None;
    }

    pub fn preferred(&self) -> Option<&NetworkInterface> {
        self.preferred_interface
            .as_deref()
            .and_then(|name| self.interfaces.get(name))
    }

    pub fn default_interface(&self) -> Option<&NetworkInterface> {
        if let Some(preferred) = self.preferred() {
            if preferred.is_usable() {
                return Some(preferred);
            }
        }

        self.interfaces
            .values()
            .find(|interface| interface.default_route && interface.is_usable())
    }

    pub fn usable_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces
            .values()
            .filter(|interface| interface.is_usable())
            .collect()
    }

    pub fn interfaces(&self) -> &HashMap<String, NetworkInterface> {
        &self.interfaces
    }
}
