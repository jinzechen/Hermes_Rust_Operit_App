# Rust 密钥管理/凭据存储生态

> **来源**：hermes-agent-rs 的 `credential_pool.rs` + `oauth.rs`  
> **Hermes_Rust_Operit_App 评分**：★★★★（API Key 安全存储）

---

## 一、Rust 密钥管理方案

| crate | 用途 | 平台支持 | Hermes 适用 |
|-------|------|---------|-----------|
| **keyring-rs** | 系统密钥链访问 | Linux/macOS/Windows | ⚠️ Android 不支持 |
| **secrets** | 加密存储 | 跨平台 | ✅ |
| **safety** | 内存安全敏感数据 | 跨平台 | ✅ |
| **zeroize** | 内存清零 | 跨平台 | ✅ |
| **redb** | 加密 KV 存储 | ✅ 已有 | Hermes 已有 |

---

## 二、Android 上的方案

Android 没有系统密钥链（keyring-rs 不可用），用加密文件存储：

```rust
use zeroize::Zeroize;
use aes_gcm::Aes256Gcm;

// Agent API Key 加密存储
struct SecureStore {
    db: redb::Database,
    cipher: Aes256Gcm,
}

impl SecureStore {
    fn store_api_key(&self, provider: &str, key: &str) {
        let encrypted = self.cipher.encrypt(key.as_bytes());
        self.db.insert(provider, &encrypted)?;
    }
    
    fn get_api_key(&self, provider: &str) -> Option<String> {
        let encrypted = self.db.get(provider)?;
        let decrypted = self.cipher.decrypt(encrypted)?;
        Some(String::from_utf8(decrypted))
    }
}
```

### 评分：★★★★

Hermes_Rust_Operit_App 已有 redb 做 KV 存储，加上 `zeroize` + `aes-gcm` 即可实现 Android 的 API Key 安全存储。
