package com.operit.hermes;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.WebView;
import android.webkit.WebSettings;

public class MainActivity extends Activity {
    @Override
    protected void onCreate(Bundle saved) {
        super.onCreate(saved);

        // Load Rust native library
        try { System.loadLibrary("hermes_operit_core"); } catch (UnsatisfiedLinkError e) {
            android.util.Log.w("HermesOperit", "Native lib not loaded: " + e);
        }

        WebView wv = new WebView(this);
        WebSettings s = wv.getSettings();
        s.setJavaScriptEnabled(true);
        s.setDomStorageEnabled(true);

        wv.loadDataWithBaseURL(
            "file:///android_asset/",
            getHtml(),
            "text/html",
            "utf-8",
            "about:blank"
        );

        setContentView(wv);
    }

    private String getHtml() {
        return "<!DOCTYPE html><html lang=zh-CN><head><meta charset=utf-8>" +
        "<meta name=viewport content='width=device-width,initial-scale=1'>" +
        "<style>*{margin:0;padding:0;box-sizing:border-box}" +
        "body{font:14px sans-serif;background:#1a1a2e;color:#e0e0e0;display:flex;flex-direction:column;height:100vh}" +
        "#header{background:#0f3460;padding:12px 16px;font-size:16px;font-weight:bold;color:#00d4ff}" +
        "#msg{flex:1;overflow-y:auto;padding:12px}" +
        "#input-area{padding:10px 12px;background:#0f3460;display:flex;gap:8px}" +
        "#input{flex:1;padding:10px;border-radius:20px;border:1px solid #333;background:#1a1a2e;color:#e0e0e0;outline:none}" +
        "#send{padding:10px 20px;border-radius:20px;border:none;background:#00d4ff;color:#000;font-weight:bold}" +
        ".msg{padding:8px 12px;border-radius:10px;margin:4px 0;max-width:85%;animation:fadein .2s}" +
        ".user{margin-left:auto;background:#16213e}" +
        ".ai{background:#0f3460}" +
        ".sys{text-align:center;color:#666;font-size:12px}" +
        "@keyframes fadein{from{opacity:0}to{opacity:1}}" +
        "</style></head><body>" +
        "<div id=header>Hermes Operit</div>" +
        "<div id=msg><div class='msg sys'>v0.3.2 — Ready</div></div>" +
        "<div id=input-area>" +
        "<input id=input placeholder='输入消息...' autocomplete=off>" +
        "<button id=send>发送</button></div>" +
        "<script>var m=document.getElementById('msg'),i=document.getElementById('input');" +
        "document.getElementById('send').onclick=function(){" +
        "var t=i.value.trim();if(!t)return;" +
        "m.innerHTML+='<div class=\"msg user\">'+t+'</div>';" +
        "i.value='';" +
        "setTimeout(function(){m.innerHTML+='<div class=\"msg ai\">'+t+'</div>';m.scrollTop=m.scrollHeight},200);" +
        "m.scrollTop=m.scrollHeight};" +
        "i.onkeydown=function(e){if(e.key=='Enter')document.getElementById('send').click()};" +
        "</script></body></html>";
    }
}
