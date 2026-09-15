/*
 * Nexus Browser - Blink Rendering Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include "document.hpp"

namespace nexus::blink {

std::uint64_t Document::next_document_id_ = 1;

Document::Document()
    : document_id_(next_document_id_++)
{
}

Document::~Document()
{
    unload();
}

bool Document::load(const std::string& url)
{
    if (url.empty()) {
        return false;
    }

    url_ = url;
    loaded_ = true;

    return true;
}

bool Document::unload()
{
    loaded_ = false;
    url_.clear();
    html_.clear();

    return true;
}

bool Document::is_loaded() const
{
    return loaded_;
}

const std::string& Document::url() const
{
    return url_;
}

void Document::set_html(const std::string& html)
{
    html_ = html;
}

const std::string& Document::html() const
{
    return html_;
}

std::uint64_t Document::document_id() const
{
    return document_id_;
}

} // namespace nexus::blink
