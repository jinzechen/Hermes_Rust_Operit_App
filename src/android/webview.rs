//! JNI-driven WebView — delegates UI-thread work to Java WebViewHelper.
//!
//! WebView MUST be created on the real Android main thread.
//! Looper.prepare() on a worker thread is NOT sufficient —
//! Android checks Thread.currentThread() == mainThread.
//!
//! Architecture:
//!   NativeActivity (ndk_glue) → worker thread (android_main)
//!   → JNI → WebViewHelper.createOnUiThread(activity)
//!   → UI thread: creates WebView, configures, loads HTML, setContentView

use jni::objects::{GlobalRef, JObject, JValue};
use jni::JNIEnv;
use log;

static mut WEBVIEW_REF: Option<GlobalRef> = None;

/// Set the HTML content on the Java helper, then delegate WebView
/// creation to the UI thread via WebViewHelper.createOnUiThread().
pub fn init_webview(env: &mut JNIEnv<'_>, activity: &JObject<'_>) {
    log::info!("[WebView] Setting HTML + delegating to UI thread...");

    // ── 1. Set the HTML content on the Java helper ──
    let html = get_chat_html();
    let helper = env
        .find_class("com/operit/hermes/WebViewHelper")
        .expect("WebViewHelper class not found — check DEX injection");

    env.call_static_method(
        &helper,
        "setHtml",
        "(Ljava/lang/String;)V",
        &[JValue::Object(&env.new_string(&html).unwrap())],
    )
    .expect("WebViewHelper.setHtml");
    log::info!("[WebView] HTML set ({} bytes)", html.len());

    // ── 2. Post WebView creation to UI thread ──
    env.call_static_method(
        &helper,
        "createOnUiThread",
        "(Landroid/app/Activity;)V",
        &[JValue::Object(activity)],
    )
    .expect("WebViewHelper.createOnUiThread");
    log::info!("[WebView] Posted WebView creation to UI thread");
}

/// Chat UI HTML — inlined for zero external file dependencies.
fn get_chat_html() -> String {
    r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1,user-scalable=no">
<title>Hermes Operit</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
body{font:14px sans-serif;background:#1a1a2e;color:#e0e0e0;display:flex;flex-direction:column;height:100vh;overflow:hidden}
#header{background:#16213e;padding:12px 16px;font-size:16px;font-weight:bold;color:#00d4ff;border-bottom:1px solid #0f3460;flex-shrink:0;display:flex;align-items:center;gap:8px}
#header .dot{width:8px;height:8px;border-radius:50%;background:#00ff88;display:inline-block}
#messages{flex:1;overflow-y:auto;padding:12px;display:flex;flex-direction:column;gap:8px}
.msg{max-width:85%;padding:10px 14px;border-radius:12px;line-height:1.5;word-wrap:break-word;animation:fadeIn .2s}
.msg.user{align-self:flex-end;background:#0f3460;color:#e0e0e0}
.msg.ai{align-self:flex-start;background:#16213e;color:#e0e0e0;border:1px solid #0f3460}
.msg.system{align-self:center;background:transparent;color:#666;font-size:12px;padding:4px 8px}
.msg .sender{font-size:11px;color:#00d4ff;margin-bottom:2px;font-weight:bold}
#input-area{flex-shrink:0;padding:10px 12px;background:#16213e;display:flex;gap:8px;border-top:1px solid #0f3460}
#input{flex:1;padding:10px 14px;border-radius:20px;border:1px solid #0f3460;background:#1a1a2e;color:#e0e0e0;font-size:14px;outline:none}
#input:focus{border-color:#00d4ff}
#send{padding:10px 20px;border-radius:20px;border:none;background:#00d4ff;color:#1a1a2e;font-weight:bold;cursor:pointer;font-size:14px}
#send:active{opacity:.8}
#status{text-align:center;font-size:11px;color:#666;padding:4px;flex-shrink:0}
@keyframes fadeIn{from{opacity:0;transform:translateY(4px)}to{opacity:1;transform:translateY(0)}}
</style>
</head>
<body>
<div id="header"><span class="dot"></span>Hermes Operit</div>
<div id="messages">
  <div class="msg system">Hermes Rust Agent v0.3.1 — JNI WebView</div>
</div>
<div id="status">已连接</div>
<div id="input-area">
  <input type="text" id="input" placeholder="输入消息..." autocomplete="off">
  <button id="send">发送</button>
</div>
<script>
const messages = document.getElementById('messages');
const input = document.getElementById('input');
const sendBtn = document.getElementById('send');
const statusEl = document.getElementById('status');

function addMessage(text, role) {
  const div = document.createElement('div');
  div.className = 'msg ' + role;
  if (role === 'user') {
    div.innerHTML = '<div class="sender">You</div>' + escapeHtml(text);
  } else if (role === 'ai') {
    div.innerHTML = '<div class="sender">Hermes</div>' + escapeHtml(text);
  } else {
    div.textContent = text;
  }
  messages.appendChild(div);
  messages.scrollTop = messages.scrollHeight;
}

function escapeHtml(s) {
  return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
}

function setStatus(text) { statusEl.textContent = text; }

function sendMessage() {
  const text = input.value.trim();
  if (!text) return;
  addMessage(text, 'user');
  input.value = '';
  setStatus('Echo:');
  setTimeout(() => { addMessage(text, 'ai'); setStatus('已连接'); }, 200);
}

sendBtn.addEventListener('click', sendMessage);
input.addEventListener('keydown', (e) => { if (e.key === 'Enter') sendMessage(); });
</script>
</body>
</html>"#
    .to_string()
}
