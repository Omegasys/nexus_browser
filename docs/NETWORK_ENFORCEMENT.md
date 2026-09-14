# Nexus Browser Network Enforcement Architecture

## Overview

Network Enforcement is the security layer responsible for ensuring that browser traffic follows user-defined networking policies.

The purpose is to prevent accidental privacy failures caused by:

- Network failures
- DNS leaks
- VPN disconnects
- Tor failures
- Protocol bypasses
- Application fallback behavior

## Core Principle

Nexus follows a fail-closed security model.

If a user requires a privacy or security layer, Nexus should block traffic rather than silently bypass that protection.

## Network Policy Engine

The Network Policy Engine controls:

- Allowed networks
- Blocked networks
- Allowed protocols
- DNS requirements
- IP requirements
- Routing rules

## Security Profiles

Users can create profiles.

Examples:

## Compatibility Mode

Allows:

- Direct connections
- IPv4
- IPv6
- TCP
- UDP
- QUIC

## Privacy Mode

Enables:

- Secure DNS
- HTTPS enforcement
- Tracker protection
- Leak prevention

## Tor Lockdown

Requires:

- Tor routing
- No direct connections
- DNS protection
- Network lock

## VPN Lockdown

Requires:

- Active VPN tunnel
- VPN DNS
- No direct traffic

## Network Lock

Network Lock prevents traffic from bypassing required routes.

Example:

VPN active:

Traffic:

Browser

↓

VPN tunnel

↓

Internet

If VPN fails:

Browser

↓

Blocked

The browser does not automatically use a direct connection.

## Multi-Layer Kill Switch

Nexus expands the traditional VPN kill switch.

Supported protections:

- VPN kill switch
- Tor kill switch
- DNS kill switch
- HTTPS lock
- Protocol lock
- IPv4/IPv6 lock
- Network route lock

## DNS Enforcement

DNS policies can require:

- DNS over HTTPS
- DNS over TLS
- DNSCrypt
- Custom resolver

If secure DNS fails:

Allowed behavior:

- Block DNS requests
- Notify user

Disallowed behavior:

- Falling back to plaintext DNS

## HTTPS Enforcement

HTTPS policies control:

- HTTP blocking
- HTTPS upgrades
- TLS requirements
- Certificate validation

Example:

HTTP website:

Attempt connection

↓

HTTPS available

↓

Upgrade

If HTTPS unavailable:

Block or warn based on policy.

## Protocol Filtering

Users can control:

Allowed:

- TCP
- UDP
- QUIC
- WebRTC
- WebTransport

Blocked protocols cannot bypass the network policy.

## IPv4 and IPv6 Enforcement

Users may configure:

IPv4:

- Allowed
- Blocked
- Required

IPv6:

- Allowed
- Blocked
- Required

This prevents unexpected network paths.

## Leak Prevention

Nexus monitors for:

- DNS leaks
- IPv6 leaks
- WebRTC leaks
- Proxy leaks
- VPN leaks

## Per-Workspace Enforcement

Each workspace may have separate policies.

Example:

Personal Workspace:

- VPN required

Research Workspace:

- Tor required

Development Workspace:

- Direct networking allowed

## Per-Tab Enforcement

Experimental support:

Each tab may have:

- Network route
- DNS policy
- Protocol policy
- Security level

## Network Failure Handling

When a required service fails:

Nexus can:

- Pause connections
- Block new connections
- Display security warning
- Restore automatically when safe

## Logging

Network enforcement records:

- Blocked connections
- Failed routes
- DNS failures
- Policy violations

## Future Research

Possible future features:

- Automatic threat detection
- AI-assisted network auditing
- Adaptive security profiles
- Decentralized routing verification
