//! Android entry point — NativeActivity via ndk_glue::main.
//!
//! ndk_glue handles the NativeActivity lifecycle and spawns a worker thread.
//! We attach JNI, set the HTML, and delegate WebView creation to the UI
//! thread via Java's WebViewHelper.createOnUiThread().
//! The worker thread then blocks waiting for native activity events.

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

    let activity = unsafe { JObject::from_raw(native.activity()) };

    log::info!("[Main] Delegating WebView to UI thread via Java helper...");
    crate::android::webview::init_webview(&mut env, &activity);
    log::info!("HermesOperit done — worker thread idle. UI thread handles WebView.");
}

#[cfg(not(target_os = "android"))]
fn main() {}
