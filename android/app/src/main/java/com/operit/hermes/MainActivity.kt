package com.operit.hermes

import android.os.Bundle
import android.view.WindowManager
import androidx.appcompat.app.AppCompatActivity

/**
 * Main entry point for the Hermes Operit Dioxus application.
 * This Activity hosts the native Rust/Dioxus UI via a SurfaceView
 * and initializes the HermesBridge JNI layer.
 */
class MainActivity : AppCompatActivity() {

    private var nativePtr: Long = 0

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Keep screen on during development / long-running sessions
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)

        // Initialize the native bridge
        val configJson = """
            {
                "data_dir": "${filesDir.absolutePath}",
                "cache_dir": "${cacheDir.absolutePath}",
                "api_base_url": "https://api.operit.ai"
            }
        """.trimIndent()

        nativePtr = HermesBridge.nativeInit(configJson)

        // The native code will create the Dioxus surface and set it as content
        setContentView(R.layout.activity_main)

        // Handle deep links (OAuth callbacks, etc.)
        intent?.data?.let { uri ->
            handleDeepLink(uri.toString())
        }
    }

    override fun onNewIntent(intent: android.content.Intent) {
        super.onNewIntent(intent)
        intent.data?.let { uri ->
            handleDeepLink(uri.toString())
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        // Clean up native resources if needed
    }

    private fun handleDeepLink(uri: String) {
        if (nativePtr != 0L) {
            HermesBridge.nativeSendMessage(nativePtr, "deep_link", uri)
        }
    }
}
