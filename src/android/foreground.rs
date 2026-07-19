//! Foreground service bridge — keeps the agent alive in the background.
//!
//! Android 8+ requires foreground services to show a persistent notification.
//! These stubs manage the service lifecycle from native code.

use jni::objects::{JClass, JString};
use jni::sys::jboolean;
use jni::JNIEnv;

/// Start the foreground service with a persistent notification.
///
/// Kotlin: `external fun nativeForegroundStart(): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeForegroundStart<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jboolean {
    log::info!("[Foreground] Service start requested");
    // Placeholder: bind/start HermesForegroundService.
    jni::sys::JNI_TRUE
}

/// Update the notification title and text shown in the status bar.
///
/// Kotlin: `external fun nativeForegroundUpdate(title: String, text: String): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeForegroundUpdate<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    title: JString<'local>,
    text: JString<'local>,
) -> jboolean {
    let t: String = env.get_string(&title).map(|s| s.into()).unwrap_or_default();
    let tx: String = env.get_string(&text).map(|s| s.into()).unwrap_or_default();
    log::info!("[Foreground] Update title=\"{t}\" text=\"{tx}\"");
    jni::sys::JNI_TRUE
}

/// Stop the foreground service and remove the notification.
///
/// Kotlin: `external fun nativeForegroundStop(): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeForegroundStop<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jboolean {
    log::info!("[Foreground] Service stop requested");
    jni::sys::JNI_TRUE
}
