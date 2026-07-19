//! Android entry point — launched by NativeActivity.
//!
//! The Android system loads libhermes_operit_core.so and calls
//! `ANativeActivity_onCreate`, which ndk-glue forwards to `main()`.

#[cfg(target_os = "android")]
#[ndk_glue::main(backtrace = "on")]
fn android_main() {
    // Initialize Android logger so output goes to logcat
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("HermesOperit"),
    );

    log::info!("HermesOperit starting on Android...");

    // Launch the Dioxus app
    dioxus_mobile::launch(crate::ui::app::App);

    log::info!("HermesOperit started successfully");
}

// On non-Android targets, this file compiles to nothing.
#[cfg(not(target_os = "android"))]
fn main() {}
