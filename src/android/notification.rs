//! Notification management bridge (placeholder).
//!
//! Real implementation will interact with Android's NotificationManager
//! to post notifications from native code.

use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jint};
use jni::JNIEnv;

/// Show a notification.
///
/// Kotlin: `external fun nativeNotificationShow(title: String, message: String): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeNotificationShow<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    title: JString<'local>,
    message: JString<'local>,
) -> jboolean {
    let t: String = env.get_string(&title).map(|s| s.into()).unwrap_or_default();
    let m: String = env
        .get_string(&message)
        .map(|s| s.into())
        .unwrap_or_default();
    log::info!("[Notification] Show title=\"{t}\" msg=\"{m}\" (placeholder)");
    jni::sys::JNI_TRUE
}

/// Dismiss a notification by ID.
///
/// Kotlin: `external fun nativeNotificationDismiss(id: Int): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeNotificationDismiss<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jint,
) -> jboolean {
    log::info!("[Notification] Dismiss id={id} (placeholder)");
    jni::sys::JNI_TRUE
}
