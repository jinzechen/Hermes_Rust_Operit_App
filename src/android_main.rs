//! Android entry point — NativeActivity via ndk_glue::main.
//!
//! ndk_glue handles the NativeActivity lifecycle (callbacks, ALooper)
//! and spawns a worker thread for us. We prepare a Looper on this thread
//! so WebView can initialize (it needs Handler/Looper).

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

    log::info!("HermesOperit starting (ndk_glue worker thread)...");

    let native = ndk_glue::native_activity();
    let jvm = unsafe { jni::JavaVM::from_raw(native.vm()) }.expect("JavaVM");
    let mut env = jvm.attach_current_thread().expect("JNI attach");

    // WebView requires a Looper on the creating thread
    log::info!("[Main] Preparing Looper...");
    env.call_static_method("android/os/Looper", "prepare", "()V", &[])
        .expect("Looper.prepare");

    let activity = unsafe { JObject::from_raw(native.activity()) };

    log::info!("[Main] Creating WebView...");
    crate::android::webview::init_webview(&mut env, &activity);
    log::info!("HermesOperit WebView ready, entering Looper.loop()...");

    // Process events on our thread's Looper (blocks forever)
    env.call_static_method("android/os/Looper", "loop", "()V", &[])
        .expect("Looper.loop");
}

#[cfg(not(target_os = "android"))]
fn main() {}
