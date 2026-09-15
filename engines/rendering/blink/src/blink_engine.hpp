/*
 * Nexus Browser - Blink Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_BLINK_ENGINE_HPP
#define NEXUS_BLINK_ENGINE_HPP

#include <cstdint>
#include <memory>
#include <string>

namespace nexus::blink {

class RenderingContext;
class Page;

struct EngineConfiguration {
    bool javascript_enabled = true;
    bool images_enabled = true;
    bool plugins_enabled = false;
    bool webgl_enabled = false;
    bool hardware_acceleration = false;
    bool private_mode = true;
    bool network_access = false;
    bool filesystem_access = false;
};

struct EngineStatus {
    bool initialized = false;
    bool running = false;
    bool healthy = false;
    std::uint64_t pages_created = 0;
    std::uint64_t frames_created = 0;
};

class BlinkEngine {
public:
    BlinkEngine();
    ~BlinkEngine();

    BlinkEngine(const BlinkEngine&) = delete;
    BlinkEngine& operator=(const BlinkEngine&) = delete;

    bool initialize(const EngineConfiguration& configuration);
    bool shutdown();

    bool is_initialized() const;
    bool is_running() const;

    const EngineStatus& status() const;
    const EngineConfiguration& configuration() const;

    std::shared_ptr<RenderingContext> create_rendering_context();
    std::shared_ptr<Page> create_page();

    bool destroy_page(const std::shared_ptr<Page>& page);

    bool reset();

private:
    EngineConfiguration configuration_;
    EngineStatus status_;
};

} // namespace nexus::blink

#endif
