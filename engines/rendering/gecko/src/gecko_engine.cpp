/*
 * Nexus Browser - Gecko Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include "gecko_engine.hpp"

#include "page.hpp"
#include "rendering_context.hpp"

namespace nexus::gecko {

GeckoEngine::GeckoEngine() = default;

GeckoEngine::~GeckoEngine()
{
    shutdown();
}

bool GeckoEngine::initialize(
    const EngineConfiguration& configuration
)
{
    if (status_.initialized) {
        return true;
    }

    configuration_ = configuration;

    status_.initialized = true;
    status_.running = true;
    status_.healthy = true;

    return true;
}

bool GeckoEngine::shutdown()
{
    if (!status_.initialized) {
        return true;
    }

    status_.running = false;
    status_.healthy = false;
    status_.initialized = false;

    return true;
}

bool GeckoEngine::is_initialized() const
{
    return status_.initialized;
}

bool GeckoEngine::is_running() const
{
    return status_.running;
}

const EngineStatus& GeckoEngine::status() const
{
    return status_;
}

const EngineConfiguration&
GeckoEngine::configuration() const
{
    return configuration_;
}

std::shared_ptr<RenderingContext>
GeckoEngine::create_rendering_context()
{
    if (!status_.running) {
        return nullptr;
    }

    auto context =
        std::make_shared<RenderingContext>(configuration_);

    ++status_.frames_created;

    return context;
}

std::shared_ptr<Page>
GeckoEngine::create_page()
{
    if (!status_.running) {
        return nullptr;
    }

    auto context = create_rendering_context();

    if (!context) {
        return nullptr;
    }

    auto page = std::make_shared<Page>(context);

    ++status_.pages_created;

    return page;
}

bool GeckoEngine::destroy_page(
    const std::shared_ptr<Page>& page
)
{
    if (!page) {
        return false;
    }

    page->close();

    return true;
}

bool GeckoEngine::reset()
{
    if (!status_.initialized) {
        return false;
    }

    EngineConfiguration configuration = configuration_;

    shutdown();

    return initialize(configuration);
}

} // namespace nexus::gecko
