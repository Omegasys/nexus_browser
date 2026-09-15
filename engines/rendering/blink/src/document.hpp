/*
 * Nexus Browser - Blink Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_BLINK_DOCUMENT_HPP
#define NEXUS_BLINK_DOCUMENT_HPP

#include <cstdint>
#include <string>

namespace nexus::blink {

class Document {
public:
    Document();
    ~Document();

    Document(const Document&) = delete;
    Document& operator=(const Document&) = delete;

    bool load(const std::string& url);
    bool unload();

    bool is_loaded() const;

    const std::string& url() const;

    void set_html(const std::string& html);
    const std::string& html() const;

    std::uint64_t document_id() const;

private:
    std::uint64_t document_id_;
    std::string url_;
    std::string html_;
    bool loaded_ = false;

    static std::uint64_t next_document_id_;
};

} // namespace nexus::blink

#endif
