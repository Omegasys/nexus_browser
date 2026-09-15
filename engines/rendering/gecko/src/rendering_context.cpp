/*
 * Nexus Browser - Gecko Rendering Context
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include "rendering_context.hpp"

namespace nexus::gecko {

RenderingContext::RenderingContext(
    const EngineConfiguration& configuration
)
    : configuration_(configuration)
{
}

RenderingContext::~RenderingContext()
{
    shutdown();
}

bool RenderingContext::initialize()
{
    if (initialized_) {
        return true;
    }

    initialized_ = true;

    return true;
}

bool RenderingContext::shutdown()
{
    initialized_ = false;

    return true;
}

bool RenderingContext::render()
{
    if (!initialized_) {
        return false;
    }

    ++metrics_.frames_rendered;

    return true;
}

bool RenderingContext::execute_javascript(
    const std::string& script
)
{
    if (!initialized_) {
        return false;
    }

    if (!configuration_.javascript_enabled) {
        return false;
    }

    if (script.empty()) {
        return true;
    }

    ++metrics_.scripts_executed;

    return true;
}

bool RenderingContext::set_viewport(
    std::uint32_t width,
    std::uint32_t height
)
{
    if (width == 0 || height == 0) {
        return false;
    }

    viewport_width_ = width;
    viewport_height_ = height;

    return true;
}

std::uint32_t
RenderingContext::viewport_width() const
{
    return viewport_width_;
}

std::uint32_t
RenderingContext::viewport_height() const
{
    return viewport_height_;
}

bool RenderingContext::is_initialized() const
{
    return initialized_;
}

const RenderingMetrics&
RenderingContext::metrics() const
{
    return metrics_;
}

} // namespace nexus::gecko
