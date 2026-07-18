# SESSION_HANDOFF.md — 逐步骤执行清单

> 目标 AI：deepseek-v4-flash  
> 规则：严格按编号顺序执行，完成一步再下一步。每步完成后 `git commit && git push`。

---

## 第 0 步：了解项目

读这 3 个文件了解全局（不要读别的，浪费时间）：
1. `D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App\README.md`
2. `D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App\ARCHITECTURE.md`
3. `D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App\Cargo.toml`

---

## 第 1 步：修复 CI

```
任务：让 GitHub Actions 的 cargo check 通过
当前状态：CI 失败，exit code 101
```

**操作**：
1. `git pull` 拉最新代码
2. 打开 `D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App\.github\workflows\build.yml`
3. 确认内容只有 `cargo check` 和 `cargo test`
4. 推送后跑 `gh workflow run build.yml --repo jinzechen/Hermes_Rust_Operit_App`
5. 等 CI 结果：`gh run watch <run_id> --repo jinzechen/Hermes_Rust_Operit_App --exit-status`
6. 如果失败，用 `gh run view <run_id> --log --job=<job_id>` 查看错误
7. 修改对应源码文件，提交推送，重复直到 CI 绿色

**已知可能错误**：
- `src/core/memory.rs` — redb `ReadTransaction` 生命周期问题
- `src/ui/login.rs` — oauth2 `request()` API 签名问题
- 各种 `use anyhow::anyhow` 缺失

**检查命令**（用于 CI 日志）：
```
gh run view <run_id> --repo jinzechen/Hermes_Rust_Operit_App --log 2>&1 | grep "error\[E" | head -10
```

---

## 第 2 步：替换记忆系统

```
任务：用 tinycortex 替换当前的简陋 MemoryStore
来源：Learning/02-OpenHuman-记忆系统.md
```

**操作**：
1. 读 `Learning/02-OpenHuman-记忆系统.md` 了解 tinycortex API
2. `cargo add tinycortex --features git-diff,persona`（在项目目录执行）
3. 修改 `src/core/memory.rs`：
   - 保留 `Message` 结构体
   - 删除 `MemoryStore` 和它的 redb 实现
   - 新建 `MemoryStore` 封装 tinycortex 的 API
   - `save_session()` → tinycortex 的 memory store
   - `load_session()` → tinycortex 的 recall
4. 修改 `src/core/agent.rs` 中所有引用 `MemoryStore` 的地方适配新 API
5. `git commit -m "Replace MemoryStore with tinycortex" && git push`
6. 确认 CI 通过

---

## 第 3 步：完善 Skill 系统

```
任务：实现完整的 Skill 生命周期
来源：Learning/13-Operit-插件格式规范.md + Learning/14-Python-Hermes-原版Agent.md
```

**操作**：
1. 读 `Learning/13-Operit-插件格式规范.md` 的 Skill 格式部分
2. 读 `Learning/14-Python-Hermes-原版Agent.md` 的 Skills 系统部分
3. 修改 `src/store/mod.rs`：
   - `skill_view(name)` → 读取 SKILL.md 返回内容
   - `skill_list()` → 扫描目录返回所有 skill 名称
   - `skill_install(zip_path)` → 解压 ZIP 到 skills 目录
4. 在 `src/core/agent.rs` 添加：
   - AgentManager 新增方法：`invoke_skill(skill_name, user_text)`
   - skill 调用时展开 SKILL.md 内容到 system prompt
5. `git commit -m "Complete skill lifecycle" && git push`

---

## 第 4 步：Agent Loop 增强

```
任务：参考 Python Hermes 增强 AgentLoop
来源：Learning/14-Python-Hermes-原版Agent.md
```

**操作**：
1. 读 `Learning/14-Python-Hermes-原版Agent.md` 的 Agent Loop 部分
2. 修改 `src/core/agent.rs` 的 `agent_loop()` 方法：
   - 添加 `max_iterations` 限制（默认 20）
   - 添加 context 压力检测（消息数 > 阈值时压缩）
   - 添加 tool call 去重（同名同参数不重复执行）
3. `git commit -m "Enhance AgentLoop: iteration limit, context pressure, tool dedup" && git push`

---

## 第 5 步：安全层

```
任务：添加基本的工具安全策略
来源：Learning/03-Hermes-Agent-Ultra-安全策略.md
```

**操作**：
1. 读 `Learning/03-Hermes-Agent-Ultra-安全策略.md` 的 guard.rs 部分
2. 新建 `src/core/guard.rs`：
   - 危险命令检测函数（rm -rf, sudo, chmod 777 等）
   - prompt injection 检测函数
3. 修改 `src/core/tool_registry.rs`：
   - ToolRegistry::execute_tool() 前调用 guard 检查
4. `git commit -m "Add basic tool security guard" && git push`

---

## 第 6 步：Token 优化

```
任务：添加工具输出过滤
来源：Learning/07-rtk-cc-switch-优化与路由.md
```

**操作**：
1. 新建 `src/core/optimizer.rs`
2. 实现 3 个基础过滤策略：
   - 去重：相同行只保留一条 + 计数
   - 截断：输出超过 N 字符自动截断
   - 空行压缩：连续空行 → 单个空行
3. 修改 `src/core/agent.rs`：
   - `execute_tool_call()` 后调用 optimizer.filter()
4. `git commit -m "Add token optimizer with dedup/truncate/compact" && git push`

---

## 第 7 步：补充缺失模块

```
任务：创建之前计划但未实现的文件
```

**逐个创建以下文件**（不要批量，一个一个来）：

### 7a: `src/core/character.rs`
```rust
// 角色卡系统，仿 Operit CharacterCard 结构
// CharacterCard { id, name, description, persona_prompt, opening_statement, tags }
// CharacterCardManager { load(), save(), list(), set_active() }
```

### 7b: `src/store/sources.rs`
```rust
// GitHub 源聚合
// SourceConfig { name, url, category }
// SourceManager { add_source(), list_sources(), fetch_index() }
```

### 7c: `src/core/local_model.rs`
```rust
// 本地模型管理（骨架）
// LocalModelManager { list_models(), load_model() }
```

### 7d: 更新 `src/core/mod.rs` 添加新模块声明
### 7e: 更新 `src/lib.rs` 添加新模块声明

`git commit -m "Add character, sources, local_model modules" && git push`

---

## 第 8 步：确认全部通过

```
cargo check 通过
cargo test 通过
CI 绿色
```

---

## ⚠️ 重要规则

1. **每步完成后必须 git commit && git push**，不要攒着一起提交
2. **不要从头学起**——17 份学习报告在 Learning/ 目录，直接读需要的
3. **不要改已有模块的接口签名**——只做增量修改
4. **cargo check 失败时优先修编译错误**，不要继续写新功能
5. **回复用中文**，简洁直接，不要废话

---

*生成时间：2026-07-18 | 生成者：deepseek-v4-pro*
