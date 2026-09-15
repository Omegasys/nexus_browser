/*
 * Nexus Browser - Gecko Public Rendering Context
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_GECKO_PUBLIC_RENDERING_CONTEXT_HPP
#define NEXUS_GECKO_PUBLIC_RENDERING_CONTEXT_HPP

#include <cstdint>

#include "rendering_types.hpp"

namespace nexus::gecko {

class RenderingContext {
public:
    RenderingContext();
    ~RenderingContext();

    RenderingContext(const RenderingContext&) = delete;
    RenderingContext& operator=(
        const RenderingContext&
    ) = delete;

    bool initialize(
        const EngineConfiguration& configuration
    );

    bool shutdown();

    bool render();

    bool execute_javascript(
        const char* script
    );

    bool resize(
        std::uint32_t width,
        std::uint32_t height
    );

    ViewportSize viewport() const;

    RenderingMetrics metrics() const;

    bool is_initialized() const;

private:
    class Impl;
    Impl* implementation_;
};

} // namespace nexus::gecko

#endif
