//! Android entry point — main thread, manual ANativeActivity_onCreate.
//! Creates WebView after the first ALooper poll (window surface ready).

use jni::objects::JObject;
use std::ffi::c_void;

#[repr(C)]
struct NativeActivityRaw {
    callbacks: *mut c_void,
    vm: *mut *const jni::sys::JNIInvokeInterface_,
    _env: *mut c_void,
    clazz: jni::sys::jobject,
}

#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate(
    activity: *mut c_void,
    _saved_state: *mut c_void,
    _saved_state_size: usize,
) {
    let act = unsafe { &*(activity as *const NativeActivityRaw) };

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("HermesOperit"),
    );

    log::info!("HermesOperit onCreate (MAIN thread)");

    let jvm = unsafe { jni::JavaVM::from_raw(act.vm) }.expect("JavaVM");
    let mut env = jvm.get_env().expect("JNIEnv");
    let activity_jobj = unsafe { JObject::from_raw(act.clazz) };

    let mut created = false;

    // Event loop — wait for first frame, then create WebView
    loop {
        let mut fdesc: i32 = 0;
        let mut events: i32 = 0;
        let mut data: *mut c_void = std::ptr::null_mut();
        unsafe {
            ndk_sys::ALooper_pollAll(100, &mut fdesc, &mut events, &mut data);
        }

        if !created {
            log::info!("First poll done, creating WebView...");
            crate::android::webview::init_webview_direct(&mut env, &activity_jobj);
            created = true;
            log::info!("WebView created, continuing event loop");
        }
    }
}

#[cfg(not(target_os = "android"))]
fn main() {}
