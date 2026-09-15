/*
 * Nexus Browser - Gecko Rendering Types
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_GECKO_RENDERING_TYPES_HPP
#define NEXUS_GECKO_RENDERING_TYPES_HPP

#include <cstdint>

namespace nexus::gecko {

enum class EngineState {
    Uninitialized,
    Initializing,
    Running,
    ShuttingDown,
    Stopped,
    Failed
};

enum class TrustLevel {
    Untrusted,
    Staged,
    Testing,
    Limited,
    Approved,
    Trusted
};

enum class RenderingMode {
    Software,
    HardwareAccelerated,
    VirtualGPU
};

enum class JavaScriptMode {
    Disabled,
    Sandboxed,
    Enabled
};

enum class NetworkAccess {
    Disabled,
    Restricted,
    Enabled
};

enum class StorageMode {
    Temporary,
    Persistent,
    Encrypted,
    Disposable
};

struct ViewportSize {
    std::uint32_t width = 1280;
    std::uint32_t height = 720;
};

struct EngineConfiguration {
    bool javascript_enabled = true;
    bool images_enabled = true;
    bool plugins_enabled = false;
    bool webgl_enabled = false;
    bool private_mode = true;

    RenderingMode rendering_mode =
        RenderingMode::Software;

    JavaScriptMode javascript_mode =
        JavaScriptMode::Sandboxed;

    NetworkAccess network_access =
        NetworkAccess::Disabled;

    StorageMode storage_mode =
        StorageMode::Temporary;

    TrustLevel trust_level =
        TrustLevel::Untrusted;
};

struct RenderingMetrics {
    std::uint64_t frames_rendered = 0;
    std::uint64_t scripts_executed = 0;
    std::uint64_t bytes_rendered = 0;
};

struct EngineStatus {
    EngineState state =
        EngineState::Uninitialized;

    TrustLevel trust_level =
        TrustLevel::Untrusted;

    bool initialized = false;
    bool running = false;
    bool healthy = false;

    std::uint64_t pages_created = 0;
    std::uint64_t frames_created = 0;
    std::uint64_t crashes = 0;
};

struct PageInfo {
    std::uint64_t page_id = 0;
    std::uint64_t main_frame_id = 0;

    bool active = false;
    bool loading = false;
};

struct FrameInfo {
    std::uint64_t frame_id = 0;
    std::uint64_t document_id = 0;

    bool active = false;
};

} // namespace nexus::gecko

#endif
