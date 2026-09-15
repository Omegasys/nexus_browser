/*
 * Nexus Browser - Gecko Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_GECKO_ENGINE_HPP
#define NEXUS_GECKO_ENGINE_HPP

#include <cstdint>
#include <memory>
#include <string>

namespace nexus::gecko {

class Page;
class RenderingContext;

struct EngineConfiguration {
    bool javascript_enabled = true;
    bool images_enabled = true;
    bool plugins_enabled = false;
    bool webgl_enabled = false;
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

class GeckoEngine {
public:
    GeckoEngine();
    ~GeckoEngine();

    GeckoEngine(const GeckoEngine&) = delete;
    GeckoEngine& operator=(const GeckoEngine&) = delete;

    bool initialize(const EngineConfiguration& configuration);
    bool shutdown();

    bool is_initialized() const;
    bool is_running() const;

    const EngineStatus& status() const;
    const EngineConfiguration& configuration() const;

    std::shared_ptr<RenderingContext>
    create_rendering_context();

    std::shared_ptr<Page>
    create_page();

    bool destroy_page(
        const std::shared_ptr<Page>& page
    );

    bool reset();

private:
    EngineConfiguration configuration_;
    EngineStatus status_;
};

} // namespace nexus::gecko

#endif
