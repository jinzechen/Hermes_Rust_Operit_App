package com.operit.hermes.bridge

/**
 * JNI bridge to the native Rust library (libhermes_operit_core.so).
 *
 * All functions are declared as external and map to Rust functions
 * with the naming convention:
 *   Java_com_operit_hermes_bridge_HermesBridge_<methodName>
 *
 * The native pointer (ptr) is an opaque handle returned by nativeInit
 * and must be passed to all subsequent calls.
 */
object HermesBridge {

    // ──────────────────────────────────────────────
    //  Lifecycle
    // ──────────────────────────────────────────────

    /**
     * Initialize the native engine with a JSON configuration string.
     * Returns an opaque pointer (as Long) to the native state.
     */
    @JvmStatic
    external fun nativeInit(configJson: String): Long

    /**
     * Send a message to the native engine.
     * @param ptr    Opaque pointer from nativeInit
     * @param sessionId Session / channel identifier
     * @param message JSON-encoded message
     * @return JSON-encoded response
     */
    @JvmStatic
    external fun nativeSendMessage(ptr: Long, sessionId: String, message: String): String

    // ──────────────────────────────────────────────
    //  Shizuku input (via shell / service)
    // ──────────────────────────────────────────────

    /** Execute an arbitrary shell command via Shizuku. */
    @JvmStatic
    external fun nativeShizukuExec(ptr: Long, cmd: String): String

    /** Tap at screen coordinates (x, y). */
    @JvmStatic
    external fun nativeShizukuTap(ptr: Long, x: Float, y: Float): String

    /** Swipe from (x1,y1) to (x2,y2) over duration ms. */
    @JvmStatic
    external fun nativeShizukuSwipe(ptr: Long, x1: Float, y1: Float, x2: Float, y2: Float, duration: Long): String

    /** Type text (inject via input method). */
    @JvmStatic
    external fun nativeShizukuType(ptr: Long, text: String): String

    /** Inject a key event by Android key code. */
    @JvmStatic
    external fun nativeShizukuKeyEvent(ptr: Long, keyCode: Int): String

    /** Take a screenshot; returns base64-encoded PNG or file path. */
    @JvmStatic
    external fun nativeShizukuScreenshot(ptr: Long): String

    // ──────────────────────────────────────────────
    //  Accessibility service
    // ──────────────────────────────────────────────

    /** Get the full accessibility tree as JSON. */
    @JvmStatic
    external fun nativeAccessibilityGetTree(ptr: Long): String

    /** Find a node by resource-id and return its info as JSON. */
    @JvmStatic
    external fun nativeAccessibilityFindById(ptr: Long, resourceId: String): String

    /** Perform an accessibility click at coordinates (or on a node). */
    @JvmStatic
    external fun nativeAccessibilityClick(ptr: Long, x: Float, y: Float): String

    /** Set text on an editable accessibility node. */
    @JvmStatic
    external fun nativeAccessibilitySetText(ptr: Long, nodeId: String, text: String): String

    // ──────────────────────────────────────────────
    //  Foreground service
    // ──────────────────────────────────────────────

    /** Start the foreground service with a JSON notification config. */
    @JvmStatic
    external fun nativeForegroundStart(ptr: Long, configJson: String): String

    /** Update the foreground service notification. */
    @JvmStatic
    external fun nativeForegroundUpdate(ptr: Long, configJson: String): String

    /** Stop the foreground service. */
    @JvmStatic
    external fun nativeForegroundStop(ptr: Long): String

    // ──────────────────────────────────────────────
    //  Clipboard
    // ──────────────────────────────────────────────

    /** Get clipboard text. */
    @JvmStatic
    external fun nativeClipboardGet(ptr: Long): String

    /** Set clipboard text. */
    @JvmStatic
    external fun nativeClipboardSet(ptr: Long, text: String): String
}
