/*
 * Nexus Browser - Servo Nexus Bridge
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_SERVO_NEXUS_BRIDGE_HPP
#define NEXUS_SERVO_NEXUS_BRIDGE_HPP

#include <cstdint>

namespace nexus::servo {

class NexusBridge {
public:
    NexusBridge();
    ~NexusBridge();

    NexusBridge(const NexusBridge&) = delete;
    NexusBridge& operator=(const NexusBridge&) = delete;

    bool initialize();
    bool shutdown();

    bool is_connected() const;

    bool request_network_access(
        std::uint64_t page_id,
        const char* destination
    );

    bool request_storage_access(
        std::uint64_t page_id
    );

    bool request_javascript_execution(
        std::uint64_t page_id
    );

    bool request_wasm_execution(
        std::uint64_t page_id
    );

    bool request_gpu_access(
        std::uint64_t page_id
    );

    bool report_navigation(
        std::uint64_t page_id,
        const char* url
    );

    bool report_security_event(
        std::uint64_t page_id,
        const char* event
    );

    bool check_capability(
        std::uint64_t page_id,
        const char* capability
    ) const;

private:
    bool connected_ = false;
};

} // namespace nexus::servo

#endif
