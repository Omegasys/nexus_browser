/*
 * Nexus Browser - Blink Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_BLINK_PUBLIC_ENGINE_HPP
#define NEXUS_BLINK_PUBLIC_ENGINE_HPP

#include <cstdint>

#include "rendering_types.hpp"

namespace nexus::blink {

class NexusBridge;

class BlinkEngine {
public:
    BlinkEngine();
    ~BlinkEngine();

    BlinkEngine(const BlinkEngine&) = delete;
    BlinkEngine& operator=(const BlinkEngine&) = delete;

    bool initialize(const EngineConfiguration& configuration);
    bool shutdown();

    bool reset();

    bool is_initialized() const;
    bool is_running() const;
    bool is_healthy() const;

    EngineStatus status() const;

    bool create_page(std::uint64_t& page_id);
    bool destroy_page(std::uint64_t page_id);

    bool navigate(
        std::uint64_t page_id,
        const char* url
    );

    bool reload(std::uint64_t page_id);

    NexusBridge* bridge();

private:
    class Impl;
    Impl* implementation_;
};

} // namespace nexus::blink

#endif
