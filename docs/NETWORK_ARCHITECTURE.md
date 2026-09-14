# Nexus Browser Network Architecture

## Overview

Nexus Browser uses a modular network architecture designed to separate browser functionality from network transport systems.

The goal is to allow users to choose, combine, and control different networking technologies without forcing all traffic through one system.

Supported network systems may include:

- Direct internet connections
- VPN networks
- Tor
- I2P
- Nym Mixnet
- Lokinet
- IPFS
- Other decentralized networks

## Design Goals

The network architecture provides:

- User-controlled routing
- Network independence
- Privacy protection
- Leak prevention
- Protocol control
- Per-profile networking
- Per-workspace networking
- Experimental per-tab routing

## Network Abstraction Layer

The Network Abstraction Layer provides a common interface between Nexus and network providers.

The abstraction layer manages:

- Connection creation
- Routing selection
- Network policies
- Security enforcement
- Provider communication

## Network Providers

Each network provider operates as an independent module.

Providers may include:

- VPN engine
- Tor engine
- I2P engine
- Nym engine
- Lokinet engine
- Direct connection engine

Each provider contains:

- Adapter
- Manifest
- Configuration
- Routing logic
- Security policy support

## Direct Network Engine

The direct network engine provides normal internet connectivity.

Features:

- Standard TCP connections
- HTTPS connections
- IPv4 support
- IPv6 support

Users may disable direct networking completely.

## VPN Network Engine

The VPN engine provides encrypted tunnel support.

Features:

- VPN connection management
- Multi-hop support
- Kill switch integration
- Per-workspace routing
- Split tunneling

The VPN system remains separate from Tor and other anonymity systems.

## Tor Network Engine

The Tor engine provides integration with the Tor network.

Features:

- Onion routing
- Circuit management
- Bridge support
- Identity reset
- Tor-specific network policies

Tor operates independently.

Nexus does not automatically force VPN, I2P, or other systems through Tor unless configured by the user.

## I2P Network Engine

The I2P engine provides access to the Invisible Internet Project.

Features:

- I2P routing
- Tunnel management
- Destination handling
- I2P-specific policies

## Nym Network Engine

The Nym engine provides mixnet-based routing.

Features:

- Mixnet communication
- Privacy routing
- Gateway management

## Lokinet Network Engine

The Lokinet engine provides onion-routing style networking.

Features:

- Service routing
- Tunnel management
- Network isolation

## Decentralized Network Support

Future network modules may include:

- IPFS
- GNUnet
- Freenet
- Hyphanet
- Yggdrasil

## Network Routing Profiles

Users can create routing profiles.

Examples:

## Normal Profile

Uses:

- Direct connection
- Secure DNS
- HTTPS

## VPN Profile

Uses:

- VPN routing
- VPN DNS
- Network lock

## Tor Profile

Uses:

- Tor routing
- Tor-compatible DNS
- Direct connection disabled

## Maximum Privacy Profile

Uses:

- Privacy network
- Secure DNS
- Strict protocol filtering
- Network lock

## Per-Workspace Networking

Each workspace can have independent networking.

Examples:

Research Workspace:

- Tor

Gaming Workspace:

- Direct connection

Private Workspace:

- VPN

Development Workspace:

- Custom routing

## Per-Tab Networking

Experimental support allows individual tabs to have separate network routes.

Examples:

Tab 1:

- Direct connection

Tab 2:

- Tor

Tab 3:

- VPN

## Protocol Support

Nexus controls:

- HTTP
- HTTPS
- TCP
- UDP
- QUIC
- WebRTC
- WebTransport

Users can allow or block protocols.

## IPv4 and IPv6 Control

Users can configure:

- IPv4 enabled
- IPv4 disabled
- IPv6 enabled
- IPv6 disabled

Policies can be applied globally or per profile.

## DNS Integration

DNS is treated as a separate security layer.

Supported systems:

- DNS over HTTPS
- DNS over TLS
- DNSCrypt
- DNSSEC
- Tor DNS handling
- Custom resolvers

## Network Monitoring

The network system monitors:

- Active connections
- Routes
- DNS requests
- Protocol usage
- Leaks

## Future Research

Possible future features:

- Mesh networking
- Browser-to-browser communication
- Decentralized routing
- Automatic censorship detection
- Multi-network experimentation
