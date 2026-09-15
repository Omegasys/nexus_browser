/*
 * Nexus Browser - Blink Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include "frame.hpp"

#include "document.hpp"
#include "rendering_context.hpp"

namespace nexus::blink {

std::uint64_t Frame::next_frame_id_ = 1;

Frame::Frame(const std::shared_ptr<RenderingContext>& context)
    : frame_id_(next_frame_id_++)
    , rendering_context_(context)
{
}

Frame::~Frame()
{
    close();
}

bool Frame::initialize()
{
    if (active_) {
        return true;
    }

    if (!rendering_context_) {
        return false;
    }

    if (!rendering_context_->is_initialized()) {
        if (!rendering_context_->initialize()) {
            return false;
        }
    }

    document_ = std::make_shared<Document>();
    active_ = true;

    return true;
}

bool Frame::close()
{
    if (!active_) {
        return true;
    }

    if (document_) {
        document_->unload();
        document_.reset();
    }

    active_ = false;

    return true;
}

bool Frame::navigate(const std::string& url)
{
    if (!active_) {
        return false;
    }

    if (!document_) {
        return false;
    }

    return document_->load(url);
}

bool Frame::reload()
{
    if (!active_ || !document_) {
        return false;
    }

    const std::string current_url = document_->url();

    if (current_url.empty()) {
        return false;
    }

    return document_->load(current_url);
}

std::shared_ptr<Document> Frame::document() const
{
    return document_;
}

std::shared_ptr<RenderingContext> Frame::rendering_context() const
{
    return rendering_context_;
}

std::uint64_t Frame::frame_id() const
{
    return frame_id_;
}

bool Frame::is_active() const
{
    return active_;
}

} // namespace nexus::blink
