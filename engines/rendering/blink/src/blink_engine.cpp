/*
 * Nexus Browser - Blink Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include "blink_engine.hpp"

#include "page.hpp"
#include "rendering_context.hpp"

namespace nexus::blink {

BlinkEngine::BlinkEngine() = default;

BlinkEngine::~BlinkEngine()
{
    shutdown();
}

bool BlinkEngine::initialize(const EngineConfiguration& configuration)
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

bool BlinkEngine::shutdown()
{
    if (!status_.initialized) {
        return true;
    }

    status_.running = false;
    status_.healthy = false;
    status_.initialized = false;

    return true;
}

bool BlinkEngine::is_initialized() const
{
    return status_.initialized;
}

bool BlinkEngine::is_running() const
{
    return status_.running;
}

const EngineStatus& BlinkEngine::status() const
{
    return status_;
}

const EngineConfiguration& BlinkEngine::configuration() const
{
    return configuration_;
}

std::shared_ptr<RenderingContext>
BlinkEngine::create_rendering_context()
{
    if (!status_.running) {
        return nullptr;
    }

    auto context = std::make_shared<RenderingContext>(configuration_);

    ++status_.frames_created;

    return context;
}

std::shared_ptr<Page> BlinkEngine::create_page()
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

bool BlinkEngine::destroy_page(const std::shared_ptr<Page>& page)
{
    if (!page) {
        return false;
    }

    page->close();

    return true;
}

bool BlinkEngine::reset()
{
    if (!status_.initialized) {
        return false;
    }

    shutdown();

    EngineConfiguration previous_configuration = configuration_;

    return initialize(previous_configuration);
}

} // namespace nexus::blink
