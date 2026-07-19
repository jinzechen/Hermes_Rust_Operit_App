//! Android entry point — NativeActivity onCreate handler.
//!
//! Exports ANativeActivity_onCreate directly (no ndk_glue::main wrapper)
//! so we run on the Android MAIN thread, which already has a Looper.
//! WebView MUST be created on a thread with a Looper (otherwise NPE crash).

use jni::objects::JObject;
use std::ffi::c_void;

/// ANativeActivity raw layout (from android/native_activity.h).
/// We only need the first 4 fields.
#[repr(C)]
struct NativeActivityRaw {
    callbacks: *mut c_void,
    vm: *mut *const jni::sys::JNIInvokeInterface_,
    _env: *mut c_void,
    clazz: jni::sys::jobject,
}

/// Called by the Android framework on the MAIN thread.
/// Creates the WebView directly — no thread-spawning wrapper.
#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate(
    activity: *mut c_void,
    _saved_state: *mut c_void,
    _saved_state_size: usize,
) {
    let act = unsafe { &*(activity as *const NativeActivityRaw) };

    // Get JavaVM from the ANativeActivity struct
    let jvm = unsafe { jni::JavaVM::from_raw(act.vm) }.expect("JavaVM");

    // Get JNI env already attached on the main thread
    let mut env = jvm.get_env().expect("JNIEnv on main thread");

    // Initialize logging
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("HermesOperit"),
    );

    log::info!("HermesOperit starting (main thread, ANativeActivity_onCreate)...");

    // Get the Activity JObject
    let activity_jobj = unsafe { JObject::from_raw(act.clazz) };

    // Create WebView directly on the main thread
    crate::android::webview::init_webview(&mut env, &activity_jobj);

    log::info!("HermesOperit WebView initialized on main thread");
}
