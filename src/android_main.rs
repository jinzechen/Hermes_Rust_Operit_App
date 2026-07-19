//! Android entry point — runs on MAIN thread.
//! Creates WebView then runs ALooper event loop so window can render.

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

    log::info!("WebView created. Running event loop...");

    // Run ALooper event loop so the window actually renders.
    // Without this, the surface is never composited → black screen.
    loop {
        let mut fdesc: i32 = 0;
        let mut events: i32 = 0;
        let mut data: *mut c_void = std::ptr::null_mut();
        unsafe {
            ndk_sys::ALooper_pollAll(-1, &mut fdesc, &mut events, &mut data);
        }
        if fdesc >= 0 && events != 0 {
            // Event ready on fd — would handle in full impl
        }
        // Process Android lifecycle events
        if events & ndk_sys::ALOOPER_EVENT_INPUT as i32 != 0 {
            // Input events
        }
    }
}

#[cfg(not(target_os = "android"))]
fn main() {}
