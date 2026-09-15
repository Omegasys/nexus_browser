//! Core VPN engine interface.
//!
//! Handles communication with VPN providers
//! and tunnel implementations.

use super::tunnel::VpnTunnel;


#[derive(Debug, Clone)]
pub enum VpnState {

    Disconnected,

    Connecting,

    Connected,

    Disconnecting,

    Failed,

}



#[derive(Debug, Clone)]

pub enum VpnProtocol {

    WireGuard,

    OpenVPN,

    IKEv2,

    Custom,

}



pub struct VpnEngine {


    provider: String,

    protocol: VpnProtocol,

    state: VpnState,

    tunnel: Option<VpnTunnel>,


}



impl VpnEngine {


    pub fn new(
        provider: String,
        protocol: VpnProtocol
    ) -> Self {


        Self {

            provider,

            protocol,

            state:
                VpnState::Disconnected,

            tunnel:
                None,

        }

    }



    pub fn connect(
        &mut self
    ) {


        self.state =
            VpnState::Connecting;


        self.tunnel =
            Some(
                VpnTunnel::new()
            );


        self.state =
            VpnState::Connected;

    }



    pub fn disconnect(
        &mut self
    ) {


        self.tunnel =
            None;


        self.state =
            VpnState::Disconnected;

    }



    pub fn state(
        &self
    ) -> &VpnState {

        &self.state

    }



    pub fn protocol(
        &self
    ) -> &VpnProtocol {

        &self.protocol

    }



    pub fn provider(
        &self
    ) -> &str {

        &self.provider

    }



    pub fn is_connected(
        &self
    ) -> bool {


        matches!(
            self.state,
            VpnState::Connected
        )

    }

}
