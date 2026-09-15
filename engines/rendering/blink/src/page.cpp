/*
 * Nexus Browser - Blink Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include "page.hpp"

#include <algorithm>

#include "frame.hpp"
#include "rendering_context.hpp"

namespace nexus::blink {

std::uint64_t Page::next_page_id_ = 1;

Page::Page(const std::shared_ptr<RenderingContext>& context)
    : page_id_(next_page_id_++)
    , rendering_context_(context)
{
    initialize();
}

Page::~Page()
{
    close();
}

bool Page::initialize()
{
    if (open_) {
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

    main_frame_ = std::make_shared<Frame>(rendering_context_);

    if (!main_frame_->initialize()) {
        main_frame_.reset();
        return false;
    }

    open_ = true;

    return true;
}

bool Page::close()
{
    if (!open_) {
        return true;
    }

    for (auto& frame : child_frames_) {
        if (frame) {
            frame->close();
        }
    }

    child_frames_.clear();

    if (main_frame_) {
        main_frame_->close();
        main_frame_.reset();
    }

    open_ = false;

    return true;
}

bool Page::navigate(const std::string& url)
{
    if (!open_ || !main_frame_) {
        return false;
    }

    return main_frame_->navigate(url);
}

bool Page::reload()
{
    if (!open_ || !main_frame_) {
        return false;
    }

    return main_frame_->reload();
}

std::shared_ptr<Frame> Page::main_frame() const
{
    return main_frame_;
}

bool Page::add_frame(const std::shared_ptr<Frame>& frame)
{
    if (!open_ || !frame) {
        return false;
    }

    if (!frame->is_active()) {
        if (!frame->initialize()) {
            return false;
        }
    }

    child_frames_.push_back(frame);

    return true;
}

bool Page::remove_frame(const std::shared_ptr<Frame>& frame)
{
    if (!frame) {
        return false;
    }

    auto iterator = std::find(
        child_frames_.begin(),
        child_frames_.end(),
        frame
    );

    if (iterator == child_frames_.end()) {
        return false;
    }

    (*iterator)->close();
    child_frames_.erase(iterator);

    return true;
}

std::size_t Page::frame_count() const
{
    if (!open_) {
        return 0;
    }

    return 1 + child_frames_.size();
}

std::uint64_t Page::page_id() const
{
    return page_id_;
}

bool Page::is_open() const
{
    return open_;
}

} // namespace nexus::blink
