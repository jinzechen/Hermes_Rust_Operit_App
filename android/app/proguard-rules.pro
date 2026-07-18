# ──────────────────────────────────────────────
#  Hermes Operit — ProGuard / R8 Rules
# ──────────────────────────────────────────────

# Keep JNI bridge — all native methods and their class
-keep class com.operit.hermes.bridge.HermesBridge {
    native <methods>;
    *;
}

# Keep Application and Activity classes
-keep class com.operit.hermes.HermesApplication { *; }
-keep class com.operit.hermes.MainActivity { *; }

# Keep service classes
-keep class com.operit.hermes.service.** { *; }

# Keep Kotlin serialization-related classes
-keepattributes *Annotation*
-keepattributes InnerClasses
-keep class kotlinx.serialization.** { *; }

# Keep JSON model classes (if any)
-keep class com.operit.hermes.model.** { *; }

# ──────────────────────────────────────────────
#  General Android / Kotlin rules
# ──────────────────────────────────────────────

-keepattributes Signature
-keepattributes Exceptions

# Don't warn about missing classes from optional dependencies
-dontwarn com.google.android.**
-dontwarn org.jetbrains.annotations.**
-dontwarn kotlin.**

# Keep the application entry point
-keep class com.operit.hermes.** {
    public protected *;
}
