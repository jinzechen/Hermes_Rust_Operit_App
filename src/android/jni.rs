//! JNI bridge — Agent lifecycle management.
//!
//! Corresponding Kotlin: `com.operit.hermes.bridge.HermesBridge`
//! All functions are `#[no_mangle]` so the JVM can find them.
//! When compiling on non-Android hosts the whole module is gated out.

use jni::objects::{JClass, JObject, JString};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;

/// Creates a native Agent instance and returns an opaque pointer (jlong).
///
/// Kotlin: `external fun nativeInit(sendMsgCb: Any, toolNamesCb: Any): Long`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeInit<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    send_message_callback: JObject<'local>,
    tool_names_callback: JObject<'local>,
) -> jlong {
    log::info!("[JNI] nativeInit called");

    // Keep the callbacks as global refs so they survive across JNI calls.
    let _send_cb = env
        .new_global_ref(send_message_callback)
        .unwrap_or_else(|e| {
            log::error!("[JNI] Failed to create global ref for sendMessage callback: {e}");
            panic!("JNI global ref error");
        });
    let _tools_cb = env.new_global_ref(tool_names_callback).unwrap_or_else(|e| {
        log::error!("[JNI] Failed to create global ref for toolNames callback: {e}");
        panic!("JNI global ref error");
    });

    // Placeholder: In a real build this would instantiate the Agent struct
    // and stash the callbacks inside it. Return a dummy non-null pointer.
    log::info!("[JNI] Agent initialized (placeholder), returning ptr=0x1");
    0x1
}

/// Send a chat message to the native Agent and return the response.
///
/// Kotlin: `external fun nativeSendMessage(agentPtr: Long, message: String): String`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeSendMessage<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    agent_ptr: jlong,
    message: JString<'local>,
) -> jstring {
    let msg: String = env
        .get_string(&message)
        .map(|s| s.into())
        .unwrap_or_default();

    log::info!("[JNI] nativeSendMessage(ptr=0x{agent_ptr:x}, msg=\"{msg}\")");

    // Placeholder: real impl delegates to Agent::chat().
    let reply = format!("[placeholder] Received: {msg}");
    env.new_string(reply)
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Return a JSON array of available tool names.
///
/// Kotlin: `external fun nativeToolNames(agentPtr: Long): String`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeToolNames<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    agent_ptr: jlong,
) -> jstring {
    log::info!("[JNI] nativeToolNames(ptr=0x{agent_ptr:x})");

    // Placeholder: real impl queries the tool registry.
    env.new_string("[\"browser\",\"terminal\",\"filesystem\"]")
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Destroy the native Agent and free associated resources.
///
/// Kotlin: `external fun nativeDestroy(agentPtr: Long)`
#[no_mangle]
pub extern "C" fn Java_com_operit_hermes_bridge_HermesBridge_nativeDestroy<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    agent_ptr: jlong,
) {
    log::info!("[JNI] nativeDestroy(ptr=0x{agent_ptr:x}) — cleaning up");
    // Placeholder: drop Agent, delete global refs, free memory.
}
