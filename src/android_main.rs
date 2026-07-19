//! Android entry point — launched by NativeActivity.
//!
//! The Android system loads libhermes_operit_core.so and calls
//! `ANativeActivity_onCreate`, which ndk-glue forwards to this function.
//!
//! Uses JNI-driven WebView instead of dioxus-mobile for reliable
//! rendering on Android (dioxus-mobile 0.5.x Android support is experimental).

#[cfg(target_os = "android")]
use jni::objects::JObject;

#[cfg(target_os = "android")]
#[ndk_glue::main(backtrace = "on")]
fn android_main() {
    // Initialize Android logger so output goes to logcat
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("HermesOperit"),
    );

    log::info!("HermesOperit starting on Android (JNI WebView mode)...");

    // Get the NativeActivity JNI handles via ndk-glue.
    // native.vm() returns *mut *const JNIInvokeInterface_ (= jni::sys::JavaVM)
    // native.activity() returns *mut _jobject (= jni::sys::jobject)
    let native = ndk_glue::native_activity();

    // Attach to the JVM — vm_ptr is already the right type for JavaVM::from_raw
    let jvm = unsafe { jni::JavaVM::from_raw(native.vm()) }
        .expect("Failed to get JavaVM");
    let mut env = jvm
        .attach_current_thread()
        .expect("Failed to attach to JVM thread");

    // activity pointer is already *mut _jobject, wrap in JObject
    let activity = unsafe { JObject::from_raw(native.activity()) };

    log::info!("[Main] JVM attached, initializing WebView...");

    // Initialize WebView directly via JNI
    crate::android::webview::init_webview(&mut env, &activity);

    log::info!("HermesOperit WebView initialized");
}

// On non-Android targets, this file compiles to nothing.
#[cfg(not(target_os = "android"))]
fn main() {}
