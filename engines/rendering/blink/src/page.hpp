/*
 * Nexus Browser - Blink Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_BLINK_PAGE_HPP
#define NEXUS_BLINK_PAGE_HPP

#include <cstdint>
#include <memory>
#include <string>
#include <vector>

namespace nexus::blink {

class Frame;
class RenderingContext;

class Page {
public:
    explicit Page(const std::shared_ptr<RenderingContext>& context);
    ~Page();

    Page(const Page&) = delete;
    Page& operator=(const Page&) = delete;

    bool initialize();
    bool close();

    bool navigate(const std::string& url);
    bool reload();

    std::shared_ptr<Frame> main_frame() const;

    bool add_frame(const std::shared_ptr<Frame>& frame);
    bool remove_frame(const std::shared_ptr<Frame>& frame);

    std::size_t frame_count() const;

    std::uint64_t page_id() const;

    bool is_open() const;

private:
    std::uint64_t page_id_;

    std::shared_ptr<RenderingContext> rendering_context_;
    std::shared_ptr<Frame> main_frame_;

    std::vector<std::shared_ptr<Frame>> child_frames_;

    bool open_ = false;

    static std::uint64_t next_page_id_;
};

} // namespace nexus::blink

#endif
