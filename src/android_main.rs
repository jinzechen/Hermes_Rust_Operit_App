//! Android entry point — main thread with full lifecycle callbacks.
//! Creates WebView after window surface is ready.

use jni::objects::JObject;
use std::ffi::c_void;

#[repr(C)]
struct NativeActivityRaw {
    callbacks: *mut c_void,
    vm: *mut *const jni::sys::JNIInvokeInterface_,
    _env: *mut c_void,
    clazz: jni::sys::jobject,
}

// Callback type signatures
type VoidCb = unsafe extern "C" fn(activity: *mut c_void);

#[repr(C)]
struct ActivityCallbacks {
    onStart: Option<VoidCb>,
    onResume: Option<VoidCb>,
    onSaveInstanceState: Option<VoidCb>,
    onPause: Option<VoidCb>,
    onStop: Option<VoidCb>,
    onDestroy: Option<VoidCb>,
    onWindowFocusChanged: Option<unsafe extern "C" fn(*mut c_void, i32)>,
    onNativeWindowCreated: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    onNativeWindowResized: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    onNativeWindowRedrawNeeded: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    onNativeWindowDestroyed: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    onInputQueueCreated: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    onInputQueueDestroyed: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    onContentRectChanged: Option<unsafe extern "C" fn(*mut c_void, *const ndk_sys::ARect)>,
    onConfigurationChanged: Option<VoidCb>,
    onLowMemory: Option<VoidCb>,
}

static mut ACT_CBS: ActivityCallbacks = ActivityCallbacks {
    onStart: Some(on_start_resume),
    onResume: Some(on_start_resume),
    onSaveInstanceState: None,
    onPause: None,
    onStop: None,
    onDestroy: None,
    onWindowFocusChanged: None,
    onNativeWindowCreated: Some(on_native_window_created),
    onNativeWindowResized: None,
    onNativeWindowRedrawNeeded: None,
    onNativeWindowDestroyed: None,
    onInputQueueCreated: None,
    onInputQueueDestroyed: None,
    onContentRectChanged: None,
    onConfigurationChanged: None,
    onLowMemory: None,
};

static mut WEBVIEW_CREATED: bool = false;
static mut JVM_PTR: *mut *const jni::sys::JNIInvokeInterface_ = std::ptr::null_mut();

#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate(
    activity: *mut c_void,
    _saved_state: *mut c_void,
    _saved_state_size: usize,
) {
    let act = unsafe { &mut *(activity as *mut NativeActivityRaw) };

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("HermesOperit"),
    );

    log::info!("HermesOperit onCreate (MAIN thread)");

    // Set our callbacks BEFORE the system calls them
    unsafe {
        JVM_PTR = act.vm;
        act.callbacks = &ACT_CBS as *const _ as *mut c_void;
    }

    // Event loop
    loop {
        let mut fdesc: i32 = 0;
        let mut events: i32 = 0;
        let mut data: *mut c_void = std::ptr::null_mut();
        unsafe {
            ndk_sys::ALooper_pollAll(-1, &mut fdesc, &mut events, &mut data);
        }
    }
}

unsafe extern "C" fn on_start_resume(_activity: *mut c_void) {
    log::info!("Lifecycle: onStart/onResume");
}

unsafe extern "C" fn on_native_window_created(activity: *mut c_void, _window: *mut c_void) {
    if WEBVIEW_CREATED { return; }
    WEBVIEW_CREATED = true;

    log::info!("NativeWindow created — creating WebView...");

    let jvm = jni::JavaVM::from_raw(JVM_PTR).unwrap();
    let mut env = jvm.get_env().unwrap();
    let activity_jobj = JObject::from_raw(activity as jni::sys::jobject);

    crate::android::webview::init_webview_direct(&mut env, &activity_jobj);

    log::info!("WebView created via onNativeWindowCreated callback!");
}

#[cfg(not(target_os = "android"))]
fn main() {}
