//! Android entry point — NativeActivity onCreate handler.
//!
//! Exports ANativeActivity_onCreate directly (no ndk_glue::main wrapper)
//! so we run on the Android MAIN thread, which already has a Looper.
//! WebView MUST be created on a thread with a Looper (otherwise NPE crash).

use jni::objects::JObject;
use std::ffi::c_void;

/// Called by the Android framework on the MAIN thread.
/// Creates the WebView directly — no thread-spawning wrapper.
#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate(
    activity: *mut c_void,
    _saved_state: *mut c_void,
    _saved_state_size: usize,
) {
    // activity is an ANativeActivity* — we need to extract the JavaVM
    // and the JNI environment from it.
    // ANativeActivity layout: clazz, vm, env, ...
    // vm is at offset after ANativeActivity_callbacks (callbacks + instance data etc.)
    // Actually, we use the ndk::ffi types to access it properly.

    let activity_ref = unsafe { &*(activity as *const ndk::ffi::ANativeActivity) };

    // Get JavaVM pointer from the activity
    let vm_ptr = activity_ref.vm as *mut *const jni::sys::JNIInvokeInterface_;
    let jvm = unsafe { jni::JavaVM::from_raw(vm_ptr) }.expect("JavaVM");

    // Get the JNI environment that's already attached on the main thread
    let mut env = jvm.get_env().expect("JNIEnv on main thread");

    // Initialize logging
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("HermesOperit"),
    );

    log::info!("HermesOperit starting (main thread, ANativeActivity_onCreate)...");

    // Get the Activity JObject from the ANativeActivity's clazz field
    let activity_jobj = unsafe { JObject::from_raw(activity_ref.clazz as jni::sys::jobject) };

    // Create WebView directly on the main thread (has Looper!)
    crate::android::webview::init_webview(&mut env, &activity_jobj);

    log::info!("HermesOperit WebView initialized on main thread");
    // The native activity event loop continues after this function returns.
}
