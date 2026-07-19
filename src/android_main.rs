//! Android entry point — sets callbacks, returns immediately.
//! The Android framework calls lifecycle callbacks (onStart, onResume,
//! onNativeWindowCreated) AFTER onCreate returns.

use jni::objects::JObject;

static mut WEBVIEW_CREATED: bool = false;
static mut JVM_PTR: *mut *const jni::sys::JNIInvokeInterface_ = std::ptr::null_mut();

static mut ACT_CBS: ndk_sys::ANativeActivityCallbacks = ndk_sys::ANativeActivityCallbacks {
    onStart: Some(on_started),
    onResume: Some(on_started),
    onSaveInstanceState: None,
    onPause: None,
    onStop: None,
    onDestroy: None,
    onWindowFocusChanged: None,
    onNativeWindowCreated: Some(on_window_created),
    onNativeWindowResized: None,
    onNativeWindowRedrawNeeded: None,
    onNativeWindowDestroyed: None,
    onInputQueueCreated: None,
    onInputQueueDestroyed: None,
    onContentRectChanged: None,
    onConfigurationChanged: None,
    onLowMemory: None,
};

#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate(
    activity: *mut std::ffi::c_void,
    _saved_state: *mut std::ffi::c_void,
    _saved_state_size: usize,
) {
    let act = unsafe { &mut *(activity as *mut ndk_sys::ANativeActivity) };

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("HermesOperit"),
    );

    log::info!("HermesOperit onCreate — hooking callbacks then returning");

    unsafe {
        JVM_PTR = act.vm;
        act.callbacks = &ACT_CBS as *const _ as *mut _;
    }

    // RETURN so the system can call onStart → onResume → onNativeWindowCreated
}

unsafe extern "C" fn on_started(_activity: *mut ndk_sys::ANativeActivity) {
    log::info!("onStart/onResume");
}

unsafe extern "C" fn on_window_created(
    activity: *mut ndk_sys::ANativeActivity,
    _window: *mut ndk_sys::ANativeWindow,
) {
    if WEBVIEW_CREATED {
        return;
    }
    WEBVIEW_CREATED = true;

    log::info!("onNativeWindowCreated! Creating WebView...");

    let jvm = jni::JavaVM::from_raw(JVM_PTR).unwrap();
    let mut env = jvm.get_env().unwrap();
    let activity_jobj = JObject::from_raw((*activity).clazz as jni::sys::jobject);

    crate::android::webview::init_webview_direct(&mut env, &activity_jobj);

    log::info!("WebView created!");
}

#[cfg(not(target_os = "android"))]
fn main() {}
