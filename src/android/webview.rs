//! JNI-driven WebView — creates and manages android.webkit.WebView directly.
//!
//! Avoids dioxus-mobile/tao/wry's experimental Android support by using
//! raw JNI calls to create a WebView inside the NativeActivity.
//!
//! Architecture:
//!   NativeActivity (ndk-glue) → JNI → android.webkit.WebView
//!   WebView loads HTML chat UI → JS ↔ Rust via JavascriptInterface

use jni::objects::{GlobalRef, JObject, JValue};
use jni::JNIEnv;
use log;

/// Holds global references to the WebView and related JNI objects
/// so they survive across JNI calls.
static mut WEBVIEW_REF: Option<GlobalRef> = None;

/// Initialize a WebView inside the current NativeActivity.
///
/// Called from android_main() on the Android main thread.
/// Sets up JavaScript interface for Rust↔JS communication and
/// loads the chat UI HTML.
pub fn init_webview(env: &mut JNIEnv<'_>, activity: &JObject<'_>) {
    log::info!("[WebView] Initializing...");

    // ── 1. Get Activity → Window → DecorView ──
    let window = match env.call_method(activity, "getWindow", "()Landroid/view/Window;", &[]) {
        Ok(w) => w,
        Err(e) => {
            log::error!("[WebView] getWindow failed: {e}");
            return;
        }
    };

    let decor = match env.call_method(
        &window.l().unwrap(),
        "getDecorView",
        "()Landroid/view/View;",
        &[],
    ) {
        Ok(d) => d,
        Err(e) => {
            log::error!("[WebView] getDecorView failed: {e}");
            return;
        }
    };

    // ── 2. Get content parent (FrameLayout) ──
    let content = match env.call_method(
        &decor.l().unwrap(),
        "findViewById",
        "(I)Landroid/view/View;",
        &[JValue::Int(0x01020002)], // android.R.id.content
    ) {
        Ok(c) => c,
        Err(e) => {
            log::error!("[WebView] findViewById(content) failed: {e}");
            return;
        }
    };

    // ── 3. Create WebView ──
    let ctx = JObject::from(env.call_method(
        activity,
        "getApplicationContext",
        "()Landroid/content/Context;",
        &[],
    ).unwrap().l().unwrap());

    let webview = match env.new_object(
        "android/webkit/WebView",
        "(Landroid/content/Context;)V",
        &[JValue::Object(&ctx)],
    ) {
        Ok(w) => w,
        Err(e) => {
            log::error!("[WebView] new WebView failed: {e}");
            return;
        }
    };

    // ── 4. Configure WebSettings ──
    let settings = env
        .call_method(
            &webview,
            "getSettings",
            "()Landroid/webkit/WebSettings;",
            &[],
        )
        .unwrap();
    let settings_obj = settings.l().unwrap();

    let _ = env.call_method(&settings_obj, "setJavaScriptEnabled", "(Z)V", &[JValue::Bool(1)]);
    let _ = env.call_method(
        &settings_obj,
        "setDomStorageEnabled",
        "(Z)V",
        &[JValue::Bool(1)],
    );
    let _ = env.call_method(
        &settings_obj,
        "setAllowFileAccess",
        "(Z)V",
        &[JValue::Bool(1)],
    );

    log::info!("[WebView] WebSettings configured");

    // ── 5. Add JavaScript interface for Rust↔JS bridged ──
    // Uses the existing HermesBridge JNI functions exposed via @JavascriptInterface
    // on the Kotlin side. For now, create a simple bridge object.
    match env.find_class("com/operit/hermes/bridge/HermesBridge") {
        Ok(cls) => {
            let bridge = env.new_object(&cls, "()V", &[]).unwrap();
            let _ = env.call_method(
                &webview,
                "addJavascriptInterface",
                "(Ljava/lang/Object;Ljava/lang/String;)V",
                &[
                    JValue::Object(&bridge),
                    JValue::Object(&env.new_string("HermesBridge").unwrap()),
                ],
            );
            log::info!("[WebView] JavascriptInterface HermesBridge registered");
        }
        Err(_) => {
            log::warn!("[WebView] HermesBridge class not found (no Kotlin companion?)");
        }
    }

    // ── 6. Store global ref so WebView stays alive ──
    let global = env.new_global_ref(&webview).unwrap();
    unsafe {
        WEBVIEW_REF = Some(global);
    }

    // ── 7. Replace content view with WebView ──
    let content_obj = content.l().unwrap();
    if content_obj.is_null() {
        // Fallback: set WebView directly as activity content
        let _ = env.call_method(
            activity,
            "setContentView",
            "(Landroid/view/View;)V",
            &[JValue::Object(&webview)],
        );
        log::info!("[WebView] setContentView directly");
    } else {
        // Add WebView to the content FrameLayout
        let _ = env.call_method(
            content_obj,
            "addView",
            "(Landroid/view/View;)V",
            &[JValue::Object(&webview)],
        );
        log::info!("[WebView] Added to content FrameLayout");
    }

    // ── 8. Load HTML ──
    let html = get_chat_html();
    let base_url = env.new_string("file:///android_asset/").unwrap();
    let mime = env.new_string("text/html").unwrap();
    let encoding = env.new_string("utf-8").unwrap();
    let history = env.new_string("about:blank").unwrap();

    let _ = env.call_method(
        &webview,
        "loadDataWithBaseURL",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
        &[
            JValue::Object(&base_url),
            JValue::Object(&env.new_string(&html).unwrap()),
            JValue::Object(&mime),
            JValue::Object(&encoding),
            JValue::Object(&history),
        ],
    );

    log::info!("[WebView] HTML loaded, length={} bytes", html.len());
}

/// Chat UI HTML — inlined for zero external file dependencies.
///
/// The JavaScript in this page communicates with Rust via:
///   window.HermesBridge.nativeSendMessage(ptr, msg) → returns reply
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

function setStatus(text) {
  statusEl.textContent = text;
}

function sendMessage() {
  const text = input.value.trim();
  if (!text) return;
  addMessage(text, 'user');
  input.value = '';
  setStatus('思考中...');

  // Try to call native bridge
  if (window.HermesBridge && typeof window.HermesBridge.sendToRust === 'function') {
    try {
      const reply = window.HermesBridge.sendToRust(text);
      addMessage(reply || '(empty reply)', 'ai');
      setStatus('已连接');
    } catch (e) {
      addMessage('[Bridge error: ' + e + ']', 'system');
      setStatus('Bridge 错误');
    }
  } else {
    // Fallback: echo for testing
    setTimeout(() => {
      addMessage('[Hermes Bridge 未连接 — 回显测试] ' + text, 'system');
      setStatus('Bridge 未连接 (测试模式)');
    }, 300);
  }
}

sendBtn.addEventListener('click', sendMessage);
input.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') sendMessage();
});
</script>
</body>
</html>"#
    .to_string()
}
