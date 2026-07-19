//! Accessibility service bridge — inspecting & manipulating the UI tree.
//!
//! On Android these operations require an active AccessibilityService.
//! The stubs below would communicate with that service in a real build.

use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jstring};
use jni::JNIEnv;

/// Return the full accessibility tree as an XML string.
///
/// Kotlin: `external fun nativeAccessibilityGetTree(): String`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeAccessibilityGetTree<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    log::info!("[A11y] GetTree requested");

    // Placeholder: real impl calls AccessibilityService.getWindows() and
    // serialises each AccessibilityNodeInfo tree.
    let xml = r#"<?xml version="1.0"?><hierarchy><node text="placeholder"/></hierarchy>"#;
    env.new_string(xml)
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Locate a node by its resource-id and return its JSON representation.
///
/// Kotlin: `external fun nativeAccessibilityFindById(resourceId: String): String`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeAccessibilityFindById<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    resource_id: JString<'local>,
) -> jstring {
    let rid: String = env
        .get_string(&resource_id)
        .map(|s| s.into())
        .unwrap_or_default();
    log::info!("[A11y] FindById \"{rid}\"");

    let json = format!(
        r#"{{"resourceId":"{rid}","className":"android.widget.TextView","text":"placeholder","bounds":"0,0-100,100"}}"#
    );
    env.new_string(json)
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Perform a click on the node identified by resource-id.
///
/// Kotlin: `external fun nativeAccessibilityClick(resourceId: String): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeAccessibilityClick<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    resource_id: JString<'local>,
) -> jboolean {
    let rid: String = env
        .get_string(&resource_id)
        .map(|s| s.into())
        .unwrap_or_default();
    log::info!("[A11y] Click \"{rid}\"");
    jni::sys::JNI_TRUE
}

/// Set text on an editable node identified by resource-id.
///
/// Kotlin: `external fun nativeAccessibilitySetText(resourceId: String, text: String): Boolean`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeAccessibilitySetText<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    resource_id: JString<'local>,
    text: JString<'local>,
) -> jboolean {
    let rid: String = env
        .get_string(&resource_id)
        .map(|s| s.into())
        .unwrap_or_default();
    let txt: String = env.get_string(&text).map(|s| s.into()).unwrap_or_default();
    log::info!("[A11y] SetText \"{rid}\" ← \"{txt}\"");
    jni::sys::JNI_TRUE
}
