//! Android entry point — launched by NativeActivity.
//!
//! ndk_glue::main spawns a NEW thread for android_main(), so we must:
//! 1. attach_current_thread() to get a JNI env
//! 2. Looper.prepare() so WebView (which needs a Handler/Looper) can init
//! 3. Looper.loop() at the end to keep the thread alive processing events

#[cfg(target_os = "android")]
use jni::objects::JObject;

#[cfg(target_os = "android")]
#[ndk_glue::main(backtrace = "on")]
fn android_main() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("HermesOperit"),
    );

    log::info!("HermesOperit starting on Android (JNI WebView mode)...");

    let native = ndk_glue::native_activity();

    // attach_current_thread: ndk_glue spawns us on a worker thread,
    // so we must explicitly attach. Keep the guard alive for the whole
    // function so the thread stays attached.
    let jvm = unsafe { jni::JavaVM::from_raw(native.vm()) }
        .expect("Failed to get JavaVM");
    let mut env = jvm
        .attach_current_thread()
        .expect("Failed to attach to JVM");

    // WebView needs a Looper (Handler.init requires Looper.mQueue)
    log::info!("[Main] Preparing Looper for WebView thread...");
    env.call_static_method("android/os/Looper", "prepare", "()V", &[])
        .expect("Failed to prepare Looper");

    let activity = unsafe { JObject::from_raw(native.activity()) };

    log::info!("[Main] JVM attached + Looper ready, initializing WebView...");
    crate::android::webview::init_webview(&mut env, &activity);
    log::info!("HermesOperit WebView initialized, entering event loop");

    // Run the Looper — blocks forever, processing Android UI events
    env.call_static_method("android/os/Looper", "loop", "()V", &[])
        .expect("Looper.loop failed");
}

#[cfg(not(target_os = "android"))]
fn main() {}
