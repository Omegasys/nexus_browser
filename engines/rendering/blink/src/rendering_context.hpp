/*
 * Nexus Browser - Blink Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_BLINK_RENDERING_CONTEXT_HPP
#define NEXUS_BLINK_RENDERING_CONTEXT_HPP

#include <cstdint>
#include <string>

#include "blink_engine.hpp"

namespace nexus::blink {

struct RenderingMetrics {
    std::uint64_t frames_rendered = 0;
    std::uint64_t scripts_executed = 0;
    std::uint64_t bytes_rendered = 0;
};

class RenderingContext {
public:
    explicit RenderingContext(const EngineConfiguration& configuration);
    ~RenderingContext();

    RenderingContext(const RenderingContext&) = delete;
    RenderingContext& operator=(const RenderingContext&) = delete;

    bool initialize();
    bool shutdown();

    bool render();

    bool execute_javascript(const std::string& script);

    bool set_viewport(std::uint32_t width, std::uint32_t height);

    std::uint32_t viewport_width() const;
    std::uint32_t viewport_height() const;

    bool is_initialized() const;

    const RenderingMetrics& metrics() const;

private:
    EngineConfiguration configuration_;
    RenderingMetrics metrics_;

    std::uint32_t viewport_width_ = 1280;
    std::uint32_t viewport_height_ = 720;

    bool initialized_ = false;
};

} // namespace nexus::blink

#endif
