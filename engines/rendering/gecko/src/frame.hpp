/*
 * Nexus Browser - Gecko Frame
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_GECKO_FRAME_HPP
#define NEXUS_GECKO_FRAME_HPP

#include <cstdint>
#include <memory>
#include <string>

namespace nexus::gecko {

class Document;
class RenderingContext;

class Frame {
public:
    explicit Frame(
        const std::shared_ptr<RenderingContext>& context
    );

    ~Frame();

    Frame(const Frame&) = delete;
    Frame& operator=(const Frame&) = delete;

    bool initialize();
    bool close();

    bool navigate(const std::string& url);
    bool reload();

    std::shared_ptr<Document> document() const;

    std::shared_ptr<RenderingContext>
    rendering_context() const;

    std::uint64_t frame_id() const;

    bool is_active() const;

private:
    static std::uint64_t next_frame_id_;

    std::uint64_t frame_id_;

    std::shared_ptr<RenderingContext>
        rendering_context_;

    std::shared_ptr<Document>
        document_;

    bool active_ = false;
};

} // namespace nexus::gecko

#endif
