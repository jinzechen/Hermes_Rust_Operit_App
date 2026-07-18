# Codex-Mobile + iHermes — 移动 AI Agent 模式
> codex-mobile: https://github.com/friuns2/codex-mobile
> iHermes: https://github.com/2winter-dev/iHermes
> 学习日期：2026-07-18

## codex-mobile
- **Termux 运行时**：Android 终端模拟器提供 Linux userspace
- **Web-first 桥接**：Browser → Express + WebSocket → Codex App Server
- **Cloudflared 隧道**：公网 URL + QR 码移动接入
- **电池管理**：wake-lock, 电池优化禁用, 持久通知

## iHermes
- **Expo SDK 54**：直接连 Hermes Agent，无后端
- **SSE 流式**：实时响应
- **多实例管理**：add/edit/delete/switch agent
- **expo-secure-store**：API key 平台原生安全存储

## 对 Hermes_Rust_Operit_App 的借鉴
| 模式 | codex-mobile | iHermes | 推荐 |
|------|-------------|---------|------|
| 运行时 | Termux (Node.js) | Native (Expo) | 两者都支持 |
| 连接 | Cloudflared + LAN | 直连 HTTP | 双模式 |
| 存储 | Firebase + FS | expo-secure-store | 平台原生安全存储 |
| API | Codex RPC/WS | OpenAI REST+SSE | OpenAI 兼容优先 |
