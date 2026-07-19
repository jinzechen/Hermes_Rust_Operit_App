//! Direct WebView creation on the main thread.
//! Called from ANativeActivity_onCreate which runs on the main thread.

use jni::objects::{GlobalRef, JObject, JValue};
use jni::JNIEnv;
use log;

static mut WEBVIEW_REF: Option<GlobalRef> = None;

/// Create WebView directly — caller MUST be on Android main thread.
pub fn init_webview_direct(env: &mut JNIEnv<'_>, activity: &JObject<'_>) {
    log::info!("[WebView] Creating directly on main thread...");

    let ctx = JObject::from(
        env.call_method(
            activity,
            "getApplicationContext",
            "()Landroid/content/Context;",
            &[],
        )
        .unwrap()
        .l()
        .unwrap(),
    );

    let webview = env
        .new_object(
            "android/webkit/WebView",
            "(Landroid/content/Context;)V",
            &[JValue::Object(&ctx)],
        )
        .expect("new WebView");

    // Configure
    let settings = env
        .call_method(&webview, "getSettings", "()Landroid/webkit/WebSettings;", &[])
        .unwrap();
    let s = settings.l().unwrap();
    let _ = env.call_method(s, "setJavaScriptEnabled", "(Z)V", &[JValue::Bool(1)]);
    let _ = env.call_method(s, "setDomStorageEnabled", "(Z)V", &[JValue::Bool(1)]);

    // Store global ref
    let global = env.new_global_ref(&webview).unwrap();
    unsafe { WEBVIEW_REF = Some(global); }

    // Replace content
    env.call_method(
        activity,
        "setContentView",
        "(Landroid/view/View;)V",
        &[JValue::Object(&webview)],
    )
    .expect("setContentView");

    // Load HTML
    let html = get_chat_html();
    let base_url = env.new_string("file:///android_asset/").unwrap();
    let mime = env.new_string("text/html").unwrap();
    let encoding = env.new_string("utf-8").unwrap();
    let history = env.new_string("about:blank").unwrap();

    env.call_method(
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
    )
    .expect("loadDataWithBaseURL");

    log::info!("[WebView] Created + HTML loaded ({} bytes)", html.len());
}

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
#header{background:#16213e;padding:12px 16px;font-size:16px;font-weight:bold;color:#00d4ff;border-bottom:1px solid #0f3460}
#messages{flex:1;overflow-y:auto;padding:12px;display:flex;flex-direction:column;gap:8px}
.msg{max-width:85%;padding:10px 14px;border-radius:12px;line-height:1.5;animation:fadeIn .2s}
.msg.user{align-self:flex-end;background:#0f3460}
.msg.ai{align-self:flex-start;background:#16213e;border:1px solid #0f3460}
.msg.system{align-self:center;background:transparent;color:#666;font-size:12px;padding:4px 8px}
.msg .sender{font-size:11px;color:#00d4ff;margin-bottom:2px;font-weight:bold}
#input-area{flex-shrink:0;padding:10px 12px;background:#16213e;display:flex;gap:8px;border-top:1px solid #0f3460}
#input{flex:1;padding:10px 14px;border-radius:20px;border:1px solid #0f3460;background:#1a1a2e;color:#e0e0e0;font-size:14px;outline:none}
#input:focus{border-color:#00d4ff}
#send{padding:10px 20px;border-radius:20px;border:none;background:#00d4ff;color:#1a1a2e;font-weight:bold;cursor:pointer}
#status{text-align:center;font-size:11px;color:#666;padding:4px;flex-shrink:0}
@keyframes fadeIn{from{opacity:0;transform:translateY(4px)}to{opacity:1;transform:translateY(0)}}
</style>
</head>
<body>
<div id="header">Hermes Operit</div>
<div id="messages"><div class="msg system">Hermes v0.3.2 — Ready</div></div>
<div id="status">已连接</div>
<div id="input-area">
  <input type="text" id="input" placeholder="输入消息..." autocomplete="off">
  <button id="send">发送</button>
</div>
<script>
const m=document.getElementById('messages'),i=document.getElementById('input');
document.getElementById('send').onclick=()=>{let t=i.value.trim();if(!t)return;
let d=document.createElement('div');d.className='msg user';d.innerHTML='<div class=sender>You</div>'+t;m.appendChild(d);
i.value='';setTimeout(()=>{let r=document.createElement('div');r.className='msg ai';
r.innerHTML='<div class=sender>Hermes</div>'+t;m.appendChild(r);m.scrollTop=m.scrollHeight},200);
m.scrollTop=m.scrollHeight};
i.onkeydown=e=>{if(e.key=='Enter')document.getElementById('send').click()};
</script>
</body>
</html>"#
    .to_string()
}
