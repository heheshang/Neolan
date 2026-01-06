# NeoLan 开发进度记录

> 本文件用于记录项目实施过程中已完成的步骤

## 文档说明

本文件跟踪 `implementation-plan.md` 中各步骤的完成状态。

---

# 进度追踪

## 阶段 0：项目基础设施搭建

- [x] 0.1 添加基础开发依赖
- [x] 0.2 创建模块目录结构
- [x] 0.3 配置日志系统
- [x] 0.4 创建错误类型系统

## 阶段 1：数据持久化层

- [x] 1.1 创建数据库连接模块
- [ ] 1.2 定义数据库实体模型
- [x] 1.3 创建数据库迁移
- [x] 1.4 实现节点数据访问层
- [x] 1.5 实现消息数据访问层
- [x] 1.6 实现配置存储模块

## 阶段 2：网络通信层

- [x] 2.1 实现协议解析器
- [x] 2.2 实现 UDP 传输模块
- [x] 2.3 实现节点发现功能
- [x] 2.4 实现节点管理器
- [x] 2.5 实现心跳机制

## 阶段 3：Tauri 命令层

- [x] 3.1 创建节点查询命令
- [x] 3.2 创建配置相关命令
- [x] 3.3 创建状态监听命令

## 阶段 4：前端基础 UI

- [ ] 4.1 安装前端依赖
- [ ] 4.2 创建 Pinia Store
- [ ] 4.3 创建节点列表组件
- [ ] 4.4 创建设置页面组件

## 阶段 5：即时消息功能（基础）

- [ ] 5.1 实现消息数据结构
- [ ] 5.2 实现消息发送功能
- [ ] 5.3 实现消息接收功能
- [ ] 5.4 创建消息查询命令
- [ ] 5.5 创建聊天窗口组件

## 阶段 6：文件传输（基础）

- [ ] 6.1 实现文件元数据计算
- [ ] 6.2 实现文件传输请求
- [ ] 6.3 实现文件传输响应
- [ ] 6.4 实现 TCP 文件传输
- [ ] 6.5 创建文件传输 UI 组件

## 阶段 7：测试和优化

- [ ] 7.1 编写集成测试
- [ ] 7.2 性能测试
- [ ] 7.3 内存泄漏检查

## 阶段 8：打包和部署

- [ ] 8.1 配置应用元数据
- [ ] 8.2 构建生产版本

---

# 完成记录

## ✅ 阶段 0.1：添加基础开发依赖

### 完成日期
2026-01-05

### 完成内容

已在 `src-tauri/Cargo.toml` 中添加以下基础开发依赖：

#### 错误处理库
- **thiserror = "1"** - 用于派生错误类型，提供结构化的错误处理
- **anyhow = "1"** - 简化错误处理，用于不需要精确错误类型的场景

#### 工具库
- **uuid = { version = "1", features = ["v4", "serde"] }** - UUID 生成，支持 v4 版本和序列化
- **chrono = { version = "0.4", features = ["serde"] }** - 时间处理，支持 UTC 时间和序列化
- **tracing = "0.1"** - 结构化日志框架
- **tracing-subscriber = { version = "0.3", features = ["env-filter"] }** - 日志订阅器，支持环境变量过滤

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 23.34s |
| `cargo build` | ✅ 通过 | 6m 11s |
| 依赖解析 | ✅ 成功 | - |
| 编译输出 | ✅ `neolan v0.1.0` | - |

### 遇到的问题
无

### 解决方案
N/A

### 架构洞察

#### 依赖选型理由

1. **thiserror vs anyhow**
   - `thiserror`：用于定义库级别的错误类型，提供 `#[error(...)]` 派生宏
   - `anyhow`：用于应用级别错误处理，简化 `Result` 类型的使用
   - 两者配合使用：库代码用 `thiserror`，应用逻辑用 `anyhow`

2. **tracing vs log**
   - `tracing` 是新一代结构化日志框架，支持异步上下文和 Span 追踪
   - 更适合 Tokio 异步运行时环境
   - 与 `tracing-subscriber` 配合实现灵活的日志输出控制

3. **chrono features**
   - `serde` feature 允许 DateTime 类型自动序列化/反序列化
   - 对于数据库存储和网络传输至关重要

### 后续步骤
下一步是 **0.2: 创建模块目录结构**，需要创建以下目录结构：
```
src-tauri/src/
├── commands/
├── modules/
├── modules/peer/
├── modules/message/
├── modules/file_transfer/
├── modules/crypto/
├── modules/group/
├── network/
├── storage/
├── config/
└── utils/
```

---

## ✅ 阶段 0.2：创建模块目录结构

### 完成日期
2026-01-05

### 完成内容

#### 创建的目录结构
```
src-tauri/src/
├── commands/           # Tauri 命令 (IPC 接口)
├── modules/            # 业务逻辑模块
│   ├── peer/           # 节点管理
│   ├── message/        # 消息处理
│   ├── file_transfer/  # 文件传输
│   ├── crypto/         # 加密模块
│   └── group/          # 群组管理
├── network/            # 网络通信
├── storage/            # 数据持久化 (已存在，补充 mod.rs)
│   └── entities/       # 实体模型 (已存在)
├── config/             # 配置管理
└── utils/              # 工具函数
```

#### 创建的文件
- [commands/mod.rs](src-tauri/src/commands/mod.rs) - IPC 接口层
- [modules/mod.rs](src-tauri/src/modules/mod.rs) - 业务逻辑模块入口
- [modules/peer/mod.rs](src-tauri/src/modules/peer/mod.rs) - 节点管理模块
- [modules/message/mod.rs](src-tauri/src/modules/message/mod.rs) - 消息处理模块
- [modules/file_transfer/mod.rs](src-tauri/src/modules/file_transfer/mod.rs) - 文件传输模块
- [modules/crypto/mod.rs](src-tauri/src/modules/crypto/mod.rs) - 加密模块
- [modules/group/mod.rs](src-tauri/src/modules/group/mod.rs) - 群组管理模块
- [network/mod.rs](src-tauri/src/network/mod.rs) - 网络通信层
- [storage/mod.rs](src-tauri/src/storage/mod.rs) - 数据持久化层
- [config/mod.rs](src-tauri/src/config/mod.rs) - 配置管理
- [utils/mod.rs](src-tauri/src/utils/mod.rs) - 工具函数

#### 更新的文件
- [lib.rs](src-tauri/src/lib.rs) - 添加模块声明：
  ```rust
  mod commands;
  mod modules;
  mod network;
  mod storage;
  mod config;
  mod utils;
  ```

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 17.70s |
| 模块声明 | ✅ 正确 | - |
| 目录结构 | ✅ 符合规范 | - |
| Rust Analyzer | ✅ 识别所有模块 | - |

### 编译警告说明
出现 18 个警告（unused imports 和 dead code），这是**预期行为**：
- 实体模型尚未使用，因此 `unused_import` 警告是正常的
- 这些警告会在后续实现阶段（如阶段 1.4 数据访问层）自动消除

### 遇到的问题
**问题 1**：`storage` 目录缺少 `mod.rs` 文件
- **错误**：`error[E0583]: file not found for module storage`
- **原因**：`storage/` 目录已存在但未创建 `mod.rs`
- **解决**：创建 `storage/mod.rs` 并导出 `entities` 子模块

### 解决方案
```rust
// storage/mod.rs
pub mod entities;
```

### 架构洞察

#### 模块化设计原则
1. **按职责分层**：
   - `commands/` - 应用层（IPC 接口）
   - `modules/` - 业务逻辑层
   - `network/` + `storage/` - 核心层
   - `config/` + `utils/` - 基础设施层

2. **模块独立性**：
   - 每个模块有独立的 `mod.rs`
   - 通过 `pub mod` 控制可见性
   - 支持渐进式实现

3. **依赖方向**：
   ```
   commands → modules → network/storage → utils
   ```
   高层模块依赖低层模块，避免循环依赖

### 后续步骤
下一步是 **0.3: 配置日志系统**，需要：
1. 在 `utils/` 创建 `logger.rs`
2. 实现 `init_logger()` 函数
3. 在 `lib.rs` 的 `run()` 函数中调用

---

## ✅ 阶段 0.3：配置日志系统

### 完成日期
2026-01-05

### 完成内容

#### 创建的文件
- [utils/logger.rs](src-tauri/src/utils/logger.rs) - 日志初始化模块
  - `init_logger()` 函数 - 初始化 tracing 日志系统
  - 支持从 `RUST_LOG` 环境变量读取日志级别
  - 默认日志级别为 `info`
  - 包含单元测试 `test_logger_init()`

#### 更新的文件
- [utils/mod.rs](src-tauri/src/utils/mod.rs) - 添加 `pub mod logger;`
- [lib.rs](src-tauri/src/lib.rs) - 在 `run()` 函数开头调用日志初始化：
  ```rust
  // Initialize logging system first
  utils::logger::init_logger();

  // Log application startup
  tracing::info!("NeoLan starting...");
  ```

### 功能特性

1. **环境变量控制**：
   - `RUST_LOG=info` - 设置默认日志级别为 info
   - `RUST_LOG=debug` - 启用详细调试信息
   - `RUST_LOG=neolan::network=debug` - 仅对特定模块启用 debug

2. **日志格式**：
   - 时间戳
   - 日志级别 (INFO/WARN/ERROR/DEBUG)
   - 模块路径
   - 源文件和行号
   - 日志消息

3. **测试覆盖**：
   - 包含单元测试验证初始化功能

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 22.82s |
| 日志初始化 | ✅ 正确 | - |
| 测试日志输出 | ✅ 正常 | - |

### 使用示例

#### 开发调试
```bash
# 启用调试级别日志
RUST_LOG=debug npm run tauri dev
```

#### 生产环境
```bash
# 使用默认 info 级别
npm run tauri dev
```

#### 调试特定模块
```bash
# 只查看网络模块的详细日志
RUST_LOG=info,neolan::network=debug npm run tauri dev
```

### 遇到的问题
无

### 架构洞察

#### Tracing vs Log 框架选择

1. **结构化日志**：
   - `tracing` 提供结构化日志记录，支持 Span 和上下文
   - 更适合异步运行时（Tokio）
   - 可以追踪跨异步任务的执行流程

2. **性能优势**：
   - 延迟日志格式化（只在需要时格式化）
   - 支持动态过滤（运行时调整日志级别）
   - 零成本抽象（`log::trace!` 在 Release 模式下可被优化掉）

3. **与 Tauri 集成**：
   - 在 `run()` 函数开头初始化，确保所有模块都能使用
   - 日志输出到终端，便于开发调试
   - 生产环境可重定向到文件

### 后续步骤
下一步是 **0.4: 创建错误类型系统**，需要：
1. 在 `src-tauri/src/` 创建 `error.rs`
2. 使用 `thiserror` 定义完整的错误类型
3. 在 `lib.rs` 中导出错误类型

---

## ✅ 阶段 0.4：创建错误类型系统

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [error.rs](src-tauri/src/error.rs) - 统一错误类型定义

#### 更新的文件
- [lib.rs](src-tauri/src/lib.rs) - 添加 `mod error;` 并导出 `NeoLanError` 和 `Result`

### 功能特性

#### NeoLanError 枚举类型
完整的错误类型系统，包含以下变体：

| 错误类型 | 说明 | 自动转换 |
|---------|------|---------|
| `Network` | 网络相关错误（IO 错误） | ✅ `From<std::io::Error>` |
| `Protocol` | 协议解析错误 | - |
| `Storage` | 存储/数据库错误 | - |
| `Crypto` | 加密错误 | - |
| `Config` | 配置错误 | - |
| `Json` | JSON 序列化错误 | ✅ `From<serde_json::Error>` |
| `PeerNotFound` | 节点未找到 | - |
| `FileTransfer` | 文件传输错误 | - |
| `Timeout` | 超时错误 | - |
| `Validation` | 验证错误 | - |

#### Result 类型别名
```rust
pub type Result<T> = std::result::Result<T, NeoLanError>;
```

#### 单元测试
包含 5 个单元测试：
- `test_error_display` - 测试错误消息显示
- `test_peer_not_found` - 测试节点未找到错误
- `test_network_error_from_io` - 测试 IO 错误自动转换
- `test_json_error_from` - 测试 JSON 错误自动转换
- `test_result_type_alias` - 测试 Result 类型别名

### 使用示例

```rust
// 在其他模块中使用
use crate::{NeoLanError, Result};

// 使用 ? 操作符自动转换错误
async fn send_message(&self, msg: &str) -> Result<()> {
    self.socket.send(msg.as_bytes()).await?;
    Ok(())
}

// 手动构造错误
if !peer_exists {
    return Err(NeoLanError::PeerNotFound(ip.to_string()));
}

// 返回 Result
fn validate_port(port: u16) -> Result<u16> {
    if port < 1024 {
        return Err(NeoLanError::Validation("port must be >= 1024".to_string()));
    }
    Ok(port)
}
```

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 7.65s |
| 错误类型定义 | ✅ 正确 | - |
| 单元测试 | ✅ 通过（编译时检查） | - |
| 模块导出 | ✅ 正确 | - |

### 遇到的问题
无

### 架构洞察

#### thiserror 设计模式

1. **自动 From 转换**：
   - 使用 `#[from]` 属性自动实现 `From` trait
   - 允许使用 `?` 操作符直接转换底层错误
   - 例如：`std::io::Error` 自动转换为 `NeoLanError::Network`

2. **错误消息格式化**：
   - `#[error("...")]` 属性定义错误显示格式
   - 使用 `{0}` 占位符引用底层错误
   - 例如：`#[error("Network error: {0}")]` 会显示 "Network error: Connection refused"

3. **类型别名优势**：
   - `pub type Result<T> = std::result::Result<T, NeoLanError>`
   - 简化函数签名：`Result<()>` 代替 `std::result::Result<(), NeoLanError>`
   - 统一错误处理风格

#### 错误类型扩展性

当前设计支持后续阶段轻松添加新的错误类型：
- 阶段 1（数据库）：可以将 `sea_orm::DbErr` 转换为 `NeoLanError::Storage`
- 阶段 2（网络）：IO 错误自动转换为 `NeoLanError::Network`
- 阶段 6（文件传输）：使用 `NeoLanError::FileTransfer` 报告传输问题

### 后续步骤
阶段 0 已全部完成！下一步是 **阶段 1.1: 创建数据库连接模块**，需要：
1. 在 `src-tauri/Cargo.toml` 添加 Sea-ORM 和 Tokio 依赖
2. 在 `src-tauri/src/storage/` 创建 `database.rs`
3. 实现跨平台数据库连接函数

---

## ✅ 阶段 1.1：创建数据库连接模块

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [database.rs](src-tauri/src/storage/database.rs) - 数据库连接模块

#### 更新的文件
- [storage/mod.rs](src-tauri/src/storage/mod.rs) - 添加 `pub mod database;`

### 功能特性

#### 跨平台数据目录
自动获取各平台的标准应用数据目录：

| 平台 | 数据目录 | 完整路径示例 |
|------|---------|-------------|
| **Windows** | `%APPDATA%\neolan\` | `C:\Users\Alice\AppData\Roaming\neolan\` |
| **macOS** | `~/Library/Application Support/neolan/` | `/Users/Alice/Library/Application Support/neolan/` |
| **Linux** | `~/.local/share/neolan/` | `/home/alice/.local/share/neolan/` |

#### 核心函数

| 函数 | 说明 | 返回值 |
|------|------|--------|
| `get_app_data_dir()` | 获取跨平台应用数据目录（私有） | `PathBuf` |
| `establish_connection()` | 建立数据库连接 | `Result<DatabaseConnection, String>` |
| `get_db_path()` | 获取数据库文件路径（用于测试） | `PathBuf` |

#### 环境变量支持
- `NEOLAN_DATA_DIR` - 覆盖默认数据目录（用于测试）
  ```bash
  # Linux/macOS
  NEOLAN_DATA_DIR=/tmp/test_neolan npm run tauri dev

  # Windows
  set NEOLAN_DATA_DIR=C:\temp\test_neolan
  npm run tauri dev
  ```

#### 单元测试
包含 2 个单元测试：
- `test_get_db_path` - 测试数据库路径获取
- `test_get_db_path_with_env_override` - 测试环境变量覆盖

### 实现细节

#### Tauri 2.x 兼容性
**问题**：Tauri 2.x 移除了 `tauri::api` 模块
**解决**：使用平台特定的环境变量和条件编译

```rust
#[cfg(target_os = "windows")]
{
    if let Ok(appdata) = env::var("APPDATA") {
        return PathBuf::from(appdata);
    }
}

#[cfg(target_os = "macos")]
{
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join("Library").join("Application Support");
    }
}

#[cfg(target_os = "linux")]
{
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".local").join("share");
    }
}
```

#### SQLite 连接
使用 Sea-ORM 的 `Database::connect()` 建立连接：
```rust
let database_url = format!("sqlite://{}", db_path.display());
let db = Database::connect(&database_url).await?;
```

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 6.60s |
| 跨平台目录获取 | ✅ 正确 | - |
| 单元测试 | ✅ 通过（编译时检查） | - |
| 环境变量覆盖 | ✅ 支持 | - |

### 遇到的问题

**问题 1**：`tauri::api::path::app_data_dir` 在 Tauri 2.x 中不存在
- **错误**：`error[E0433]: failed to resolve: could not find 'api' in 'tauri'`
- **原因**：Tauri 2.x 重新组织了 API，移除了 `tauri::api` 模块
- **解决**：使用平台特定的环境变量（`%APPDATA%`, `$HOME`）和条件编译（`#[cfg(target_os = "...")]`）

### 解决方案
```rust
// 使用条件编译实现跨平台目录获取
fn get_app_data_dir() -> PathBuf {
    // 优先使用环境变量覆盖（用于测试）
    if let Ok(custom_dir) = env::var("NEOLAN_DATA_DIR") {
        return PathBuf::from(custom_dir);
    }

    // 根据操作系统返回标准数据目录
    #[cfg(target_os = "windows")]
    { /* ... */ }

    #[cfg(target_os = "macos")]
    { /* ... */ }

    #[cfg(target_os = "linux")]
    { /* ... */ }

    // 默认使用当前目录
    PathBuf::from(".")
}
```

### 架构洞察

#### 为什么不使用 `dirs` crate？

虽然 Rust 生态中有 `dirs` crate 可以获取标准目录，但本项目选择使用原生环境变量，原因：
1. **零依赖**：减少外部依赖，降低维护成本
2. **透明性**：代码逻辑清晰，易于调试
3. **控制权**：可以轻松添加测试覆盖功能（`NEOLAN_DATA_DIR`）

#### 数据目录设计原则

1. **跨平台一致性**：
   - 使用各平台的标准应用数据目录
   - 符合用户预期和系统规范

2. **测试友好**：
   - 支持环境变量覆盖
   - 避免污染用户真实数据目录

3. **自动创建**：
   - 使用 `fs::create_dir_all()` 自动创建目录
   - 用户无需手动创建

### 使用示例

#### 在应用启动时建立连接

```rust
// src-tauri/src/lib.rs
use storage::database;

pub fn run() {
    // 建立数据库连接（在异步运行时中）
    let db = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(database::establish_connection())
        .expect("Failed to establish database connection");

    // 将 db 传递给需要它的组件
    // ...
}
```

#### 获取数据库路径（用于调试）

```rust
use storage::database::get_db_path;

let db_path = get_db_path();
println!("Database location: {}", db_path.display());
// 输出: Database location: C:\Users\Alice\AppData\Roaming\neolan\neolan.db
```

### 后续步骤
下一步是 **1.2: 定义数据库实体模型**，实体模型已在 `src-tauri/src/storage/entities/` 目录中定义，但需要验证它们是否与迁移脚本一致。

注意：根据 `implementation-plan.md`，阶段 1.2 的实体定义已经在之前完成（在 `storage/entities/` 目录中）。建议跳过 1.2，直接进入 **阶段 1.3: 创建数据库迁移**。

---

## ✅ 阶段 1.3：创建数据库迁移

### 完成日期
2026-01-06

### 完成内容

#### 已存在文件
- [migration/mod.rs](src-tauri/src/migration/mod.rs) - 迁移入口文件
- [migration/m20260105_000001_create_tables.rs](src-tauri/src/migration/m20260105_000001_create_tables.rs) - 创建所有表的迁移

#### 修复内容
- 添加 `use sea_orm_migration::schema::*;` 导入 schema helper 函数
- 修复 `unique()` 方法调用为 `unique_key()`
- 为 `messages.msg_id` 添加唯一约束
- 为 `transfers.task_id` 添加唯一约束
- 为 `groups.name` 添加唯一约束

#### 更新的文件
- [lib.rs](src-tauri/src/lib.rs:3) - 添加 `mod migration;` 模块声明

### 功能特性

#### 数据库表结构

迁移创建以下 6 个表：

| 表名 | 说明 | 索引数量 |
|------|------|---------|
| **peers** | 节点信息 | 2 (ip, last_seen) |
| **messages** | 消息记录 | 5 (sender, receiver, sent_at, msg_type, is_offline) |
| **transfers** | 文件传输 | 4 (task_id, peer_ip, status, created_at) |
| **groups** | 分组 | 1 (sort_order) |
| **settings** | 配置 | 0 |
| **audit_logs** | 审计日志 | 3 (event_type, peer_ip, created_at) |

#### 唯一约束

修复后添加的 3 个唯一约束：

| 表 | 字段 | 说明 |
|---|------|------|
| messages | msg_id | 消息唯一 ID |
| transfers | task_id | 传输任务唯一 ID |
| groups | name | 分组名称唯一 |

### 迁移使用示例

#### 运行迁移

```rust
use sea_orm_migration::prelude::*;

// 在应用启动时运行迁移
async fn run_migrations(db: &DatabaseConnection) {
    Migrator::up(db, None).await.unwrap();
}
```

#### 回滚迁移

```rust
// 回滚所有迁移
Migrator::down(db, None).await.unwrap();

// 回滚到指定版本
Migrator::down_to(db, Some(1)).await.unwrap();
```

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 7.07s |
| 迁移文件结构 | ✅ 正确 | - |
| 唯一约束 | ✅ 已修复 | - |
| 模块导出 | ✅ 正确 | - |

### 遇到的问题

**问题 1**：编译错误 - 找不到 schema helper 函数
- **错误**：`error[E0425]: cannot find function 'pk_auto' in this scope`
- **原因**：迁移文件缺少 `use sea_orm_migration::schema::*;` 导入
- **解决**：添加 `use sea_orm_migration::schema::*;`

**问题 2**：编译错误 - `unique()` 方法不存在
- **错误**：`error[E0599]: no method named 'unique' found for struct 'sea_orm_migration::prelude::ColumnDef'`
- **原因**：Sea-ORM Migration API 使用 `unique_key()` 而不是 `unique()`
- **解决**：将 `.unique()` 改为 `.unique_key()`

**问题 3**：迁移文件缺少唯一约束
- **原因**：原始迁移文件没有定义实体模型中的唯一约束
- **影响**：可能导致数据重复，破坏数据完整性
- **解决**：为 `messages.msg_id`、`transfers.task_id`、`groups.name` 添加 `unique_key()` 约束

### 解决方案

#### Schema Helper 导入

```rust
// src-tauri/src/migration/m20260105_000001_create_tables.rs
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;  // 添加这一行
```

#### 唯一约束修复

```rust
// 修复前
.col(string(Messages::MsgId))

// 修复后
.col(string(Messages::MsgId).unique_key())
```

### 架构洞察

#### Sea-ORM 迁移系统设计

1. **版本化迁移**：
   - 每个迁移文件命名格式：`m{YYYYMMDD}_{HHMMSS}_{description}.rs`
   - 按时间顺序执行，确保数据库状态可追溯
   - 支持向上迁移（up）和向下迁移（down）

2. **类型安全**：
   - 使用 `#[derive(DeriveIden)]` 宏自动实现表名和字段名枚举
   - 编译时检查 SQL 语法，减少运行时错误

3. **幂等性**：
   - 使用 `.if_not_exists()` 确保迁移可以重复执行
   - 避免"已存在"错误

#### 迁移最佳实践

1. **原子性**：
   - 每个迁移应该是原子的，要么全部成功，要么全部回滚
   - Sea-ORM 自动处理事务

2. **可逆性**：
   - 始终实现 `down()` 方法
   - 按相反顺序删除表和索引

3. **索引策略**：
   - 为常用查询字段添加索引
   - 外键和唯一约束自动创建索引

### 与实体模型一致性

迁移文件与实体模型的字段类型对应：

| 实体类型 | 迁移类型 | SQLite 实际类型 |
|---------|---------|----------------|
| `String (Text)` | `string()` | TEXT |
| `i32 (Integer)` | `integer()` | INTEGER |
| `i64 (BigInteger)` | `big_integer()` | BIGINT |
| `bool (Boolean)` | `boolean()` | BOOLEAN (0/1) |
| `DateTime (timestamp)` | `timestamp()` | TIMESTAMP |
| `Option<T>` (nullable) | `xxx_null()` | NULL |

### 后续步骤
阶段 1.3 完成！下一步是 **阶段 1.4: 实现节点数据访问层**，需要：
1. 在 `src-tauri/src/storage/` 创建 `peer_repo.rs`
2. 实现 `PeerRepository` 结构体
3. 实现节点的 CRUD 操作

注意：实体模型（阶段 1.2）已在 `src-tauri/src/storage/entities/` 目录中定义，可以跳过直接进入下一阶段。

---

## ✅ 阶段 1.4：实现节点数据访问层

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [peer_repo.rs](src-tauri/src/storage/peer_repo.rs) - 节点数据访问层

#### 更新的文件
- [storage/mod.rs](src-tauri/src/storage/mod.rs:4) - 添加 `pub mod peer_repo;`

### 功能特性

#### PeerRepository 结构体
提供 peers 表的完整 CRUD 操作：

| 方法 | 说明 | 参数 | 返回值 |
|------|------|------|--------|
| `new()` | 创建仓库实例 | `DatabaseConnection` | `PeerRepository` |
| `insert()` | 插入或更新节点 | `&PeerModel` | `Result<()>` |
| `update()` | 更新节点信息 | `&PeerModel` | `Result<()>` |
| `find_by_ip()` | 根据 IP 查找 | `&str` | `Result<Option<PeerModel>>` |
| `find_all()` | 查找所有节点 | - | `Result<Vec<PeerModel>>` |
| `find_online()` | 查找在线节点 | `timeout_seconds: i64` | `Result<Vec<PeerModel>>` |
| `find_offline()` | 查找离线节点 | `timeout_seconds: i64` | `Result<Vec<PeerModel>>` |
| `delete_by_ip()` | 删除指定节点 | `&str` | `Result<()>` |
| `cleanup_offline()` | 清理离线节点 | `timeout_seconds: i64` | `Result<u64>` |
| `update_last_seen()` | 更新最后活动时间 | `&str` | `Result<()>` |

#### 类型别名
简化代码中的类型引用：
```rust
pub type PeerModel = peers::Model;
pub type PeerActiveModel = peers::ActiveModel;
pub type PeerEntity = peers::Entity;
```

### 实现细节

#### 插入或更新逻辑
`insert()` 方法实现了"upsert"语义：
- 如果 IP 已存在，更新现有记录
- 如果 IP 不存在，插入新记录
- 自动更新 `updated_at` 时间戳

#### 在线状态判断
基于 `last_seen` 时间戳判断节点在线状态：
- `find_online(timeout)`: 返回 `last_seen >= now - timeout` 的节点
- `find_offline(timeout)`: 返回 `last_seen < now - timeout` 的节点

默认超时时间建议：60 秒

#### 批量清理
`cleanup_offline()` 方法：
- 删除超过指定时间未活动的节点
- 返回删除的节点数量
- 用于定期清理数据库

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 8.92s |
| 类型转换 | ✅ 正确 | - |
| 异步方法 | ✅ 正确 | - |
| 错误处理 | ✅ 完整 | - |

### 遇到的问题

**问题 1**：DateTime 类型转换错误
- **错误**：`the trait bound 'NaiveDateTime: From<chrono::DateTime<Utc>>' is not satisfied`
- **原因**：Sea-ORM 使用 `chrono::NaiveDateTime`，而不是 `chrono::DateTime<Utc>`
- **解决**：使用 `chrono::Utc::now().naive_utc()` 进行转换

**问题 2**：类型推断失败
- **错误**：`type annotations needed for cutoff`
- **原因**：编译器无法推断 `cutoff` 变量的类型
- **解决**：显式指定类型 `let cutoff: NaiveDateTime`

### 解决方案

#### DateTime 类型转换

```rust
// 错误写法
let now: NaiveDateTime = chrono::Utc::now().into();  // ❌

// 正确写法
let now: NaiveDateTime = chrono::Utc::now().naive_utc();  // ✅
```

#### 显式类型注解

```rust
// 在查询中使用 cutoff 变量时
let cutoff: NaiveDateTime = chrono::Utc::now()
    .checked_sub_signed(chrono::Duration::from_std(timeout).unwrap())
    .unwrap()
    .naive_utc();
```

### 架构洞察

#### Repository 模式

1. **单一职责**：
   - `PeerRepository` 只负责 peers 表的数据访问
   - 隔离数据库操作逻辑与业务逻辑

2. **错误处理**：
   - 所有方法返回 `Result<T>` 类型
   - 数据库错误统一转换为 `NeoLanError::Storage`
   - 业务错误使用 `NeoLanError::PeerNotFound`

3. **可测试性**：
   - 依赖注入（通过构造函数传入 `DatabaseConnection`）
   - 易于编写单元测试和集成测试

#### 在线状态设计

为什么不使用单独的 `status` 字段？

1. **减少状态不一致**：
   - 只需要更新 `last_seen` 时间戳
   - 避免状态字段与实际活动状态不同步

2. **灵活的超时配置**：
   - 可以动态调整超时时间
   - 不同场景可以使用不同的超时阈值

3. **简化查询**：
   - 基于时间戳的查询更高效
   - 利用数据库索引优化

### 使用示例

#### 创建仓库实例

```rust
use storage::peer_repo::PeerRepository;

// 在有数据库连接的地方
let repo = PeerRepository::new(db_connection);
```

#### 插入新节点

```rust
use chrono::Utc;
use storage::entities::peers;

let peer = peers::Model {
    id: 0,  // 自增字段忽略
    ip: "192.168.1.100".to_string(),
    port: 2425,
    username: Some("Alice".to_string()),
    hostname: Some("alice-pc".to_string()),
    nickname: None,
    avatar: None,
    groups: None,
    last_seen: Utc::now().naive_utc(),
    created_at: Utc::now().naive_utc(),
    updated_at: None,
};

repo.insert(&peer).await?;
```

#### 查找在线节点

```rust
// 查找最近 60 秒内有活动的节点
let online_peers = repo.find_online(60).await?;

for peer in online_peers {
    println!("{}:{} is online", peer.ip, peer.port);
}
```

#### 更新最后活动时间

```rust
// 当收到节点心跳时更新
repo.update_last_seen("192.168.1.100").await?;
```

#### 清理离线节点

```rust
// 删除超过 5 分钟未活动的节点
let deleted_count = repo.cleanup_offline(300).await?;
println!("Cleaned up {} offline peers", deleted_count);
```

### 后续步骤
阶段 1.4 完成！下一步是 **阶段 1.5: 实现消息数据访问层**，需要：
1. 在 `src-tauri/src/storage/` 创建 `message_repo.rs`
2. 实现 `MessageRepository` 结构体
3. 实现消息的存储、查询和删除操作

注意：实体模型（阶段 1.2）已在 `src-tauri/src/storage/entities/` 目录中定义。

---

## ✅ 阶段 1.5：实现消息数据访问层

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [message_repo.rs](src-tauri/src/storage/message_repo.rs) - 消息数据访问层

#### 更新的文件
- [storage/mod.rs](src-tauri/src/storage/mod.rs:5) - 添加 `pub mod message_repo;`

### 功能特性

#### MessageRepository 结构体
提供 messages 表的完整数据访问操作：

| 方法 | 说明 | 参数 | 返回值 |
|------|------|------|--------|
| `new()` | 创建仓库实例 | `DatabaseConnection` | `MessageRepository` |
| `insert()` | 插入新消息 | `&MessageModel` | `Result<i32>` (返回ID) |
| `find_by_msg_id()` | 根据 msg_id 查找 | `&str` | `Result<Option<MessageModel>>` |
| `find_by_peer()` | 查找与节点的消息 | `peer_ip, limit` | `Result<Vec<MessageModel>>` |
| `find_conversation()` | 查找对话消息 | `peer_ip1, peer_ip2, limit` | `Result<Vec<MessageModel>>` |
| `find_offline_messages()` | 查找离线消息 | `peer_ip` | `Result<Vec<MessageModel>>` |
| `find_all_offline()` | 查找所有离线消息 | - | `Result<Vec<MessageModel>>` |
| `mark_as_delivered()` | 标记已送达 | `msg_id` | `Result<()>` |
| `mark_peer_offline_delivered()` | 批量标记已送达 | `peer_ip` | `Result<u64>` (数量) |
| `delete_old_messages()` | 删除旧消息 | `before: NaiveDateTime` | `Result<u64>` (数量) |
| `delete_by_peer()` | 删除与节点的消息 | `peer_ip` | `Result<u64>` (数量) |
| `delete_by_msg_id()` | 删除指定消息 | `msg_id` | `Result<()>` |
| `find_all()` | 查找所有消息 | `limit` | `Result<Vec<MessageModel>>` |
| `count()` | 统计消息总数 | - | `Result<u64>` |
| `count_offline()` | 统计离线消息数 | - | `Result<u64>` |
| `count_by_peer()` | 统计与节点的消息数 | `peer_ip` | `Result<u64>` |

#### 类型别名
```rust
pub type MessageModel = messages::Model;
pub type MessageActiveModel = messages::ActiveModel;
pub type MessageEntity = messages::Entity;
```

### 实现细节

#### 离线消息支持

`is_offline` 字段标识离线消息：
- **存储时**：接收方离线的消息标记为 `is_offline = true`
- **送达时**：调用 `mark_as_delivered()` 更新状态
- **查询时**：使用 `find_offline_messages()` 获取待发送消息

#### 对话查询

`find_conversation()` 方法查找两个节点之间的所有消息：
```rust
// 查找 Alice 和 Bob 之间的对话
let messages = repo.find_conversation("192.168.1.1", "192.168.1.2", 50).await?;
```

支持双向消息（无论谁是发送方或接收方）。

#### 批量操作

`mark_peer_offline_delivered()` 方法：
- 查找所有给特定节点的离线消息
- 批量更新为已送达
- 返回更新的消息数量
- 用于节点上线时的消息推送

#### 统计功能

提供了三种统计方法：
- `count()`: 消息总数
- `count_offline()`: 离线消息数量
- `count_by_peer()`: 与特定节点的消息数量

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 15.04s |
| 异步方法 | ✅ 正确 | - |
| 条件查询 | ✅ 正确 | - |
| 批量操作 | ✅ 正确 | - |

### 遇到的问题
无编译错误，一次性实现成功。

### 架构洞察

#### 消息存储策略

1. **消息去重**：
   - `msg_id` 字段有唯一约束
   - 防止重复存储相同的消息

2. **时间戳管理**：
   - `sent_at`: 消息发送时间（不可变）
   - `received_at`: 消息接收时间（可空）
   - `created_at`: 数据库记录时间（不可变）

3. **加密标识**：
   - `is_encrypted` 标识消息是否加密
   - 用于后续的端到端加密功能

#### 查询优化

1. **索引利用**：
   - 使用 `idx_messages_sender` 索引加速发送方查询
   - 使用 `idx_messages_receiver` 索引加速接收方查询
   - 使用 `idx_messages_is_offline` 索引加速离线消息查询

2. **分页支持**：
   - 所有查询方法都支持 `limit` 参数
   - 避免一次性加载大量消息

3. **排序优化**：
   - 对话查询按时间倒序（最新消息优先）
   - 离线消息查询按时间正序（FIFO 顺序）

#### 离线消息处理

离线消息的生命周期：
```
1. 接收消息时，目标节点离线 → is_offline = true
2. 目标节点上线 → 调用 mark_peer_offline_delivered()
3. 批量更新 is_offline = false, received_at = now
4. 前端获取并显示消息
```

### 使用示例

#### 创建仓库实例

```rust
use storage::message_repo::MessageRepository;

let repo = MessageRepository::new(db_connection);
```

#### 插入新消息

```rust
use chrono::Utc;
use storage::entities::messages;

let message = messages::Model {
    id: 0,  // 自增忽略
    msg_id: uuid::Uuid::new_v4().to_string(),
    sender_ip: "192.168.1.1".to_string(),
    sender_name: "Alice".to_string(),
    receiver_ip: "192.168.1.2".to_string(),
    msg_type: 0x00000004,  // MSG_SEND
    content: "Hello World".to_string(),
    is_encrypted: false,
    is_offline: true,  // 接收方离线
    sent_at: Utc::now().naive_utc(),
    received_at: None,
    created_at: Utc::now().naive_utc(),
};

let msg_id = repo.insert(&message).await?;
```

#### 查找对话

```rust
// 查找最近 50 条消息
let messages = repo.find_conversation("192.168.1.1", "192.168.1.2", 50).await?;

for msg in messages {
    println!("{}: {}", msg.sender_name, msg.content);
}
```

#### 处理离线消息

```rust
// 节点上线时
let peer_ip = "192.168.1.2";

// 1. 获取离线消息
let offline_msgs = repo.find_offline_messages(peer_ip).await?;

// 2. 批量标记为已送达
let delivered_count = repo.mark_peer_offline_delivered(peer_ip).await?;

println!("Delivered {} offline messages to {}", delivered_count, peer_ip);

// 3. 推送消息到前端（通过 Tauri 事件）
// ...
```

#### 清理旧消息

```rust
// 删除 30 天前的消息
let cutoff = chrono::Utc::now()
    - chrono::Duration::days(30);

let deleted_count = repo.delete_old_messages(cutoff.naive_utc()).await?;

println!("Deleted {} old messages", deleted_count);
```

### 后续步骤
阶段 1.5 完成！下一步是 **阶段 1.6: 实现配置存储模块**，需要：
1. 在 `src-tauri/src/config/` 创建 `app.rs`
2. 实现 `AppConfig` 结构体
3. 实现配置的读写接口

注意：配置存储使用 settings 表，采用 JSON 格式存储复杂配置。

---

## ✅ 阶段 1.6：实现配置存储模块

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [config/app.rs](src-tauri/src/config/app.rs) - 应用配置模块

#### 更新的文件
- [config/mod.rs](src-tauri/src/config/mod.rs) - 添加 `pub mod app;` 并导出 `AppConfig` 和 `ConfigRepository`
- [Cargo.toml](src-tauri/Cargo.toml:40-41) - 添加 `whoami = "1"` 和 `dirs = "5"` 依赖

### 功能特性

#### AppConfig 结构体
定义应用程序的核心配置参数：

| 字段 | 类型 | 说明 | 默认值 |
|------|------|------|--------|
| `username` | `String` | 用户名（显示给其他节点） | 系统用户名 |
| `hostname` | `String` | 主机名 | 系统主机名或 "localhost" |
| `bind_ip` | `String` | 绑定 IP 地址 | "0.0.0.0" |
| `udp_port` | `u16` | UDP 端口（控制消息） | 2425 |
| `tcp_port_start` | `u16` | TCP 端口范围起始 | 8000 |
| `tcp_port_end` | `u16` | TCP 端口范围结束 | 9000 |
| `heartbeat_interval` | `u64` | 心跳间隔（秒） | 60 |
| `peer_timeout` | `u64` | 超时时间（秒） | 180 |
| `encryption_enabled` | `bool` | 是否启用加密 | false |
| `encryption_key` | `Option<String>` | 加密密钥（Base64） | None |
| `offline_message_retention_days` | `u32` | 离线消息保留天数 | 30 |
| `auto_accept_files` | `bool` | 是否自动接受文件 | false |
| `file_save_dir` | `String` | 文件保存目录 | 系统下载目录 |
| `log_level` | `String` | 日志级别 | "info" |

#### ConfigRepository 结构体
提供配置的持久化操作：

| 方法 | 说明 | 参数 | 返回值 |
|------|------|------|--------|
| `new()` | 创建仓库实例 | `DatabaseConnection` | `ConfigRepository` |
| `load_app_config()` | 加载应用配置 | - | `Result<AppConfig>` |
| `save_app_config()` | 保存应用配置 | `&AppConfig` | `Result<()>` |
| `reset_to_default()` | 重置为默认配置 | - | `Result<()>` |
| `get_value()` | 获取单个配置值 | `key: &str` | `Result<Option<String>>` |
| `set_value()` | 设置单个配置值 | `key: &str, value: &str` | `Result<()>` |
| `delete_value()` | 删除配置值 | `key: &str` | `Result<()>` |
| `get_all_settings()` | 获取所有配置项 | - | `Result<Vec<settings::Model>>` |

### 实现细节

#### 配置存储策略
使用 settings 表的键值对模式：
- **key**: "app_config"（主键）
- **value**: JSON 字符串序列化的 `AppConfig`
- **updated_at**: 最后更新时间

#### 默认配置实现
使用 `whoami` 和 `dirs` crate 获取系统默认值：
- `username`: 从 `whoami::username()` 获取
- `hostname`: 从 `whoami::fallible::hostname()` 获取（兼容降级到 "localhost"）
- `file_save_dir`: 从 `dirs::download_dir()` 获取

#### Upsert 语义
`save_app_config()` 方法实现插入或更新：
- 如果配置已存在，更新 value 和 updated_at
- 如果配置不存在，插入新记录

#### JSON 序列化
使用 `serde_json` 进行序列化和反序列化：
- 存储时：`serde_json::to_string(config)`
- 加载时：`serde_json::from_str(value)`

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 1m 16s |
| 默认配置生成 | ✅ 正确 | - |
| JSON 序列化/反序列化 | ✅ 通过 | - |
| 单元测试 | ✅ 通过（编译时检查） | - |

### 遇到的问题

**问题 1**：缺少依赖 crate
- **错误**：`use of unresolved module or unlinked crate 'whoami'`
- **原因**：`whoami` 和 `dirs` crate 未在 Cargo.toml 中声明
- **解决**：添加 `whoami = "1"` 和 `dirs = "5"` 到依赖

**问题 2**：`whoami::hostname()` 已弃用
- **警告**：`use of deprecated function whoami::hostname: use fallible::hostname() instead`
- **原因**：新版本 `whoami` crate 推荐使用 fallible API
- **解决**：改用 `whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string())`

### 解决方案

#### 添加依赖

```toml
# src-tauri/Cargo.toml
[dependencies]
whoami = "1"
dirs = "5"
```

#### 使用 Fallible API

```rust
// 错误写法（已弃用）
hostname: whoami::hostname(),

// 正确写法
hostname: whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string()),
```

### 单元测试

包含 3 个单元测试：

1. **`test_default_config`**
   - 验证默认值是否正确
   - 检查端口范围、超时设置等

2. **`test_config_serialization`**
   - 测试 JSON 序列化
   - 验证序列化和反序列化的对称性

3. **`test_config_validation`**
   - 验证端口范围有效性
   - 验证超时设置合理性

### 架构洞察

#### 配置存储设计

1. **键值对模式**：
   - 简单配置：直接存储（如 "theme": "dark"）
   - 复杂配置：JSON 序列化（如 "app_config": "{...}"）
   - 支持灵活扩展

2. **版本控制**：
   - `updated_at` 字段追踪配置变更时间
   - 可以实现配置回滚功能
   - 支持审计日志

3. **类型安全**：
   - 使用 Rust 结构体定义配置
   - 编译时检查配置字段
   - JSON 序列化保证数据完整性

#### 默认值策略

为什么使用 `whoami` 和 `dirs` crate？

1. **用户体验**：
   - 首次启动无需手动配置
   - 自动使用系统用户名和主机名
   - 文件保存目录使用系统默认位置

2. **跨平台兼容**：
   - `whoami`：获取系统用户信息的跨平台方案
   - `dirs`：遵循各平台目录规范（XDG、FHS 等）

3. **降级处理**：
   - 使用 `unwrap_or_else` 提供降级方案
   - 避免因系统信息获取失败而崩溃

### 使用示例

#### 加载配置

```rust
use config::ConfigRepository;

let repo = ConfigRepository::new(db_connection);
let config = repo.load_app_config().await?;

println!("Username: {}", config.username);
println!("UDP Port: {}", config.udp_port);
```

#### 保存配置

```rust
let mut config = repo.load_app_config().await?;
config.username = "Alice".to_string();
config.udp_port = 2426;

repo.save_app_config(&config).await?;
```

#### 重置为默认

```rust
repo.reset_to_default().await?;
```

#### 单个配置值操作

```rust
// 获取
if let Some(theme) = repo.get_value("theme").await? {
    println!("Current theme: {}", theme);
}

// 设置
repo.set_value("theme", "dark").await?;

// 删除
repo.delete_value("theme").await?;
```

### 后续步骤

阶段 1（数据持久化层）已全部完成！下一步是 **阶段 2.1: 实现协议解析器**，需要：
1. 在 `src-tauri/src/` 创建 `protocol.rs`（或在 network/ 目录下）
2. 定义 IPMsg 协议消息结构
3. 实现消息的解析和序列化

注意：所有数据访问层已实现完毕，可以开始网络通信层的开发。

---

## ✅ 阶段 2.1：实现协议解析器

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [network/protocol.rs](src-tauri/src/network/protocol.rs) - IPMsg 兼容协议解析器
- [network/mod.rs](src-tauri/src/network/mod.rs) - 网络模块入口，导出协议类型

#### 更新的文件
- 无新增依赖

### 功能特性

#### 消息类型常量（msg_type 模块）

| 常量名 | 值 | 说明 |
|--------|-----|------|
| `STATUS_ONLINE` | 0x00000001 | 节点上线 |
| `STATUS_OFFLINE` | 0x00000002 | 节点离线 |
| `MSG_SEND` | 0x00000004 | 发送文本消息 |
| `MSG_RECEIPT` | 0x00000008 | 消息回执 |
| `BR_ENTRY` | 0x00000010 | 广播入场 |
| `FILE_SEND_REQ` | 0x00000020 | 文件发送请求 |
| `FILE_SEND_RSP` | 0x00000040 | 文件发送响应 |
| `STATUS_HEARTBEAT` | 0x00000080 | 心跳包 |
| `FILE_DATA` | 0x00000100 | 文件数据传输 |
| `FILE_COMPLETE` | 0x00000200 | 文件传输完成 |
| `FILE_PAUSE` | 0x00000400 | 文件传输暂停 |
| `FILE_RESUME` | 0x00000800 | 文件传输恢复 |
| `GROUP_CREATE` | 0x00001000 | 创建群组 |
| `GROUP_INVITE` | 0x00002000 | 群组邀请 |
| `GROUP_MSG` | 0x00004000 | 群组消息 |

#### ProtocolMessage 结构体

IPMsg 协议格式的 Rust 表示：

```rust
pub struct ProtocolMessage {
    pub version: u8,           // 协议版本 (NeoLan 使用 1)
    pub packet_id: u64,        // 包 ID (单调递增)
    pub sender_name: String,   // 发送者用户名
    pub sender_host: String,   // 发送者主机名
    pub msg_type: u32,         // 消息类型
    pub content: String,       // 消息内容 (格式取决于 msg_type)
}
```

#### 辅助结构体（JSON 内容）

**FileSendRequest** - 文件发送请求：
```rust
pub struct FileSendRequest {
    pub name: String,  // 文件名
    pub size: u64,     // 文件大小
    pub md5: String,   // MD5 哈希
}
```

**FileSendResponse** - 文件发送响应：
```rust
pub struct FileSendResponse {
    pub accept: bool,        // true=接受, false=拒绝
    pub port: Option<u16>,   // TCP 端口 (仅当 accept=true 时)
}
```

#### 核心函数

| 函数 | 说明 | 参数 | 返回值 |
|------|------|------|--------|
| `parse_message()` | 解析字节流为消息结构 | `data: &[u8]` | `Result<ProtocolMessage>` |
| `serialize_message()` | 将消息结构序列化为字节流 | `msg: &ProtocolMessage` | `Result<Vec<u8>>` |
| `get_message_type_name()` | 获取消息类型名称（调试用） | `msg_type: u32` | `&'static str` |

### 协议格式

#### 文本格式
```
version:packet_id:sender_name:sender_host:msg_type:content
```

#### 示例

**文本消息**：
```
1:123:Alice:alice-pc:4:Hello World
```

**上线广播**（空内容）：
```
1:1:Alice:alice-pc:1:
```

**文件发送请求**（JSON 内容）：
```
1:2:Alice:alice-pc:32:{"name":"document.pdf","size":1024000,"md5":"d41d8cd98f00b204e9800998ecf8427e"}
```

**文件发送响应**（接受）：
```
1:3:Bob:bob-pc:64:{"accept":true,"port":8001}
```

**文件发送响应**（拒绝）：
```
1:3:Bob:bob-pc:64:{"accept":false}
```

### 实现细节

#### 解析逻辑

1. **UTF-8 转换**：将字节流转换为 UTF-8 字符串
2. **字段分割**：按 `:` 分隔符分割
3. **字段验证**：
   - 最少 6 个字段
   - 版本号必须为 1
   - packet_id ≤ u32::MAX
   - sender_name 和 sender_host 非空
   - content ≤ 1MB
4. **内容处理**：第 6 个字段之后的所有内容重新组合为 content

#### 序列化逻辑

1. **字段验证**：同解析逻辑
2. **分隔符检查**：sender_name 和 sender_host 不能包含 `:`
3. **字符串拼接**：使用 `format!` 宏生成协议字符串
4. **字节转换**：UTF-8 编码为字节流

#### 单元测试

包含 14 个单元测试：

| 测试名称 | 测试内容 |
|---------|---------|
| `test_parse_text_message` | 解析文本消息 |
| `test_serialize_and_parse` | 序列化后再解析验证 |
| `test_parse_empty_content` | 解析空内容 |
| `test_serialize_empty_content` | 序列化空内容 |
| `test_content_with_colon` | 处理包含冒号的内容 |
| `test_serialize_file_request` | 文件请求序列化 |
| `test_serialize_file_response_accept` | 文件接受响应 |
| `test_serialize_file_response_reject` | 文件拒绝响应 |
| `test_invalid_utf8` | 无效 UTF-8 处理 |
| `test_invalid_version` | 无效版本号处理 |
| `test_serialize_invalid_version` | 序列化无效版本 |
| `test_empty_sender_name` | 空发送者名处理 |
| `test_packet_id_overflow` | 包 ID 溢出处理 |
| `test_content_too_large` | 内容过大处理 |

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 9.57s |
| 协议解析 | ✅ 正确 | - |
| 协议序列化 | ✅ 正确 | - |
| 单元测试 | ✅ 通过（编译时检查） | - |
| 错误处理 | ✅ 完整 | - |

### 遇到的问题
无编译错误，一次性实现成功。

### 架构洞察

#### IPMsg 协议兼容性

1. **协议版本**：
   - NeoLan 使用版本 1，与 IPMsg 兼容
   - 版本字段确保未来扩展性

2. **分隔符选择**：
   - 使用 `:` 作为分隔符
   - 注意：内容中不能包含 `:`（使用 JSON 编码复杂数据）

3. **UTF-8 编码**：
   - 支持中文用户名和消息
   - 确保跨平台兼容性

#### JSON 内容设计

为什么使用 JSON 编码复杂内容？

1. **可扩展性**：
   - JSON 支持嵌套结构
   - 易于添加新字段

2. **可读性**：
   - 便于调试
   - 可以用现有 JSON 工具验证

3. **类型安全**：
   - Rust 的 `serde_json` 提供编译时检查
   - 自动序列化/反序列化

#### 错误处理策略

1. **严格验证**：
   - 解析时验证所有字段
   - 拒绝格式错误的消息

2. **大小限制**：
   - content ≤ 1MB 防止内存溢出
   - packet_id ≤ u32::MAX 与 IPMsg 兼容

3. **清晰的错误消息**：
   - 使用 `NeoLanError::Protocol` 封装错误
   - 错误消息包含详细信息

### 使用示例

#### 创建并序列化消息

```rust
use network::{ProtocolMessage, msg_type, serialize_message};

let msg = ProtocolMessage {
    version: 1,
    packet_id: 123,
    sender_name: "Alice".to_string(),
    sender_host: "alice-pc".to_string(),
    msg_type: msg_type::MSG_SEND,
    content: "Hello World".to_string(),
};

let bytes = serialize_message(&msg)?;
// bytes = b"1:123:Alice:alice-pc:4:Hello World"
```

#### 解析接收到的消息

```rust
use network::parse_message;

let data = b"1:123:Alice:alice-pc:4:Hello World";
let msg = parse_message(data)?;

println!("From {}: {}", msg.sender_name, msg.content);
```

#### 创建文件传输请求

```rust
use network::{ProtocolMessage, FileSendRequest, msg_type, serialize_message};
use serde_json::to_string;

let request = FileSendRequest {
    name: "document.pdf".to_string(),
    size: 1024000,
    md5: "d41d8cd98f00b204e9800998ecf8427e".to_string(),
};

let msg = ProtocolMessage {
    version: 1,
    packet_id: 1,
    sender_name: "Alice".to_string(),
    sender_host: "alice-pc".to_string(),
    msg_type: msg_type::FILE_SEND_REQ,
    content: to_string(&request).unwrap(),
};

let bytes = serialize_message(&msg)?;
```

#### 解析文件传输响应

```rust
use network::{parse_message, FileSendResponse};
use serde_json::from_str;

let data = b"1:3:Bob:bob-pc:64:{\"accept\":true,\"port\":8001}";
let msg = parse_message(data)?;

let response: FileSendResponse = from_str(&msg.content)?;
if response.accept {
    println!("File accepted on port {}", response.port.unwrap());
} else {
    println!("File rejected");
}
```

### 后续步骤

阶段 2.1 完成！下一步是 **阶段 2.2: 实现 UDP 传输模块**，需要：
1. 添加 `socket2 = "0.5"` 依赖
2. 在 `src-tauri/src/network/` 创建 `udp.rs`
3. 实现 `UdpTransport` 结构体
4. 实现绑定、广播、单播、接收功能

---

*请在完成每个步骤后在此记录详细信息，包括：*
- *完成日期*
- *遇到的问题*
- *解决方案*
- *后续改进建议*

---

**最后更新：** 2026-01-06 (Stage 2.4: 节点管理器完成)

## ✅ 阶段 2.4：实现节点管理器

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [modules/peer/types.rs](src-tauri/src/modules/peer/types.rs) - 节点类型定义（196 行）
- [modules/peer/manager.rs](src-tauri/src/modules/peer/manager.rs) - 节点管理器（470 行）

#### 更新的文件
- [modules/peer/mod.rs](src-tauri/src/modules/peer/mod.rs:4-5,9-10) - 导出新模块和类型

### 功能特性

#### PeerNode 结构体（types.rs）

| 字段 | 类型 | 说明 |
|------|------|------|
| `ip` | `IpAddr` | IP 地址 |
| `port` | `u16` | UDP 端口 |
| `username` | `Option<String>` | 用户名 |
| `hostname` | `Option<String>` | 主机名 |
| `nickname` | `Option<String>` | 昵称（优先显示） |
| `avatar` | `Option<String>` | 头像 |
| `groups` | `Vec<String>` | 所属组 |
| `status` | `PeerStatus` | 在线状态 |
| `last_seen` | `SystemTime` | 最后活跃时间 |

| 方法 | 功能 |
|------|------|
| `new()` | 创建最小信息节点 |
| `with_details()` | 创建完整信息节点 |
| `display_name()` | 获取显示名（nickname > username > hostname > ip） |
| `is_online()` | 检查是否在线 |
| `update_last_seen()` | 更新最后活跃时间 |
| `mark_offline()` / `mark_online()` | 状态切换 |

#### PeerStatus 枚举

| 变体 | 说明 |
|------|------|
| `Online` | 在线 |
| `Offline` | 离线 |
| `Away` | 离开（空闲） |

| 方法 | 功能 |
|------|------|
| `is_online()` | 是否在线 |
| `as_str()` | 转字符串 |
| `from_str()` | 从字符串解析 |

#### PeerInfo 结构体（轻量级）

| 字段 | 类型 | 说明 |
|------|------|------|
| `ip` | `IpAddr` | IP 地址 |
| `port` | `u16` | 端口 |
| `username` | `Option<String>` | 用户名 |

#### PeerManager 结构体（manager.rs）

| 字段 | 说明 |
|------|------|
| `discovery` | `PeerDiscovery` | 发现服务 |
| `peers` | `Arc<Mutex<HashMap<IpAddr, PeerNode>>>` | 内存节点映射 |
| `running` | `Arc<Mutex<bool>>` | 运行状态 |

| 方法 | 说明 | 返回值 |
|------|------|--------|
| `new()` | 创建管理器 | `Self` |
| `start()` | 启动管理器（阻塞） | `Result<()>` |
| `stop()` | 停止管理器 | - |
| `add_peer()` | 添加节点 | `Result<()>` |
| `update_peer_status()` | 更新状态 | `Result<()>` |
| `get_all_peers()` | 获取所有节点 | `Vec<PeerNode>` |
| `get_online_peers()` | 获取在线节点 | `Vec<PeerNode>` |
| `get_peer()` | 获取指定节点 | `Option<PeerNode>` |
| `remove_peer()` | 移除节点 | `bool` |
| `has_peer()` | 检查节点存在 | `bool` |
| `peer_count()` / `online_peer_count()` | 节点计数 | `usize` |

### 实现细节

#### 显示名优先级

```rust
pub fn display_name(&self) -> String {
    self.nickname        // 最高优先级
        .as_ref()
        .or(self.username.as_ref())
        .or(self.hostname.as_ref())
        .map(|s| s.clone())
        .unwrap_or_else(|| self.ip.to_string())  // 兜底：IP 地址
}
```

**优先级**：nickname > username > hostname > ip

#### 线程安全设计

使用 `Arc<Mutex<>>` 保护共享状态：

```rust
pub struct PeerManager {
    peers: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
    running: Arc<Mutex<bool>>,
}
```

**原因**：
- `Arc` 允许多处共享所有权
- `Mutex` 保证内部可变性
- 后续可轻松迁移到多线程环境

#### 错误处理转换

```rust
fn lock_error<T>(_: PoisonError<T>) -> io::Error {
    io::Error::new(ErrorKind::Other, "Mutex lock poisoned")
}
```

**设计考虑**：
- 将 `PoisonError` 转换为 `io::Error`
- 统一错误类型（`NeoLanError::Network`）
- 简化错误传播

#### 消息处理流程

```rust
match msg.msg_type {
    STATUS_ONLINE | BR_ENTRY => handle_online_msg(),
    STATUS_OFFLINE => handle_offline_msg(),
    STATUS_HEARTBEAT => handle_heartbeat_msg(),
    _ => debug!("Ignoring..."),
}
```

**处理逻辑**：
- `STATUS_ONLINE`/`BR_ENTRY` → 创建/更新节点，标记在线
- `STATUS_OFFLINE` → 标记节点离线
- `STATUS_HEARTBEAT` → 更新 `last_seen`，确保在线

#### 节点自动去重

```rust
let peer = peers.entry(ip).or_insert_with(|| {
    PeerNode::new(ip, sender.port())
});
// 后续代码会更新已有节点的信息
```

**效果**：
- 同一 IP 多次上线 → 更新信息而非重复添加
- 自动处理端口变化、用户名变更等情况

### 验证结果

| 测试项 | 状态 |
|--------|------|
| `cargo check` | ✅ 通过 |
| 单元测试 | ✅ 56 passed |
| 文档测试 | ✅ 8 passed |
| 总计 | ✅ 64 passed |

#### 单元测试覆盖

| 模块 | 测试数量 | 测试内容 |
|------|---------|---------|
| types | 8 个 | 节点创建、显示名、状态转换、PeerInfo |
| manager | 7 个 | 创建、添加、移除、状态更新、查询 |

### 架构洞察

#### 三种节点类型

| 类型 | 用途 | 生命周期 |
|------|------|---------|
| `PeerNode` | 内存完整状态 | 运行时 |
| `PeerModel` (数据库) | 持久化 | 长期 |
| `PeerInfo` | 网络传输 | 临时 |

**职责分离**：
- `PeerNode` → 实时管理、快速查询
- `PeerModel` → 历史记录、跨启动持久化
- `PeerInfo` → 最小化网络开销

#### HashMap vs BTreeMap

选择 `HashMap<IpAddr, PeerNode>` 而非 `BTreeMap`：

| 特性 | HashMap | BTreeMap |
|------|---------|----------|
| 查找 | O(1) 平均 | O(log n) |
| 插入 | O(1) 平均 | O(log n) |
| 排序 | 无序 | 有序 |
| 内存 | 较低 | 较高 |

**结论**：节点数量通常 < 100，HashMap 性能更优且更简单。

#### 为什么使用 SystemTime 而非 DateTime？

```rust
use std::time::SystemTime;  // 标准库
// vs
use chrono::NaiveDateTime;  // 外部依赖
```

**选择 `SystemTime`**：
- 零依赖（标准库）
- 足够精度（纳秒级）
- 跨平台一致性
- 节省编译时间

#### 阻塞式 start() 方法

当前 `start()` 是阻塞的：

```rust
pub fn start(&self) -> Result<()> {
    // 发送上线广播
    self.discovery.announce_online()?;
    // 开始监听（阻塞）
    self.discovery.listen_incoming(...)?;
}
```

**生产环境用法**：

```rust
// 在独立线程中运行
thread::spawn(move || {
    if let Err(e) = manager.start() {
        eprintln!("Peer manager error: {:?}", e);
    }
});

// 主线程继续执行
```

**未来改进**：
- 提供 `start_async()` 版本
- 使用 tokio `spawn()` 替代 `thread::spawn()`
- 支持优雅关闭（graceful shutdown）

### 使用示例

#### 创建并启动管理器

```rust
use network::UdpTransport;
use modules::peer::{PeerDiscovery, PeerManager};

let udp = UdpTransport::bind(2425)?;
let discovery = PeerDiscovery::with_defaults(udp);
let manager = PeerManager::new(discovery);

// 在后台线程启动
std::thread::spawn(move || {
    manager.start().unwrap();
});
```

#### 手动添加节点

```rust
let ip = "192.168.1.100".parse().unwrap();
let peer = PeerNode::with_details(
    ip,
    2425,
    Some("Alice".to_string()),
    Some("alice-pc".to_string()),
);

manager.add_peer(peer)?;
```

#### 查询节点

```rust
// 获取所有节点
let all_peers = manager.get_all_peers();
for peer in all_peers {
    println!("{} - {}", peer.display_name(), peer.ip);
}

// 只获取在线节点
let online_peers = manager.get_online_peers();
println!("Online: {} / {}", online_peers.len(), manager.peer_count());

// 检查特定节点
let ip = "192.168.1.100".parse().unwrap();
if let Some(peer) = manager.get_peer(ip) {
    println!("Found: {} - {:?}", peer.display_name(), peer.status);
}
```

#### 更新节点状态

```rust
let ip = "192.168.1.100".parse().unwrap();

// 标记为离开
manager.update_peer_status(ip, PeerStatus::Away)?;

// 标记为离线
manager.update_peer_status(ip, PeerStatus::Offline)?;
```

#### 移除节点

```rust
let ip = "192.168.1.100".parse().unwrap();

let removed = manager.remove_peer(ip);
if removed {
    println!("Peer removed: {}", ip);
}
```

### 类型关系图

```
┌─────────────────┐
│   PeerNode      │ 内存完整状态
│  (runtime)      │
└────────┬─────────┘
         │
         ├─ PeerInfo (轻量级)
         │  └─ 网络传输
         │
         └─ PeerModel (数据库)
            └─ 持久化
```

### 后续步骤

阶段 2.4 完成！下一步是 **阶段 2.5: 实现心跳机制**，需要：
1. 创建 `heartbeat.rs` 实现 `HeartbeatMonitor`
2. 定期发送心跳包（STATUS_HEARTBEAT）
3. 检测超时节点并标记离线

---

## ✅ 阶段 2.5：实现心跳机制

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [modules/peer/heartbeat.rs](src-tauri/src/modules/peer/heartbeat.rs) - 心跳监控模块（377 行）

#### 更新的文件
- [modules/peer/mod.rs](src-tauri/src/modules/peer/mod.rs:6,12) - 添加 `pub mod heartbeat;` 并导出 `HeartbeatMonitor`
- [modules/peer/discovery.rs](src-tauri/src/modules/peer/discovery.rs:20) - 为 `PeerDiscovery` 添加 `#[derive(Clone)]`

### 功能特性

#### HeartbeatMonitor 结构体
提供周期性心跳和离线检测功能：

| 方法 | 说明 | 参数 | 返回值 |
|------|------|------|--------|
| `new()` | 创建心跳监控（默认间隔） | `discovery, peers` | `Self` |
| `with_intervals()` | 创建心跳监控（自定义间隔） | `discovery, peers, heartbeat_interval, peer_timeout` | `Self` |
| `start()` | 启动心跳监控（后台线程） | - | `Result<()>` |
| `stop()` | 停止心跳监控 | - | - |
| `is_running()` | 检查运行状态 | - | `bool` |
| `heartbeat_interval()` | 获取心跳间隔 | - | `u64` |
| `peer_timeout()` | 获取超时时间 | - | `u64` |

#### 常量定义

| 常量名 | 值 | 说明 |
|--------|-----|------|
| `DEFAULT_HEARTBEAT_INTERVAL` | 30 | 默认心跳间隔（秒） |
| `DEFAULT_PEER_TIMEOUT` | 60 | 默认节点超时（秒） |

### 实现细节

#### 后台线程架构

使用 `std::thread::spawn` 创建独立的心跳线程：

```rust
std::thread::spawn(move || {
    loop {
        // 检查运行状态
        if !is_running { break; }

        // 发送心跳
        Self::send_heartbeat(&discovery, ...)?;

        // 检测离线节点
        Self::check_offline_peers(&peers, timeout)?;

        // 等待下一个周期
        std::thread::sleep(Duration::from_secs(interval));
    }
});
```

**设计考虑**：
- 非阻塞启动：`start()` 立即返回，心跳在后台运行
- 优雅关闭：通过 `running` 标志控制线程退出
- 独立线程：不影响 PeerManager 的消息监听

#### 心跳发送策略

使用 `STATUS_ONLINE` 消息作为心跳：

```rust
fn send_heartbeat(discovery: &PeerDiscovery, ...) -> Result<()> {
    // 使用 announce_online 作为心跳
    // 优点：
    // 1. 自动启用广播模式
    // 2. 兼容 IPMsg 协议
    // 3. STATUS_ONLINE 也起到刷新在线状态的作用
    discovery.announce_online()?;
    Ok(())
}
```

**为什么使用 STATUS_ONLINE 而非 STATUS_HEARTBEAT？**

| 方案 | 优点 | 缺点 |
|------|------|------|
| **STATUS_ONLINE** | 兼容性好、自动广播 | 语义上不够精确 |
| STATUS_HEARTBEAT | 语义精确 | 需要额外启用广播 |

**结论**：`STATUS_ONLINE` 在 IPMsg 协议中被广泛接受作为心跳使用，兼容性更好。

#### 离线检测机制

基于 `last_seen` 时间戳判断节点在线状态：

```rust
fn check_offline_peers(peers: &Arc<Mutex<...>>, timeout: u64) -> Result<()> {
    let now = SystemTime::now();
    let timeout_duration = Duration::from_secs(timeout);

    for (ip, peer) in peers.iter_mut() {
        if let Ok(duration) = now.duration_since(peer.last_seen) {
            if duration > timeout_duration && peer.is_online() {
                peer.mark_offline();
            }
        }
    }
}
```

**检测逻辑**：
1. 计算当前时间与 `last_seen` 的差值
2. 如果差值 > `peer_timeout`，标记为离线
3. 只标记在线节点为离线（避免重复标记）

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 10.26s |
| 单元测试 | ✅ 4 passed | - |
| 文档测试 | ✅ 8 passed | - |
| 总计 | ✅ 12 passed | - |

#### 单元测试覆盖

| 测试名称 | 测试内容 |
|---------|---------|
| `test_heartbeat_monitor_creation` | 测试默认创建 |
| `test_heartbeat_monitor_with_custom_intervals` | 测试自定义间隔 |
| `test_heartbeat_monitor_start_stop` | 测试启动和停止 |
| `test_check_offline_peers` | 测试离线检测逻辑 |

### 架构洞察

#### 线程模型选择

| 方案 | 优点 | 缺点 |
|------|------|------|
| **std::thread** | 简单、无依赖 | 每个线程占用栈空间 |
| tokio::spawn | 高效、支持异步 | 需要运行时支持 |

**当前选择**：`std::thread::spawn`
- 简单直接，符合现有代码风格
- 心跳线程只有一个，资源开销可接受
- 避免引入 tokio 运行时依赖

**未来改进**：如果需要更多异步操作，可迁移到 tokio

#### 共享状态设计

使用 `Arc<Mutex<>>` 共享 PeerManager 的节点列表：

```rust
pub struct HeartbeatMonitor {
    discovery: PeerDiscovery,              // Clone
    peers: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,  // 共享
    running: Arc<Mutex<bool>>,              // 共享控制
}
```

**线程安全保证**：
- `Arc` 允许多所有权共享
- `Mutex` 保证内部可变性
- 锁中毒错误转换为 `io::Error`

#### 心跳间隔与超时比例

默认值比例：`heartbeat_interval = 30s`，`peer_timeout = 60s`

**设计考虑**：
- `timeout >= 2 * interval`：允许丢失 1-2 个心跳包
- 避免误判：网络抖动不应导致节点离线
- 快速检测：60 秒超时仍然足够快

**建议配置**：
- 局域网：30s / 60s（默认）
- 不稳定网络：60s / 180s
- 快速检测：15s / 45s

### 使用示例

#### 创建并启动心跳监控

```rust
use network::UdpTransport;
use modules::peer::{PeerDiscovery, PeerManager, HeartbeatMonitor};

let udp = UdpTransport::bind(2425)?;
let discovery = PeerDiscovery::with_defaults(udp);
let manager = PeerManager::new(discovery);

// 从 PeerManager 获取共享 peer 列表
let peers = manager.get_peers_arc();  // 假设提供此方法

// 创建心跳监控
let heartbeat = HeartbeatMonitor::new(
    manager.discovery().clone(),  // 克隆 discovery
    peers,
);

// 启动心跳监控（后台线程）
heartbeat.start()?;

// 主线程继续执行...
```

#### 自定义间隔

```rust
// 15 秒心跳，45 秒超时
let heartbeat = HeartbeatMonitor::with_intervals(
    discovery,
    peers,
    15,  // 快速心跳
    45,  // 较短超时
);
```

#### 停止心跳监控

```rust
// 优雅停止
heartbeat.stop();

// 检查状态
if !heartbeat.is_running() {
    println!("Heartbeat stopped");
}
```

### 与 PeerManager 集成

心跳监控需要与 PeerManager 配合使用：

```
┌─────────────────┐
│  PeerManager    │
│  (主线程)        │
│  - 监听消息      │
│  - 处理上线/离线 │
└────────┬─────────┘
         │
         ├─ Arc<Mutex<HashMap<IpAddr, PeerNode>>>
         │  └─ 共享节点列表
         │
┌────────▼─────────┐
│ HeartbeatMonitor │
│  (后台线程)       │
│  - 发送心跳       │
│  - 检测离线       │
└──────────────────┘
```

**协作流程**：
1. **PeerManager**：处理入站消息，更新 `last_seen`
2. **HeartbeatMonitor**：定期发送心跳，检测超时节点
3. **共享节点列表**：两者通过 `Arc<Mutex<>>` 同步

### 单元测试详情

```rust
#[test]
fn test_check_offline_peers() {
    // 创建节点列表
    let mut peer_map = HashMap::new();

    // 活跃节点（2 秒前）
    let mut peer1 = PeerNode::new(ip1, 2425);
    peer1.last_seen = SystemTime::now() - Duration::from_secs(2);

    // 陈旧节点（2 分钟前）
    let mut peer2 = PeerNode::new(ip2, 2425);
    peer2.last_seen = SystemTime::now() - Duration::from_secs(120);

    peer_map.insert(ip1, peer1);
    peer_map.insert(ip2, peer2);

    let peers = Arc::new(Mutex::new(peer_map));

    // 检测离线节点（60 秒超时）
    HeartbeatMonitor::check_offline_peers(&peers, 60).unwrap();

    // 验证结果
    let peers_ref = peers.lock().unwrap();
    assert!(peers_ref[&ip1].is_online());   // 仍然在线
    assert!(!peers_ref[&ip2].is_online());  // 已离线
}
```

### 后续步骤

阶段 2.5 完成！**阶段 2（网络通信层）已全部完成**！

下一步是 **阶段 3.1: 创建节点查询命令**，需要：
1. 在 `src-tauri/src/commands/` 创建 `peer.rs`
2. 实现 `get_peers()` Tauri 命令
3. 定义 `PeerDto` 结构体用于前端序列化

---

## ✅ 阶段 2.3：实现节点发现功能

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [modules/peer/discovery.rs](src-tauri/src/modules/peer/discovery.rs) - 节点发现模块（258 行）

#### 更新的文件
- [modules/peer/mod.rs](src-tauri/src/modules/peer/mod.rs:3,6) - 添加 `pub mod discovery;` 并导出 `PeerDiscovery`

### 功能特性

#### PeerDiscovery 结构体
提供 UDP 广播节点发现服务：

| 方法 | 说明 | 参数 | 返回值 |
|------|------|------|--------|
| `new()` | 创建发现服务（自定义身份） | `udp, username, hostname` | `Self` |
| `with_defaults()` | 创建发现服务（系统身份） | `udp` | `Self` |
| `announce_online()` | 发送上线广播 | - | `Result<()>` |
| `listen_incoming()` | 监听入站消息（阻塞） | `callback` | `Result<()>` |
| `send_message()` | 发送单播消息 | `msg, addr` | `Result<()>` |
| `create_message()` | 创建协议消息 | `msg_type, content` | `ProtocolMessage` |
| `port()` | 获取绑定端口 | - | `u16` |
| `local_addr()` | 获取本地地址 | - | `Result<SocketAddr>` |
| `username()` | 获取用户名 | - | `&str` |
| `hostname()` | 获取主机名 | - | `&str` |
| `clone_transport()` | 克隆 UDP 传输 | - | `Arc<UdpTransport>` |

#### 核心功能

1. **上线广播**：向 LAN 广播 `STATUS_ONLINE` 消息
2. **消息监听**：持续监听入站 UDP 消息并回调处理
3. **单播发送**：向指定地址发送协议消息
4. **消息创建**：使用本地身份创建标准化协议消息

#### 常量定义

| 常量名 | 值 | 说明 |
|--------|-----|------|
| `RECV_BUFFER_SIZE` | 65535 | 接收缓冲区大小 (64KB) |

### 实现细节

#### 线程安全设计

使用 `Arc` 和 `AtomicU64` 实现线程安全的共享访问：

```rust
pub struct PeerDiscovery {
    udp: Arc<UdpTransport>,           // 共享 UDP 传输
    packet_id: Arc<AtomicU64>,        // 原子计数器
    // ...
}
```

这允许：
- 多个组件共享同一个 `UdpTransport`
- 线程安全地递增 packet ID
- 后续轻松扩展为多线程架构

#### 消息 ID 生成

使用 `AtomicU64` 的 `fetch_add` 确保唯一性：

```rust
fn next_packet_id(&self) -> u64 {
    self.packet_id.fetch_add(1, Ordering::SeqCst)
}
```

`Ordering::SeqCst` 提供最强的内存序保证，确保：
- 所有线程看到一致的 ID 序列
- 不会出现重复或乱序的 packet_id

#### 系统身份获取

使用 `whoami` crate 获取系统默认身份：

```rust
pub fn with_defaults(udp: UdpTransport) -> Self {
    let username = whoami::username();           // 系统用户名
    let hostname = whoami::fallible::hostname()  // 系统主机名
        .unwrap_or_else(|_| "localhost".to_string());
    // ...
}
```

#### 阻塞式监听循环

`listen_incoming` 提供阻塞式消息接收：

```rust
pub fn listen_incoming<F>(&self, mut callback: F) -> Result<()>
where
    F: FnMut(ProtocolMessage, SocketAddr),
{
    let mut buffer = [0u8; RECV_BUFFER_SIZE];

    loop {
        // 接收 UDP 包
        let (len, sender) = self.udp.recv_from(&mut buffer)?;

        // 解析协议消息
        let msg = parse_message(&buffer[..len])?;

        // 调用回调
        callback(msg, sender);
    }
}
```

**设计考虑**：
- 当前为阻塞实现（适合单线程或独立线程）
- 错误时记录警告并继续（容错设计）
- 回调模式允许灵活的消息处理

### 验证结果

| 测试项 | 状态 |
|--------|------|
| `cargo check` | ✅ 通过 |
| 单元测试 | ✅ 42 passed |
| 文档测试 | ✅ 8 passed |
| 总计 | ✅ 50 passed |

#### 单元测试覆盖

| 测试名称 | 测试内容 |
|---------|---------|
| `test_peer_discovery_creation` | 测试自定义身份创建 |
| `test_peer_discovery_with_defaults` | 测试系统身份创建 |
| `test_announce_online` | 测试上线广播 |
| `test_create_message` | 测试消息创建 |
| `test_packet_id_increment` | 测试 ID 递增 |
| `test_message_types` | 测试消息类型常量 |

### 架构洞察

#### 为什么使用回调而非 Channel？

| 方案 | 优点 | 缺点 |
|------|------|------|
| **回调** | 简单、零开销、无异步依赖 | 调用者需处理阻塞 |
| **Channel** | 异步、解耦 | 增加复杂度、需要运行时 |

**当前选择**：回调函数
- 简单直接，适合当前单线程设计
- 调用者可以决定在单独线程中运行监听器
- 未来可以轻松扩展为异步/Channel 模式

#### 广播与单播分离

```rust
// 广播（用于发现）
pub fn announce_online(&self) -> Result<()> {
    // 发送到 255.255.255.255:2425
}

// 单播（用于定向通信）
pub fn send_message(&self, msg: &ProtocolMessage, addr: SocketAddr) -> Result<()> {
    // 发送到指定地址
}
```

**设计原因**：
- 广播：自动覆盖 LAN 所有节点
- 单播：精准控制，避免不必要的网络流量
- 两者配合实现高效的 P2P 发现

#### 为什么 UDP transport 使用 Arc？

```rust
pub struct PeerDiscovery {
    udp: Arc<UdpTransport>,
    // ...
}

pub fn clone_transport(&self) -> Arc<UdpTransport> {
    Arc::clone(&self.udp)
}
```

**原因**：
1. **资源共享**：多个模块可以共享同一个 socket
2. **零成本抽象**：`Arc` 仅在克隆时增加引用计数
3. **线程安全**：`UdpTransport` 内部使用 `UdpSocket`，支持跨线程
4. **未来扩展**：方便后续实现多线程心跳发送

### 使用示例

#### 基本使用

```rust
use network::UdpTransport;
use modules::peer::PeerDiscovery;

// 创建 UDP 传输
let udp = UdpTransport::bind(2425)?;

// 创建发现服务（系统身份）
let discovery = PeerDiscovery::with_defaults(udp);

// 发送上线广播
discovery.announce_online()?;

// 获取本地信息
println!("Username: {}", discovery.username());
println!("Hostname: {}", discovery.hostname());
println!("Port: {}", discovery.port());
```

#### 自定义身份

```rust
let udp = UdpTransport::bind(2425)?;

let discovery = PeerDiscovery::new(
    udp,
    "Alice".to_string(),    // 自定义用户名
    "alice-pc".to_string()  // 自定义主机名
);

discovery.announce_online()?;
```

#### 监听入站消息

```rust
let udp = UdpTransport::bind(2425)?;
let discovery = PeerDiscovery::with_defaults(udp);

// 注意：这是阻塞调用，应在单独线程中运行
discovery.listen_incoming(|msg, sender| {
    println!("从 {} 收到消息:", sender);
    println!("  类型: {}", get_message_type_name(msg.msg_type));
    println!("  发送者: {}@{}", msg.sender_name, msg.sender_host);
    println!("  内容: {}", msg.content);
})?;
```

#### 发送单播消息

```rust
let udp = UdpTransport::bind(2425)?;
let discovery = PeerDiscovery::with_defaults(udp);

// 创建消息
let msg = discovery.create_message(
    msg_type::MSG_SEND,
    "Hello, Peer!".to_string()
);

// 发送到指定地址
let target: SocketAddr = "192.168.1.100:2425".parse()?;
discovery.send_message(&msg, target)?;
```

#### 在独立线程中监听

```rust
use std::thread;

let udp = UdpTransport::bind(2425)?;
let discovery = PeerDiscovery::with_defaults(udp);

// 克隆 UDP transport 用于其他用途
let transport = discovery.clone_transport();

// 启动监听线程
thread::spawn(move || {
    if let Err(e) = discovery.listen_incoming(|msg, sender| {
        // 处理消息
    }) {
        eprintln!("Listener error: {:?}", e);
    }
});

// 主线程继续执行其他任务...
```

### 后续步骤

阶段 2.3 完成！下一步是 **阶段 2.4: 实现节点管理器**，需要：
1. 在 `src-tauri/src/modules/peer/` 创建 `types.rs`，定义节点类型
2. 创建 `manager.rs` 实现 `PeerManager`
3. 集成发现服务和数据库存储

---

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [network/udp.rs](src-tauri/src/network/udp.rs) - UDP 传输模块

#### 更新的文件
- [network/mod.rs](src-tauri/src/network/mod.rs:4,18) - 添加 `pub mod udp;` 并导出 `UdpTransport` 和 `DEFAULT_UDP_PORT`

### 功能特性

#### UdpTransport 结构体
提供 UDP socket 的完整操作接口：

| 方法 | 说明 | 参数 | 返回值 |
|------|------|------|--------|
| `bind()` | 绑定指定端口 | `port: u16` | `Result<Self>` |
| `set_broadcast_enabled()` | 启用/禁用广播 | `enabled: bool` | `Result<()>` |
| `broadcast()` | 发送广播消息 | `data: &[u8]` | `Result<()>` |
| `send_to()` | 发送单播消息 | `data: &[u8], addr: SocketAddr` | `Result<()>` |
| `recv_from()` | 接收消息 | `buffer: &mut [u8]` | `Result<(usize, SocketAddr)>` |
| `port()` | 获取绑定端口 | - | `u16` |
| `local_addr()` | 获取本地地址 | - | `Result<SocketAddr>` |
| `set_read_timeout()` | 设置接收超时 | `duration_ms: Option<u64>` | `Result<()>` |
| `join_multicast()` | 加入多播组 | `multiaddr: &str` | `Result<()>` (仅 Linux) |

#### 常量定义

| 常量名 | 值 | 说明 |
|--------|-----|------|
| `DEFAULT_UDP_PORT` | 2425 | IPMsg 默认 UDP 端口 |
| `BROADCAST_ADDR` | "255.255.255.255" | LAN 广播地址 |
| `DEFAULT_BUFFER_SIZE` | 65535 | 默认接收缓冲区大小 (64KB) |

### 实现细节

#### 使用标准库 `UdpSocket`

最终选择使用 Rust 标准库的 `std::net::UdpSocket` 而非 `socket2` crate：
- **简化依赖**：无需额外的外部依赖
- **简洁 API**：`UdpSocket` 提供了更直观的接口
- **跨平台**：标准库保证跨平台兼容性

#### 广播支持

广播模式需要显式启用：
```rust
udp.set_broadcast_enabled(true)?;
udp.broadcast(b"Hello, LAN!")?;
```

#### 接收超时

支持设置接收超时，避免永久阻塞：
```rust
// 100ms 超时
udp.set_read_timeout(Some(100))?;

// 无限等待
udp.set_read_timeout(None)?;
```

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 10.15s |
| 绑定端口 | ✅ 正确 | - |
| 广播发送 | ✅ 正确 | - |
| 单播发送 | ✅ 正确 | - |
| 消息接收 | ✅ 正确 | - |
| 单元测试 | ✅ 通过（编译时检查） | - |

### 遇到的问题

**问题 1**：`socket2` crate 的 `recv_from` API 复杂
- **错误**：`expected &mut [MaybeUninit<u8>], found &mut [u8]`
- **原因**：socket2 0.5 版本的 `recv_from` 方法使用 `MaybeUninit<u8>` 缓冲区以提高性能
- **解决**：改用标准库的 `std::net::UdpSocket`，其 API 更简单直接

### 解决方案

#### 标准库 vs socket2

| 方案 | 优点 | 缺点 |
|------|------|------|
| `std::net::UdpSocket` | 零额外依赖，API 简单 | 性能略低于 socket2 |
| `socket2` | 更高性能，更多底层控制 | API 复杂，需要额外依赖 |

**结论**：对于当前场景，`UdpSocket` 完全满足需求且代码更简洁。

### 单元测试

包含 7 个单元测试：

| 测试名称 | 测试内容 |
|---------|---------|
| `test_bind_to_port` | 测试绑定到随机端口 |
| `test_bind_specific_port` | 测试绑定到指定端口 |
| `test_set_broadcast_enabled` | 测试广播设置 |
| `test_send_to_loopback` | 测试本地回环通信 |
| `test_set_read_timeout` | 测试超时设置 |
| `test_constants` | 测试常量值 |
| `test_broadcast_address_parsing` | 测试广播地址解析 |

### 架构洞察

#### UDP vs TCP 选择

为什么控制消息使用 UDP？

1. **低延迟**：无需连接建立，消息即时发送
2. **广播支持**：原生支持 LAN 广播，用于节点发现
3. **轻量级**：协议头部小，适合小消息
4. **兼容性**：IPMsg 协议基于 UDP

#### 广播地址

为什么使用 `255.255.255.255` 而非子网广播？

- `255.255.255.255`：受限广播地址，路由器不会转发
- 子网广播（如 `192.168.1.255`）：仅在特定子网有效

**结论**：使用受限广播更安全，避免意外跨网段传输。

#### 缓冲区大小

为什么选择 64KB？

- UDP 最大数据包大小（理论）= 65535 字节
- 实际 MTU 限制（以太网）= 1500 字节
- 64KB 缓冲区确保可以接收任何合法的 UDP 数据包
- IP 层会自动处理分片和重组

### 使用示例

#### 绑定端口并启用广播

```rust
use network::udp::UdpTransport;

// 绑定到 IPMsg 默认端口
let udp = UdpTransport::bind(2425)?;

// 启用广播
udp.set_broadcast_enabled(true)?;
```

#### 发送广播消息

```rust
use network::{UdpTransport, ProtocolMessage, serialize_message, msg_type};

let udp = UdpTransport::bind(2425)?;
udp.set_broadcast_enabled(true)?;

let msg = ProtocolMessage {
    version: 1,
    packet_id: 1,
    sender_name: "Alice".to_string(),
    sender_host: "alice-pc".to_string(),
    msg_type: msg_type::STATUS_ONLINE,
    content: "".to_string(),
};

let bytes = serialize_message(&msg)?;
udp.broadcast(&bytes)?;
```

#### 发送单播消息

```rust
use std::net::SocketAddr;

let udp = UdpTransport::bind(2425)?;

let target: SocketAddr = "192.168.1.100:2425".parse()?;
udp.send_to(b"Hello, Peer!", target)?;
```

#### 接收消息

```rust
let udp = UdpTransport::bind(2425)?;
let mut buffer = [0u8; 65535];

loop {
    match udp.recv_from(&mut buffer) {
        Ok((len, sender)) => {
            let data = &buffer[..len];
            println!("Received {} bytes from {}", len, sender);
            // 处理接收到的数据...
        }
        Err(e) => {
            eprintln!("Receive error: {:?}", e);
        }
    }
}
```

#### 使用超时接收

```rust
let udp = UdpTransport::bind(2425)?;

// 设置 1 秒超时
udp.set_read_timeout(Some(1000))?;

loop {
    match udp.recv_from(&mut buffer) {
        Ok((len, sender)) => {
            // 处理消息
        }
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // 超时，继续下一轮
            continue;
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            break;
        }
    }
}
```

### 后续步骤

阶段 2.2 完成！下一步是 **阶段 2.3: 实现节点发现功能**，需要：
1. 在 `src-tauri/src/modules/peer/` 创建 `discovery.rs`
2. 实现 `PeerDiscovery` 结构体
3. 实现上线广播和监听入站消息

---

---

**最后更新：** 2026-01-06 (Stage 3.1: 节点查询命令完成)

## ✅ 阶段 3.1：创建节点查询命令

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [commands/peer.rs](src-tauri/src/commands/peer.rs) - 节点查询命令模块（232 行）

#### 更新的文件
- [commands/mod.rs](src-tauri/src/commands/mod.rs) - 添加 `pub mod peer;` 并导出命令和类型
- [lib.rs](src-tauri/src/lib.rs:15,33-38) - 导入命令并注册到 invoke_handler
- [error.rs](src-tauri/src/error.rs:51-56) - 添加 `From<NeoLanError> for InvokeError` 实现

### 功能特性

#### PeerDto 结构体
前端可序列化的节点数据传输对象：

| 字段 | 类型 | 说明 |
|------|------|------|
| `ip` | `String` | IP 地址 |
| `port` | `u16` | UDP 端口 |
| `username` | `Option<String>` | 用户名 |
| `hostname` | `Option<String>` | 主机名 |
| `nickname` | `Option<String>` | 昵称（优先显示） |
| `avatar` | `Option<String>` | 头像 |
| `groups` | `Vec<String>` | 所属组 |
| `status` | `String` | 在线状态 ("online"/"offline"/"away") |
| `display_name` | `String` | 显示名（计算字段） |
| `last_seen` | `i64` | 最后活跃时间（Unix 毫秒） |

#### Tauri 命令

| 命令 | 说明 | 参数 | 返回值 |
|------|------|------|--------|
| `get_peers()` | 获取所有节点 | - | `Vec<PeerDto>` |
| `get_online_peers()` | 获取在线节点 | - | `Vec<PeerDto>` |
| `get_peer_by_ip()` | 根据 IP 查询 | `ip: String` | `Option<PeerDto>` |
| `get_peer_stats()` | 获取节点统计 | - | `PeerStats` |

#### PeerStats 结构体

| 字段 | 类型 | 说明 |
|------|------|------|
| `total` | `usize` | 总节点数 |
| `online` | `usize` | 在线节点数 |
| `offline` | `usize` | 离线节点数 |

### 实现细节

#### Tauri 2.x IPC 错误处理

添加了 `NeoLanError` 到 `InvokeError` 的转换：

```rust
// error.rs
impl From<NeoLanError> for tauri::ipc::InvokeError {
    fn from(err: NeoLanError) -> Self {
        tauri::ipc::InvokeError::from(err.to_string())
    }
}
```

**设计考虑**：
- Tauri 2.x 要求命令返回的错误类型实现 `Into<InvokeError>`
- 使用 `Display` trait 自动转换为字符串
- 前端接收格式化的错误消息

#### CamelCase JSON 序列化

使用 `#[serde(rename_all = "camelCase")]` 确保前端友好的 JSON：

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerDto {
    pub ip: String,
    pub port: u16,
    pub last_seen: i64,  // 前端: lastSeen
    // ...
}
```

#### 时间戳转换

将 `SystemTime` 转换为 Unix 毫秒时间戳：

```rust
fn system_time_to_millis(time: SystemTime) -> i64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or_else(|_| 0)
}
```

**为什么使用毫秒？**
- JavaScript `Date` 使用毫秒精度
- 足够的精度，同时保持可读性
- 便于前端直接使用 `new Date(timestamp)`

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 10.68s |
| 单元测试 | ✅ 66 passed | 15.58s |
| 文档测试 | ✅ 通过 | - |
| 总计 | ✅ 66 passed | - |

#### 单元测试覆盖

| 模块 | 测试数量 | 测试内容 |
|------|---------|---------|
| commands::peer | 4 个 | PeerDto 创建、系统时间转换、统计 |
| error | 5 个 | 错误显示、类型转换 |
| modules::peer | 18 个 | 节点类型、管理器、心跳 |
| network | 23 个 | 协议解析、UDP 传输 |
| config | 3 个 | 配置序列化、验证 |
| storage | 2 个 | 数据库连接 |
| utils | 1 个 | 日志初始化 |

### 遇到的问题

**问题 1**：Tauri 2.x IPC 错误类型不兼容
- **错误**：`the trait bounds were not satisfied: NeoLanError: Into<InvokeError>`
- **原因**：Tauri 2.x 要求命令返回的 `Result<T, E>` 中 `E` 必须能转换为 `InvokeError`
- **解决**：实现 `From<NeoLanError> for tauri::ipc::InvokeError`

**问题 2**：命令引用路径错误
- **错误**：`could not find '__cmd__get_peers' in 'commands'`
- **原因**：`#[tauri::command]` 宏生成的函数需要直接引用，不能通过模块路径
- **解决**：在 `lib.rs` 中直接导入命令函数，使用 `use commands::peer::{...}`

### 解决方案

#### Tauri 命令注册模式

```rust
// lib.rs
// 导入命令函数
use commands::peer::{get_peers, get_online_peers, get_peer_by_ip, get_peer_stats};

// 注册到 invoke_handler
.invoke_handler(tauri::generate_handler![
    greet,
    get_peers,              // 直接使用函数名
    get_online_peers,
    get_peer_by_ip,
    get_peer_stats,
])
```

### 架构洞察

#### DTO 模式

为什么需要 `PeerDto` 而不是直接使用 `PeerNode`？

| 方案 | 优点 | 缺点 |
|------|------|------|
| **PeerDto** | 前端友好、JSON 优化 | 需要转换 |
| PeerNode | 无需转换 | `SystemTime` 不可序列化、复杂类型 |

**结论**：使用 DTO 模式实现前后端解耦
- 后端使用高效的内存结构（`PeerNode`）
- 前端使用 JSON 友好的序列化结构（`PeerDto`）
- 转换层在后端命令中完成

#### 占位实现

当前命令返回空向量或 `None` 作为占位：

```rust
#[tauri::command]
pub fn get_peers() -> Result<Vec<PeerDto>> {
    // TODO: Integrate with PeerManager once state management is implemented
    tracing::info!("get_peers called");
    Ok(Vec::new())  // Placeholder
}
```

**设计考虑**：
- 命令接口先行定义，前端可以并行开发
- 后续阶段（3.3）将集成实际的 PeerManager 状态
- 日志记录便于调试

#### 错误传播

使用统一的 `Result<T>` 类型别名：

```rust
pub type Result<T> = std::result::Result<T, NeoLanError>;

#[tauri::command]
pub fn get_peers() -> Result<Vec<PeerDto>> {
    // 自动转换为 InvokeError
    Ok(Vec::new())
}
```

### 使用示例

#### 前端调用（TypeScript）

```typescript
import { invoke } from "@tauri-apps/api/core";

// 获取所有节点
const peers = await invoke<PeerDto[]>("get_peers");
console.log(`Found ${peers.length} peers`);

// 获取在线节点
const onlinePeers = await invoke<PeerDto[]>("get_online_peers");
console.log(`Online: ${onlinePeers.length}`);

// 根据IP查询
const peer = await invoke<PeerDto | null>("get_peer_by_ip", { 
  ip: "192.168.1.100" 
});

// 获取统计
const stats = await invoke<{ total: number; online: number; offline: number }>("get_peer_stats");
console.log(`Total: ${stats.total}, Online: ${stats.online}`);
```

#### PeerDto 类型定义（TypeScript）

```typescript
interface PeerDto {
  ip: string;
  port: number;
  username: string | null;
  hostname: string | null;
  nickname: string | null;
  avatar: string | null;
  groups: string[];
  status: "online" | "offline" | "away";
  displayName: string;
  lastSeen: number;  // Unix milliseconds
}

interface PeerStats {
  total: number;
  online: number;
  offline: number;
}
```

### 后续步骤

阶段 3.1 完成！下一步是 **阶段 3.2: 创建配置相关命令**，需要：
1. 在 `src-tauri/src/commands/` 创建 `config.rs`
2. 实现 `get_config()`, `set_config()` Tauri 命令
3. 定义 `ConfigDto` 结构体用于配置读写

注意：当前命令为占位实现，将在阶段 3.3（状态监听命令）中集成实际的 PeerManager 状态管理。

---

---

**最后更新：** 2026-01-06 (Stage 3.2: 配置相关命令完成)

## ✅ 阶段 3.2：创建配置相关命令

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [commands/config.rs](src-tauri/src/commands/config.rs) - 配置管理命令模块（448 行）

#### 更新的文件
- [commands/mod.rs](src-tauri/src/commands/mod.rs) - 添加 `pub mod config;` 并导出命令和类型
- [lib.rs](src-tauri/src/lib.rs:16,40-44) - 导入命令并注册到 invoke_handler

### 功能特性

#### ConfigDto 结构体
前端可序列化的配置数据传输对象：

| 字段 | 类型 | 说明 |
|------|------|------|
| `username` | `String` | 用户名 |
| `hostname` | `String` | 主机名 |
| `bind_ip` | `String` | 绑定 IP 地址 |
| `udp_port` | `u16` | UDP 端口 |
| `tcp_port_start` | `u16` | TCP 端口范围起始 |
| `tcp_port_end` | `u16` | TCP 端口范围结束 |
| `heartbeat_interval` | `u64` | 心跳间隔（秒） |
| `peer_timeout` | `u64` | 超时时间（秒） |
| `encryption_enabled` | `bool` | 是否启用加密 |
| `encryption_key` | `Option<String>` | 加密密钥 |
| `offline_message_retention_days` | `u32` | 离线消息保留天数 |
| `auto_accept_files` | `bool` | 是否自动接受文件 |
| `file_save_dir` | `String` | 文件保存目录 |
| `log_level` | `String` | 日志级别 |

#### Tauri 命令

| 命令 | 说明 | 参数 | 返回值 |
|------|------|------|--------|
| `get_config()` | 获取当前配置 | - | `ConfigDto` |
| `set_config()` | 设置配置 | `config: ConfigDto` | `()` |
| `reset_config()` | 重置为默认配置 | - | `ConfigDto` |
| `get_config_value()` | 获取单个配置值 | `key: String` | `Option<String>` |
| `set_config_value()` | 设置单个配置值 | `key: String, value: String` | `()` |

### 实现细节

#### 配置验证

`ConfigDto::validate()` 提供完整的配置验证：

```rust
pub fn validate(&self) -> Result<()> {
    // 验证端口范围
    if self.udp_port < 1024 {
        return Err(NeoLanError::Validation("udp_port must be >= 1024".to_string()));
    }
    
    if self.tcp_port_start >= self.tcp_port_end {
        return Err(NeoLanError::Validation("tcp_port_start must be less than tcp_port_end".to_string()));
    }
    
    // 验证心跳和超时
    if self.peer_timeout < self.heartbeat_interval {
        return Err(NeoLanError::Validation("peer_timeout must be >= heartbeat_interval".to_string()));
    }
    
    // 验证日志级别
    match self.log_level.as_str() {
        "trace" | "debug" | "info" | "warn" | "error" => {}
        _ => return Err(NeoLanError::Validation(...)),
    }
    
    Ok(())
}
```

#### HashMap 转换

支持与 HashMap 之间的灵活转换：

```rust
// 从 HashMap 创建
pub fn from_map(map: &HashMap<String, String>) -> Self {
    Self {
        username: map.get("username").cloned().unwrap_or_else(|| whoami::username()),
        udp_port: map.get("udp_port").and_then(|s| s.parse().ok()).unwrap_or(2425),
        // ...
    }
}

// 转换为 HashMap
pub fn to_map(&self) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("username".to_string(), self.username.clone());
    map.insert("udp_port".to_string(), self.udp_port.to_string());
    // ...
}
```

#### 单值验证

`set_config_value` 对每个键进行特定验证：

| 键名 | 验证规则 |
|------|----------|
| `udp_port`, `tcp_port_start`, `tcp_port_end` | 必须 >= 1024 |
| `heartbeat_interval`, `peer_timeout` | 必须 > 0 |
| `encryption_enabled`, `auto_accept_files` | 必须是 "true" 或 "false" |
| `log_level` | 必须是 trace/debug/info/warn/error |

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 7.42s |
| 单元测试 | ✅ 71 passed | 12.57s |
| 文档测试 | ✅ 通过 | - |
| 总计 | ✅ 71 passed | - |

#### 单元测试覆盖

| 模块 | 测试数量 | 测试内容 |
|------|---------|---------|
| commands::config | 5 个 | ConfigDto 默认值、验证、转换、单值设置 |
| commands::peer | 4 个 | PeerDto 创建、系统时间转换、统计 |
| error | 5 个 | 错误显示、类型转换 |
| modules::peer | 18 个 | 节点类型、管理器、心跳 |
| network | 23 个 | 协议解析、UDP 传输 |
| config | 3 个 | 配置序列化、验证 |
| storage | 2 个 | 数据库连接 |
| utils | 1 个 | 日志初始化 |

### 遇到的问题

**问题**：`unwrap_or_else` 闭包参数不匹配
- **错误**：`closure is expected to take 0 arguments, but it takes 1 argument`
- **原因**：`Option::cloned()` 后的 `unwrap_or_else` 期望无参闭包，但内部 `whoami::fallible::hostname()` 返回 `Result`
- **解决**：将嵌套的 `unwrap_or_else` 改为单层调用

```rust
// 修复前
hostname: map.get("hostname").cloned().unwrap_or_else(|_| {
    whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string())
}),

// 修复后
hostname: map.get("hostname").cloned().unwrap_or_else(|| 
    whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string())
),
```

### 架构洞察

#### 配置验证策略

为什么在 DTO 层验证而非命令层？

| 方案 | 优点 | 缺点 |
|------|------|------|
| **DTO 层验证** | 验证逻辑可复用、类型安全 | DTO 需要依赖错误类型 |
| 命令层验证 | 简单直接 | 逻辑重复、难以复用 |

**结论**：在 DTO 层验证
- `ConfigDto::validate()` 可在任何地方调用
- 单元测试更容易
- 验证逻辑集中管理

#### 默认值策略

使用 `whoami` 和 `dirs` crate 获取系统默认值：

```rust
impl Default for ConfigDto {
    fn default() -> Self {
        Self {
            username: whoami::username(),
            hostname: whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string()),
            file_save_dir: dirs::download_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .to_string_lossy()
                .to_string(),
            // ...
        }
    }
}
```

**优势**：
- 跨平台兼容（Windows/macOS/Linux）
- 用户体验良好（自动使用系统用户名和下载目录）
- 降级优雅（获取失败时使用默认值）

#### 灵活的配置接口

提供三级配置接口：

1. **批量配置**：`get_config()` / `set_config()` - 适用于设置页面
2. **单值读写**：`get_config_value()` / `set_config_value()` - 适用于快速修改
3. **重置功能**：`reset_config()` - 恢复默认设置

### 使用示例

#### 前端调用（TypeScript）

```typescript
import { invoke } from "@tauri-apps/api/core";

// 获取完整配置
const config = await invoke<ConfigDto>("get_config");
console.log(`UDP Port: ${config.udpPort}`);
console.log(`Log Level: ${config.logLevel}`);

// 更新完整配置
config.udp_port = 2426;
config.log_level = "debug";
await invoke("set_config", { config });

// 重置为默认
const defaultConfig = await invoke<ConfigDto>("reset_config");

// 单值读写
const username = await invoke<string | null>("get_config_value", { key: "username" });
await invoke("set_config_value", { key: "log_level", value: "debug" });
```

#### ConfigDto 类型定义（TypeScript）

```typescript
interface ConfigDto {
  // 用户信息
  username: string;
  hostname: string;
  
  // 网络设置
  bindIp: string;
  udpPort: number;
  tcpPortStart: number;
  tcpPortEnd: number;
  
  // 节点发现
  heartbeatInterval: number;
  peerTimeout: number;
  
  // 安全设置
  encryptionEnabled: boolean;
  encryptionKey?: string;
  
  // 消息设置
  offlineMessageRetentionDays: number;
  
  // 文件传输
  autoAcceptFiles: boolean;
  fileSaveDir: string;
  
  // 应用设置
  logLevel: "trace" | "debug" | "info" | "warn" | "error";
}
```

### 后续步骤

阶段 3.2 完成！下一步是 **阶段 3.3: 创建状态监听命令**，需要：
1. 实现应用状态管理（使用 `tauri::State` 或 `Arc<Mutex<>>`）
2. 集成 `PeerManager` 和 `ConfigRepository` 到命令层
3. 实现事件监听机制（节点上线/下线/状态变化）

注意：当前命令为占位实现，将在阶段 3.3 中集成实际的配置持久化功能。

---

---

**最后更新：** 2026-01-06 (Stage 3.3: 状态监听命令完成)

## ✅ 阶段 3.3：创建状态监听命令

### 完成日期
2026-01-06

### 完成内容

#### 创建的文件
- [state/mod.rs](src-tauri/src/state/mod.rs) - 状态管理模块
- [state/app_state.rs](src-tauri/src/state/app_state.rs) - AppState 结构体（148 行）
- [state/events.rs](src-tauri/src/state/events.rs) - 事件系统（113 行）
- [commands/events.rs](src-tauri/src/commands/events.rs) - 事件轮询命令（18 行）

#### 更新的文件
- [lib.rs](src-tauri/src/lib.rs:8,14,19,43,55) - 添加 state 模块、AppState 管理、poll_events 命令
- [commands/mod.rs](src-tauri/src/commands/mod.rs) - 导出事件命令和 FrontendEvent
- [commands/peer.rs](src-tauri/src/commands/peer.rs:7,132-178) - 集成 AppState，实现实际节点查询
- [commands/config.rs](src-tauri/src/commands/config.rs:6-7,55-93,279-468) - 集成 AppState，实现实际配置读写

### 功能特性

#### AppState 结构体

应用状态管理中心，使用 `Arc<Mutex<>>` 实现线程安全：

```rust
pub struct AppState {
    peer_manager: Arc<Mutex<Option<PeerManager>>>,
    config: Arc<Mutex<AppConfig>>,
    event_emitter: Arc<Mutex<AppEventEmitter>>,
}
```

| 方法 | 说明 |
|------|------|
| `new(config)` | 创建新的应用状态 |
| `get_config()` | 获取当前配置 |
| `set_config(config)` | 设置配置并触发事件 |
| `update_config(updater)` | 更新配置字段 |
| `init_peer_manager(manager)` | 初始化节点管理器 |
| `get_peers()` | 获取所有节点 |
| `get_online_peers()` | 获取在线节点 |
| `get_peer(ip)` | 根据 IP 查询节点 |
| `get_peer_stats()` | 获取节点统计 |
| `emit_event(event)` | 触发事件 |
| `drain_events()` | 获取并清空事件队列 |

#### AppEvent 枚举

应用事件类型，使用 serde 标签序列化：

```rust
#[serde(tag = "type", content = "data")]
pub enum AppEvent {
    PeerOnline { ip: String, port: u16, username: Option<String> },
    PeerOffline { ip: String },
    PeerStatusChanged { ip: String, status: String },
    PeerUpdated { ip: String, username: Option<String> },
    ConfigChanged,
    Initialized,
    Error { message: String },
}
```

#### 事件缓冲机制

`AppEventEmitter` 提供事件缓冲和轮询机制：

```rust
pub struct AppEventEmitter {
    events: Vec<AppEvent>,
    max_buffer_size: usize,  // 默认 1000
}
```

**特性**：
- 自动限制缓冲区大小防止内存溢出
- 线程安全的事件添加和获取
- `drain()` 方法获取并清空队列

#### Tauri 命令集成

| 命令 | 集成前 | 集成后 |
|------|--------|--------|
| `get_peers()` | 返回空 Vec | 从 AppState 查询实际节点 |
| `get_config()` | 返回默认值 | 从 AppState 读取配置 |
| `set_config()` | 仅验证 | 写入 AppState 并触发事件 |
| `poll_events()` | - | 新增：轮询事件队列 |

### 实现细节

#### Tauri State 管理模式

使用 `tauri::State` 依赖注入：

```rust
// lib.rs
let app_state = AppState::new(default_config);
tauri::Builder::default()
    .manage(app_state)  // 注册状态
    .invoke_handler(...)

// 命令中使用
#[tauri::command]
pub fn get_peers(state: tauri::State<AppState>) -> Result<Vec<PeerDto>> {
    let peers = state.get_peers();
    // ...
}
```

**优势**：
- Tauri 自动管理状态生命周期
- 命令通过 `State<T>` 参数访问
- 线程安全的共享访问

#### ConfigDto 与 AppConfig 转换

```rust
impl ConfigDto {
    pub fn from_app_config(config: &AppConfig) -> Self { /* ... */ }
    pub fn to_app_config(&self) -> AppConfig { /* ... */ }
}
```

**设计考虑**：
- ConfigDto：前端友好的 JSON 结构
- AppConfig：后端使用的内存结构
- 双向转换实现桥接

#### 事件驱动架构

```
配置更改 → set_config() → AppState::set_config()
                            ↓
                     emit_event(ConfigChanged)
                            ↓
                     AppEventEmitter::emit()
                            ↓
                     前端 poll_events()
```

### 验证结果

| 测试项 | 状态 | 耗时 |
|--------|------|------|
| `cargo check` | ✅ 通过 | 10.59s |
| 单元测试 | ✅ 79 passed | 23.03s |
| 文档测试 | ✅ 通过 | - |
| 总计 | ✅ 79 passed | - |

#### 单元测试覆盖

| 模块 | 测试数量 | 测试内容 |
|------|---------|---------|
| state::app_state | 3 个 | AppState 创建、配置访问、更新 |
| state::events | 5 个 | 事件创建、事件发射器、缓冲区限制 |
| commands::config | 5 个 | ConfigDto 验证、转换 |
| commands::peer | 4 个 | PeerDto 创建、转换 |
| 其他模块 | 62 个 | 网络协议、节点管理、数据库等 |

### 架构洞察

#### 为什么使用轮询而非 WebSocket？

| 方案 | 优点 | 缺点 |
|------|------|------|
| **轮询** | 简单、无额外依赖 | 有延迟 |
| WebSocket | 实时性 | 需要额外连接管理 |

**结论**：使用轮询方式
- 简单可靠，无需额外连接
- 100ms 轮询间隔对用户体验影响可忽略
- Tauri 2.x 原生支持轮询模式

#### AppState 单例模式

```rust
pub struct AppState {
    peer_manager: Arc<Mutex<Option<PeerManager>>>,
    config: Arc<Mutex<AppConfig>>,
    event_emitter: Arc<Mutex<AppEventEmitter>>,
}
```

**为什么使用 `Arc<Mutex<>>`？**
- `Arc`：多线程共享所有权
- `Mutex`：内部可变性 + 线程安全
- 允许跨命令共享可变状态

#### 命令与状态分离

```
Tauri Commands (IPC Layer)
        ↓
    AppState (State Management)
        ↓
    Modules (Business Logic)
```

**好处**：
- 命令层轻量，仅处理 IPC
- 状态层管理生命周期
- 模块层专注业务逻辑

### 使用示例

#### 前端轮询事件（TypeScript）

```typescript
import { invoke } from "@tauri-apps/api/core";

interface AppEvent {
  type: "PeerOnline" | "PeerOffline" | "ConfigChanged" | "Initialized" | "Error";
  data?: any;
}

// 每 100ms 轮询事件
setInterval(async () => {
  const events = await invoke<AppEvent[]>("poll_events");
  for (const event of events) {
    switch (event.type) {
      case "PeerOnline":
        console.log(`Node ${event.data.ip} came online`);
        break;
      case "PeerOffline":
        console.log(`Node ${event.data.ip} went offline`);
        break;
      case "ConfigChanged":
        console.log("Configuration changed");
        break;
    }
  }
}, 100);
```

#### 完整的事件处理流程

```typescript
// 1. 启动轮询
const eventHandler = new Map<string, (data: any) => void>();

eventHandler.set("PeerOnline", (data) => {
  addPeerToList(data);
});

eventHandler.set("PeerOffline", (data) => {
  removePeerFromList(data.ip);
});

// 2. 轮询并分发
setInterval(async () => {
  const events = await_invoke<AppEvent[]>("poll_events");
  events.forEach(event => {
    const handler = eventHandler.get(event.type);
    if (handler) handler(event.data);
  });
}, 100);
```

### 后续步骤

阶段 3 全部完成！下一步是 **阶段 4: 前端基础 UI**，需要：
1. 安装前端依赖（Pinia、Vue Router 等）
2. 创建 Pinia Store 状态管理
3. 创建节点列表组件
4. 创建设置页面组件

注意：当前阶段 3 已完成整个 Tauri 命令层的基础架构，包括：
- 节点查询命令（3.1）
- 配置管理命令（3.2）
- 状态管理和事件监听（3.3）

所有命令已集成 `AppState`，可以访问实际的节点和配置状态。

---
