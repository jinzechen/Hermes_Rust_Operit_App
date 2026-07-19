//! WebView setup helper — provides a Runnable that sets the WebView
//! as the Activity's content view from the main (UI) thread.
//!
//! This is the ONLY Java file in the project. It exists because
//! setContentView MUST be called from the main thread, and creating
//! a Runnable purely via JNI proxy is impractical without Kotlin.
//!
//! The native code stores the WebView in a static field, then calls
//! setupWebViewOnUiThread() which posts a Runnable that calls
//! nativeOnUiThread — running on the main thread.

package com.operit.hermes;

import android.app.Activity;
import android.webkit.WebView;

public class WebViewHelper {
    private static WebView sWebView;
    private static Activity sActivity;

    /** Called from native to store references, then post to UI thread. */
    public static void setupWebViewOnUiThread(Activity activity, WebView webview) {
        sWebView = webview;
        sActivity = activity;
        activity.runOnUiThread(new Runnable() {
            @Override
            public void run() {
                doSetupWebView();
            }
        });
    }

    /** Actually sets the WebView — runs on UI thread. */
    private static void doSetupWebView() {
        if (sWebView != null && sActivity != null) {
            sActivity.setContentView(sWebView);
            android.util.Log.i("HermesOperit", "[Java] WebView setContentView done on UI thread");
        }
    }
}
