//! Clipboard bridge — read/write the system clipboard.
//!
//! Real implementation uses Android's ClipboardManager via JNI calls.

use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jstring};
use jni::JNIEnv;

/// Get the current clipboard text.
///
/// Kotlin: `external fun nativeClipboardGet(): String`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeClipboardGet<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    log::info!("[Clipboard] Get requested (placeholder)");

    // Placeholder: real impl calls ClipboardManager.getPrimaryClip().
    env.new_string("")
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Set the clipboard text.
///
/// Kotlin: `external fun nativeClipboardSet(text: String): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeClipboardSet<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    text: JString<'local>,
) -> jboolean {
    let t: String = env.get_string(&text).map(|s| s.into()).unwrap_or_default();
    log::info!(
        "[Clipboard] Set ← \"{}\" (placeholder)",
        &t[..t.len().min(80)]
    );
    jni::sys::JNI_TRUE
}
