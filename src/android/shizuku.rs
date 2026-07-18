//! Shizuku bridge — system-level UI automation on Android.
//!
//! Shizuku runs as a privileged shell so it can inject tap/swipe/key events
//! and capture screenshots without root.  Each function here is a stub that
//! would call into Shizuku's UserService AIDL in a real build.

use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jbyteArray, jint, jstring};
use jni::JNIEnv;

/// Execute an arbitrary shell command via Shizuku.
///
/// Kotlin: `external fun nativeShizukuExec(command: String): String`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeShizukuExec<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    command: JString<'local>,
) -> jstring {
    let cmd: String = env.get_string(&command).map(|s| s.into()).unwrap_or_default();
    log::info!("[Shizuku] Exec: \"{cmd}\"");

    let output = format!("[placeholder] exec result for: {cmd}");
    env.new_string(output)
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Tap at screen coordinates (x, y).
///
/// Kotlin: `external fun nativeShizukuTap(x: Int, y: Int): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeShizukuTap<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    x: jint,
    y: jint,
) -> jboolean {
    log::info!("[Shizuku] Tap ({x}, {y})");
    // Placeholder: always succeeds.
    jni::sys::JNI_TRUE
}

/// Swipe from (x1,y1) to (x2,y2) over a given duration in ms.
///
/// Kotlin: `external fun nativeShizukuSwipe(x1: Int, y1: Int, x2: Int, y2: Int, durationMs: Int): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeShizukuSwipe<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    x1: jint,
    y1: jint,
    x2: jint,
    y2: jint,
    duration: jint,
) -> jboolean {
    log::info!("[Shizuku] Swipe ({x1},{y1})→({x2},{y2}) dur={duration}ms");
    jni::sys::JNI_TRUE
}

/// Type a text string (inject via input method).
///
/// Kotlin: `external fun nativeShizukuType(text: String): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeShizukuType<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    text: JString<'local>,
) -> jboolean {
    let t: String = env.get_string(&text).map(|s| s.into()).unwrap_or_default();
    log::info!("[Shizuku] Type: \"{t}\"");
    jni::sys::JNI_TRUE
}

/// Send a key event by Android keycode.
///
/// Kotlin: `external fun nativeShizukuKeyEvent(keycode: Int): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeShizukuKeyEvent<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    keycode: jint,
) -> jboolean {
    log::info!("[Shizuku] KeyEvent keycode={keycode}");
    jni::sys::JNI_TRUE
}

/// Capture a screenshot and return the raw PNG bytes.
///
/// Kotlin: `external fun nativeShizukuScreenshot(): ByteArray?`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeShizukuScreenshot<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jbyteArray {
    log::info!("[Shizuku] Screenshot requested (placeholder)");

    // Return a tiny valid PNG so callers don't crash on null.
    // (Empty byte array — real impl returns the captured buffer.)
    let png: [u8; 0] = [];
    env.byte_array_from_slice(&png)
        .map(|arr| arr.into_raw())
        .unwrap_or(std::ptr::null_mut())
}
