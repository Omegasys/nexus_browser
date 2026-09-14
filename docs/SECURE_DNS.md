# Nexus Browser Secure DNS Architecture

## Overview

Nexus Browser treats DNS as a critical privacy and security component.

Traditional DNS can expose browsing activity because domain lookups may be visible to network operators, internet providers, or other observers.

Nexus provides a modular Secure DNS system designed to prevent DNS leaks and provide user control.

## Design Goals

The Secure DNS system provides:

- Encrypted DNS communication
- Resolver flexibility
- DNS leak prevention
- DNS isolation
- Workspace-specific DNS policies
- Fail-closed DNS protection

## DNS Architecture

The DNS system operates independently from the browser rendering system.

The architecture contains:

- DNS Manager
- Resolver Manager
- DNS Engines
- DNS Cache
- DNS Policy Engine
- DNS Leak Protection

## DNS Manager

The DNS Manager controls all DNS activity.

Responsibilities:

- Select DNS method
- Apply DNS policies
- Manage resolvers
- Monitor DNS requests
- Enforce privacy settings

## Supported DNS Technologies

Nexus supports:

- DNS over HTTPS (DoH)
- DNS over TLS (DoT)
- DNSCrypt
- DNSSEC
- Custom DNS resolvers
- Network-provided DNS when explicitly allowed

## DNS over HTTPS

DoH sends DNS queries through HTTPS connections.

Advantages:

- Encrypted DNS traffic
- Uses standard HTTPS infrastructure
- Reduces DNS monitoring

Nexus supports:

- Multiple DoH providers
- Custom DoH endpoints
- Resolver health monitoring
- DoH-only profiles

## DNS over TLS

DoT provides encrypted DNS using TLS.

Features:

- Dedicated DNS encryption channel
- Resolver authentication
- Enterprise compatibility

## DNSCrypt

DNSCrypt provides encrypted and authenticated DNS communication.

Features:

- Resolver authentication
- Privacy-focused DNS transport
- Alternative to HTTPS-based DNS

## DNSSEC

DNSSEC validates DNS responses.

Features:

- Authenticity verification
- Domain response validation
- Protection against DNS manipulation

## Resolver Manager

The Resolver Manager manages available DNS providers.

Functions:

- Add resolvers
- Remove resolvers
- Test resolver availability
- Select preferred resolver
- Monitor resolver reliability

## Resolver Profiles

Users can create resolver profiles.

Examples:

## Privacy Profile

Uses:

- DoH
- DNSCrypt
- No fallback

## Compatibility Profile

Uses:

- System DNS
- Secure fallback

## Tor Profile

Uses:

- Tor-compatible DNS handling
- No direct DNS requests

## VPN Profile

Uses:

- VPN-provided resolver
- VPN DNS enforcement

## DNS Isolation

Nexus supports DNS separation between:

- Tabs
- Workspaces
- Profiles
- MicroVM compartments

Example:

Personal workspace:

- Private DoH resolver

Research workspace:

- Tor DNS

Development workspace:

- Local resolver

## DNS Cache Protection

DNS caching can reveal browsing history.

Nexus protects:

- DNS cache contents
- Cache lifetime
- Cache sharing

Features:

- Partitioned DNS cache
- Workspace isolation
- Cache clearing controls

## DNS Leak Protection

Nexus prevents:

- System DNS fallback
- VPN DNS leaks
- IPv6 DNS leaks
- Browser DNS leaks

## DNS Kill Switch

The DNS kill switch prevents unsafe DNS fallback.

Example:

Required:

- DNS over HTTPS

DoH fails:

Nexus response:

- Block DNS requests
- Prevent plaintext DNS
- Notify user

## User Controls

Users can configure:

- DNS method
- Resolver choice
- Fallback behavior
- Logging level
- Cache behavior

## Security Logging

DNS events include:

- Resolver changes
- Failed queries
- Blocked fallback attempts
- DNS policy violations

## Future Research

Possible future features:

- Decentralized DNS
- Peer-to-peer DNS
- AI-assisted DNS threat detection
- Automatic resolver trust scoring
- Anonymous DNS routing
