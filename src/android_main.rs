//! Android native library entry.
//! Doesn't export ANativeActivity_onCreate — Java MainActivity handles the entry point.
//! JNI bridge functions are in src/android/jni.rs etc.

// On Android, this module just exists for the mod declaration in lib.rs.
// Java's MainActivity (com.operit.hermes.MainActivity) is the real entry point.
