#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkLockUiState {
    Disabled,
    Initializing,
    Locked,
    Unlocking,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkLockLayer {
    AllTraffic,
    Dns,
    Ipv4,
    Ipv6,
    Tcp,
    Udp,
    Quic,
    DirectConnections,
    ProxyBypass,
    VpnBypass,
    TorBypass,
    WebRtc,
}

#[derive(Debug, Clone)]
pub struct NetworkLockLayerState {
    pub layer: NetworkLockLayer,
    pub enabled: bool,
    pub locked: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkLockUi {
    state: NetworkLockUiState,
    fail_closed: bool,
    prevent_direct_fallback: bool,
    layers: Vec<NetworkLockLayerState>,
    last_error: Option<String>,
}

impl Default for NetworkLockUi {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkLockUi {
    pub fn new() -> Self {
        let layers = vec![
            NetworkLockLayer::AllTraffic,
            NetworkLockLayer::Dns,
            NetworkLockLayer::Ipv4,
            NetworkLockLayer::Ipv6,
            NetworkLockLayer::Tcp,
            NetworkLockLayer::Udp,
            NetworkLockLayer::Quic,
            NetworkLockLayer::DirectConnections,
            NetworkLockLayer::ProxyBypass,
            NetworkLockLayer::VpnBypass,
            NetworkLockLayer::TorBypass,
            NetworkLockLayer::WebRtc,
        ]
        .into_iter()
        .map(|layer| NetworkLockLayerState {
            layer,
            enabled: true,
            locked: false,
        })
        .collect();

        Self {
            state: NetworkLockUiState::Disabled,
            fail_closed: true,
            prevent_direct_fallback: true,
            layers,
            last_error: None,
        }
    }

    pub fn state(&self) -> NetworkLockUiState {
        self.state
    }

    pub fn set_state(&mut self, state: NetworkLockUiState) {
        self.state = state;
    }

    pub fn fail_closed(&self) -> bool {
        self.fail_closed
    }

    pub fn set_fail_closed(&mut self, enabled: bool) {
        self.fail_closed = enabled;
    }

    pub fn prevent_direct_fallback(&self) -> bool {
        self.prevent_direct_fallback
    }

    pub fn set_prevent_direct_fallback(&mut self, enabled: bool) {
        self.prevent_direct_fallback = enabled;
    }

    pub fn layers(&self) -> &[NetworkLockLayerState] {
        &self.layers
    }

    pub fn enable_layer(&mut self, layer: NetworkLockLayer) {
        if let Some(state) = self.layers.iter_mut().find(|state| state.layer == layer) {
            state.enabled = true;
        }
    }

    pub fn disable_layer(&mut self, layer: NetworkLockLayer) {
        if let Some(state) = self.layers.iter_mut().find(|state| state.layer == layer) {
            state.enabled = false;
            state.locked = false;
        }
    }

    pub fn set_layer_locked(
        &mut self,
        layer: NetworkLockLayer,
        locked: bool,
    ) {
        if let Some(state) = self.layers.iter_mut().find(|state| state.layer == layer) {
            state.locked = locked;
        }
    }

    pub fn is_layer_locked(&self, layer: NetworkLockLayer) -> bool {
        self.layers
            .iter()
            .find(|state| state.layer == layer)
            .map(|state| state.enabled && state.locked)
            .unwrap_or(false)
    }

    pub fn lock_all(&mut self) {
        self.state = NetworkLockUiState::Locked;

        for layer in &mut self.layers {
            if layer.enabled {
                layer.locked = true;
            }
        }

        self.last_error = None;
    }

    pub fn unlock_all(&mut self) {
        self.state = NetworkLockUiState::Disabled;

        for layer in &mut self.layers {
            layer.locked = false;
        }
    }

    pub fn set_error(&mut self, message: impl Into<String>) {
        self.state = NetworkLockUiState::Error;
        self.last_error = Some(message.into());
    }

    pub fn clear_error(&mut self) {
        self.last_error = None;

        if self.state == NetworkLockUiState::Error {
            self.state = NetworkLockUiState::Disabled;
        }
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn all_enabled_layers_locked(&self) -> bool {
        self.layers
            .iter()
            .filter(|layer| layer.enabled)
            .all(|layer| layer.locked)
    }
}
