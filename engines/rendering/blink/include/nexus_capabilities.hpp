/*
 * Nexus Browser - Blink Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_BLINK_NEXUS_CAPABILITIES_HPP
#define NEXUS_BLINK_NEXUS_CAPABILITIES_HPP

#include <cstdint>

namespace nexus::blink {

enum class Capability {
    None,

    Render,
    JavaScript,

    Network,
    NetworkTCP,
    NetworkUDP,

    DNS,

    Storage,
    PersistentStorage,
    TemporaryStorage,

    Cookies,

    WebRTC,

    WebGL,
    GPU,

    Clipboard,

    Camera,
    Microphone,

    Notifications,

    FileRead,
    FileWrite,

    Downloads,

    Printing,

    DevTools
};

struct CapabilitySet {
    bool render = true;
    bool javascript = true;

    bool network = false;
    bool network_tcp = false;
    bool network_udp = false;

    bool dns = false;

    bool storage = false;
    bool persistent_storage = false;
    bool temporary_storage = true;

    bool cookies = false;

    bool webrtc = false;

    bool webgl = false;
    bool gpu = false;

    bool clipboard = false;

    bool camera = false;
    bool microphone = false;

    bool notifications = false;

    bool file_read = false;
    bool file_write = false;

    bool downloads = false;
    bool printing = false;

    bool devtools = false;
};

class CapabilityManager {
public:
    CapabilityManager();
    ~CapabilityManager();

    bool grant(
        Capability capability
    );

    bool revoke(
        Capability capability
    );

    bool has(
        Capability capability
    ) const;

    bool set(
        Capability capability,
        bool enabled
    );

    void revoke_all();

    CapabilitySet capabilities() const;

private:
    CapabilitySet capabilities_;
};

const char* capability_name(
    Capability capability
);

} // namespace nexus::blink

#endif
