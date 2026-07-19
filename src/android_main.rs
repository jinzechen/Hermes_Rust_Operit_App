//! Android entry point — runs on MAIN thread.
//! WebView MUST be created on main thread (w/ proper Looper).

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

    log::info!("HermesOperit starting (MAIN thread)");

    let jvm = unsafe { jni::JavaVM::from_raw(act.vm) }.expect("JavaVM");
    let mut env = jvm.get_env().expect("JNIEnv");
    let activity_jobj = unsafe { JObject::from_raw(act.clazz) };

    crate::android::webview::init_webview_direct(&mut env, &activity_jobj);

    log::info!("HermesOperit ready on main thread");
}

#[cfg(not(target_os = "android"))]
fn main() {}
