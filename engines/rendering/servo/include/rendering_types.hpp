/*
 * Nexus Browser - Servo Rendering Types
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#ifndef NEXUS_SERVO_RENDERING_TYPES_HPP
#define NEXUS_SERVO_RENDERING_TYPES_HPP

#include <cstdint>

namespace nexus::servo {

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

    bool wasm_enabled = true;

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

} // namespace nexus::servo

#endif
