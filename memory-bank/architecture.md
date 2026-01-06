# NeoLan 文件索引与作用说明

> 本文件说明 memory-bank 中每个文件的作用和使用场景

## 文档版本

| 版本号 | 日期 | 修订说明 |
|--------|------|---------|
| V1.0 | 2026-01-05 | 初始版本 |

---

# memory-bank 目录说明

`memory-bank/` 是项目的核心知识库目录，包含所有关键设计文档、技术规范和进度追踪文件。

## 文件清单

### 1. neolan-design-document.md

**作用**：完整的设计文档

**内容概要**：
- 产品定位与核心特性
- 技术架构设计（六大核心模块）
- 通信协议定义（IPMsg 兼容）
- 功能模块详细设计
- 数据模型设计
- 安全设计（加密、权限）
- 性能设计
- 部署方案

**何时阅读**：
- AI 生成任何代码前必须完整阅读
- 新成员加入项目时必读
- 架构评审时参考

**何时更新**：
- 数据库表结构变更时
- 协议定义变更时
- 架构调整时

---

### 2. tech-stack.md

**作用**：技术栈定义和模块化规范

**内容概要**：
- 技术栈选型决策
- 前端/后端依赖管理
- 模块化开发规范（文件拆分规则）
- 开发工具链配置
- 构建与部署配置
- 性能优化策略
- 安全性技术栈

**何时阅读**：
- AI 生成任何代码前必须完整阅读
- 添加新依赖时参考
- 代码审查时检查模块化规范

**何时更新**：
- 添加/删除依赖时
- 模块化规则调整时

---

### 3. implementation-plan.md

**作用**：详细的实施计划和路线图

**内容概要**：
- 8 个核心阶段的详细步骤
- 每步包含：目标、指令、验证测试、预期结果
- 聚焦基础功能，高级功能后置
- 里程碑检查清单

**何时阅读**：
- 开始开发前阅读整体计划
- 每个阶段开始前阅读对应步骤
- 遇到问题时参考验证测试方法

**何时更新**：
- 阶段顺序调整时
- 步骤细节优化时

---

### 4. progress.md

**作用**：开发进度追踪

**内容概要**：
- 所有步骤的完成状态检查清单
- 已完成步骤的详细记录
- 问题记录和解决方案

**何时阅读**：
- 每次开始新步骤前查看当前进度
- 团队同步进度时参考

**何时更新**：
- **每次完成一个步骤后立即更新**
- 记录遇到的问题和解决方案

---

### 5. architecture.md（本文件）

**作用**：文档索引和作用说明

**内容概要**：
- memory-bank 中每个文件的作用说明
- 文件间的依赖关系
- 使用场景和更新时机

**何时阅读**：
- 首次接触项目时
- 不确定应该阅读哪个文档时

**何时更新**：
- 添加新文档时
- 文档作用变更时

---

### 6. dto-definitions.md

**作用**：DTO 定义和前后端数据转换规范

**内容概要**：
- 所有 DTO 的 Rust 和 TypeScript 定义
- Model 到 DTO 的转换逻辑
- Tauri 事件 Payload 格式
- 前后端类型同步策略

**何时阅读**：
- 实现 Tauri 命令层（阶段 3）前必读
- 添加新的 Tauri 命令时参考
- 前端对接后端 API 时查看类型定义

**何时更新**：
- 添加新的 DTO 时
- 修改数据转换逻辑时

---

# 文档依赖关系

```
┌─────────────────────────────────────────────────────────┐
│                                                          │
│  ┌───────────────────────────────────────────────────┐  │
│  │     neolan-design-document.md（设计文档）          │  │
│  │     - 产品定位、架构、协议、功能、数据模型         │  │
│  └───────────────────┬───────────────────────────────┘  │
│                      │                                   │
│                      ├──► ┌──────────────────────────┐   │
│                      │    │ tech-stack.md           │   │
│                      │    │ - 技术选型、模块化规范   │   │
│                      │    └──────────────────────────┘   │
│                      │                                   │
│                      ├──► ┌──────────────────────────┐   │
│                      │    │ implementation-plan.md   │   │
│                      │    │ - 实施步骤、验证测试     │   │
│                      │    └──────────┬───────────────┘   │
│                      │               │                   │
│                      │               ▼                   │
│                      │    ┌──────────────────────────┐   │
│                      │    │ progress.md             │   │
│                      │    │ - 完成状态记录           │   │
│                      │    └──────────────────────────┘   │
│                      │                                   │
│                      └──► ┌──────────────────────────┐   │
│                           │ architecture.md（本文件）│   │
│                           │ - 文档索引和说明          │   │
│                           └──────────────────────────┘   │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

# 使用指南

## 对于 AI（代码生成）

在生成任何代码前，**必须**按以下顺序阅读：

1. **architecture.md**（本文件）- 了解文档结构
2. **tech-stack.md** - 了解技术栈和模块化规则
3. **neolan-design-document.md** - 了解完整设计
4. **implementation-plan.md** - 了解当前步骤

完成后，**必须**更新 **progress.md**

## 对于开发者

### 新人入门
1. 阅读 `architecture.md`（本文件）
2. 阅读 `neolan-design-document.md`
3. 阅读 `tech-stack.md`
4. 查看 `progress.md` 了解当前进度

### 日常开发
1. 查看 `progress.md` 确认当前步骤
2. 阅读 `implementation-plan.md` 对应步骤
3. 完成后更新 `progress.md`

### 架构变更
1. 修改 `neolan-design-document.md`
2. 检查 `tech-stack.md` 是否需要同步更新
3. 更新 `implementation-plan.md` 如有影响

---

# 强制规则（Always）

```markdown
# 写任何代码前必须完整阅读：
1. memory-bank/architecture.md（本文件）
2. memory-bank/tech-stack.md
3. memory-bank/neolan-design-document.md

# 每完成一个步骤后必须更新：
1. memory-bank/progress.md
   - 更新检查清单状态
   - 记录完成日期和遇到的问题

# 每完成一个里程碑后必须更新：
1. memory-bank/neolan-design-document.md
   - 数据库表结构变更
   - 新增模块/组件
   - 架构调整记录
```

---

# 项目架构状态

## 当前构建状态

| 组件 | 状态 | 说明 |
|------|------|------|
| 依赖层 | ✅ 阶段 0.1 完成 | 基础开发依赖已添加并验证 |
| 模块结构 | 🔄 阶段 0.2 待完成 | 模块目录结构未创建 |
| 数据持久化 | ⏳ 阶段 1 待开始 | 数据库层未实现 |
| 网络通信 | ⏳ 阶段 2 待开始 | 网络层未实现 |

## 依赖层次架构

```
┌─────────────────────────────────────────────────────────────────┐
│                        应用层 (Application)                      │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  commands/        - Tauri 命令 (前后端 IPC 接口)          │ │
│  │  modules/         - 业务逻辑模块 (peer/message/transfer)   │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              ▲                                  │
├──────────────────────────────┼──────────────────────────────────┤
│                              │                                  │
│  ┌───────────────────────────┴─────────────────────────────────┐ │
│  │                    核心层 (Core)                            │ │
│  │  ┌──────────────────────────────────────────────────────┐  │ │
│  │  │  network/        - 网络通信 (UDP/TCP)                │  │ │
│  │  │  storage/        - 数据持久化 (SeaORM + SQLite)       │  │ │
│  │  │  config/         - 配置管理                          │  │ │
│  │  │  utils/          - 工具函数                          │  │ │
│  │  └──────────────────────────────────────────────────────┘  │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                              ▲                                  │
├──────────────────────────────┼──────────────────────────────────┤
│                              │                                  │
│  ┌───────────────────────────┴─────────────────────────────────┐ │
│  │                   基础设施层 (Infrastructure)               │ │
│  │  ┌──────────────────────────────────────────────────────┐  │ │
│  │  │  tauri           - 桌面应用框架                      │  │ │
│  │  │  tokio           - 异步运行时                        │  │ │
│  │  │  sea-orm         - ORM 框架                          │  │ │
│  │  │  serde/serde_json - 序列化                           │  │ │
│  │  │  tracing         - 结构化日志                        │  │ │
│  │  │  thiserror/anyhow - 错误处理                         │  │ │
│  │  │  uuid/chrono     - 工具库                            │  │ │
│  │  └──────────────────────────────────────────────────────┘  │ │
│  └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## 依赖职责说明

### 错误处理层

| 依赖 | 职责 | 使用场景 |
|------|------|---------|
| **thiserror** | 定义结构化错误类型 | 在 `src-tauri/src/error.rs` 中定义 `NeoLanError` 枚举 |
| **anyhow** | 简化错误传播 | 在应用逻辑中使用 `anyhow::Result<T>` 快速处理错误 |

**设计模式**：
```rust
// 库级别错误 - 使用 thiserror
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NeoLanError {
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),
    #[error("Peer not found: {0}")]
    PeerNotFound(String),
}

// 应用级别错误 - 使用 anyhow
use anyhow::Result;

async fn send_message(&self, msg: &str) -> Result<()> {
    // 使用 ? 自动转换错误
    self.socket.send(msg.as_bytes()).await?;
    Ok(())
}
```

### 日志系统层

| 依赖 | 职责 | 使用场景 |
|------|------|---------|
| **tracing** | 结构化日志记录 | 在代码中使用 `tracing::info!()`, `tracing::error!()` |
| **tracing-subscriber** | 日志收集和输出 | 在 `run()` 函数中初始化，配置日志格式和过滤 |

**设计模式**：
```rust
// 初始化 (在 src-tauri/src/lib.rs)
tracing_subscriber::fmt()
    .with_env_filter(
        tracing_subscriber::EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into())
    )
    .init();

// 使用 (在业务代码中)
use tracing::{info, error, debug, instrument};

#[instrument(skip(self))]
async fn handle_message(&self, msg: &str) -> Result<()> {
    info!("Received message: {}", msg);
    // ...
    Ok(())
}
```

### 时间与唯一标识层

| 依赖 | 职责 | 使用场景 |
|------|------|---------|
| **chrono** | 时间处理 | 数据库时间戳、消息时间、心跳超时 |
| **uuid** | 唯一标识生成 | 消息 ID、传输任务 ID、会话 ID |

**设计模式**：
```rust
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Message {
    pub id: Uuid,                    // 唯一消息 ID
    pub sent_at: DateTime<Utc>,      // 发送时间
    pub received_at: Option<DateTime<Utc>>, // 接收时间
}

// 生成新消息
let msg = Message {
    id: Uuid::new_v4(),
    sent_at: Utc::now(),
    received_at: None,
};
```

### 数据序列化层

| 依赖 | 职责 | 使用场景 |
|------|------|---------|
| **serde** | 序列化框架 | 为结构体派生 `Serialize`/`Deserialize` |
| **serde_json** | JSON 格式支持 | 网络协议 content 字段、配置文件 |

**设计模式**：
```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProtocolMessage {
    pub version: u8,
    pub packet_id: u64,
    pub sender_name: String,
    pub content: String,  // 可能是 JSON 格式
}

// 序列化
let json = serde_json::to_string(&msg)?;

// 反序列化
let msg: ProtocolMessage = serde_json::from_str(&json)?;
```

## 当前文件结构状态

### 已创建的文件

```
src-tauri/
├── Cargo.toml           ✅ 已配置依赖
├── src/
│   ├── main.rs          ✅ Tauri 入口
│   ├── lib.rs           🔄 默认模板 (待扩展)
│   ├── build.rs         ⏳ 未创建 (如需要)
│   ├── migration/       ✅ 数据库迁移文件已存在
│   │   ├── mod.rs
│   │   └── m20260105_000001_create_tables.rs
│   └── storage/
│       └── entities/    ✅ 实体模型已存在
│           ├── mod.rs
│           ├── peers.rs
│           ├── messages.rs
│           ├── transfers.rs
│           ├── groups.rs
│           ├── settings.rs
│           └── audit_logs.rs
```

### 待创建的目录结构 (阶段 0.2)

```
src-tauri/src/
├── commands/           ⏳ Tauri 命令 (IPC 接口)
│   └── mod.rs
├── modules/            ⏳ 业务逻辑模块
│   ├── mod.rs
│   ├── peer/           ⏳ 节点管理
│   │   ├── mod.rs
│   │   ├── manager.rs
│   │   ├── discovery.rs
│   │   ├── state.rs
│   │   └── types.rs
│   ├── message/        ⏳ 消息处理
│   │   ├── mod.rs
│   │   ├── handler.rs
│   │   ├── router.rs
│   │   ├── crypto.rs
│   │   └── types.rs
│   ├── file_transfer/  ⏳ 文件传输
│   ├── crypto/         ⏳ 加密模块
│   └── group/          ⏳ 群组管理
├── network/            🔄 阶段 0.4 部分完成
│   ├── mod.rs          ✅ 已创建
│   ├── protocol.rs     ✅ 已实现 (IPMsg 协议解析/序列化)
│   ├── udp.rs          ⏳ 未创建 (UDP 传输)
│   └── tcp.rs          ⏳ 未创建 (TCP 传输)
├── config/             ⏳ 配置管理
│   ├── mod.rs
│   └── app.rs          ⏳ 应用配置
└── utils/              ⏳ 工具函数
    ├── mod.rs
    ├── logger.rs       ⏳ 日志初始化
    ├── hash.rs         ⏳ MD5 计算
    └── error.rs        ⏳ 错误类型 (阶段 0.4)
```

## 模块间依赖关系

```
                    ┌─────────────────┐
                    │   lib.rs        │  ← 应用入口
                    │   (run())       │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
        ┌──────────┐   ┌──────────┐   ┌──────────┐
        │ commands │   │ modules  │   │ network  │
        └────┬─────┘   └─────┬────┘   └─────┬────┘
             │               │              │
             └───────┬───────┘              │
                     ▼                      ▼
              ┌─────────────┐         ┌──────────┐
              │   storage   │         │  config  │
              │  (SeaORM)   │         └──────────┘
              └──────┬──────┘
                     ▼
              ┌─────────────┐
              │   utils/    │
              │   error.rs  │
              └─────────────┘
```

---

**最后更新：** 2026-01-05 (更新：阶段 0.1 依赖架构说明)
