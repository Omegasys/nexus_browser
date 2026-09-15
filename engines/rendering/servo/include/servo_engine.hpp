/*
 * Nexus Browser - Servo Public Engine API
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_SERVO_PUBLIC_ENGINE_HPP
#define NEXUS_SERVO_PUBLIC_ENGINE_HPP

#include <cstdint>

#include "rendering_types.hpp"

namespace nexus::servo {

class NexusBridge;

class ServoEngine {
public:
    ServoEngine();
    ~ServoEngine();

    ServoEngine(const ServoEngine&) = delete;
    ServoEngine& operator=(const ServoEngine&) = delete;

    bool initialize(
        const EngineConfiguration& configuration
    );

    bool shutdown();

    bool reset();

    bool is_initialized() const;
    bool is_running() const;
    bool is_healthy() const;

    EngineStatus status() const;

    bool create_page(
        std::uint64_t& page_id
    );

    bool destroy_page(
        std::uint64_t page_id
    );

    bool navigate(
        std::uint64_t page_id,
        const char* url
    );

    bool reload(
        std::uint64_t page_id
    );

    NexusBridge* bridge();

private:
    class Impl;
    Impl* implementation_;
};

} // namespace nexus::servo

#endif
