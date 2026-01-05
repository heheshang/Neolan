# NeoLan 实施计划

> 本文档提供 NeoLan 项目的详细实施路线图，每一步都小而具体，包含验证测试。

## 文档版本

| 版本号 | 日期 | 修订说明 |
|--------|------|---------|
| V1.0 | 2026-01-05 | 初始版本 |

---

# 前置说明

## 实施原则

1. **小步快跑**：每个步骤独立可验证，完成后可演示
2. **测试驱动**：每个功能完成后立即编写测试验证
3. **模块化优先**：严格遵循文件拆分规则，禁止单体文件
4. **基础功能优先**：先完成核心通信，再扩展高级功能
5. **文档同步**：每个里程碑完成后更新 `memory-bank/@architecture.md`

## 强制规则（Always）

在编写任何代码前，必须完整阅读：
- `memory-bank/@architecture.md`（包含完整数据库结构）
- `memory-bank/@neolan-design-document.md`
- `tech-stack.md`（模块化规则）

---

# 阶段 0：项目基础设施搭建

## 0.1 添加基础开发依赖

### 目标
配置 Rust 基础工具链和错误处理库

### 指令
1. 打开 `src-tauri/Cargo.toml`
2. 在 `[dependencies]` 部分添加以下依赖：
   - `thiserror = "1"` - 错误处理 derive
   - `anyhow = "1"` - 简化错误处理
   - `uuid = { version = "1", features = ["v4", "serde"] }` - UUID 生成
   - `chrono = { version = "0.4", features = ["serde"] }` - 时间处理
   - `tracing = "0.1"` - 结构化日志
   - `tracing-subscriber = { version = "0.3", features = ["env-filter"] }` - 日志订阅器

### 验证测试
1. 运行 `cargo check`，确保所有依赖正确解析
2. 运行 `cargo build`，确保编译成功
3. 在 `src-tauri/src/lib.rs` 中导入 `thiserror::Error` 和 `anyhow::Result`，无编译错误

### 预期结果
- 所有依赖成功下载
- 项目编译通过

---

## 0.2 创建模块目录结构

### 目标
建立 Rust 后端模块化目录结构

### 指令
1. 在 `src-tauri/src/` 下创建以下目录结构：
   ```
   commands/
   modules/
   modules/peer/
   modules/message/
   modules/file_transfer/
   modules/crypto/
   modules/group/
   network/
   storage/
   config/
   utils/
   ```

2. 为每个目录创建 `mod.rs` 文件，内容为空模块声明：
   ```rust
   // 例如 commands/mod.rs
   ```

3. 在 `src-tauri/src/lib.rs` 中添加模块声明：
   - `mod commands;`
   - `mod modules;`
   - `mod network;`
   - `mod storage;`
   - `mod config;`
   - `mod utils;`

4. 在 `src-tauri/src/modules/mod.rs` 中添加子模块声明

### 验证测试
1. 运行 `cargo check`，确保模块结构正确
2. 确保 Rust Analyzer 可以识别所有模块

### 预期结果
- 所有模块正确声明
- 无编译错误

---

## 0.3 配置日志系统

### 目标
配置 `tracing` 日志系统，用于调试和监控

### 指令
1. 在 `src-tauri/src/utils/` 创建 `logger.rs`
2. 实现一个 `init_logger()` 函数，配置 tracing-subscriber：
   - 日志级别从环境变量 `RUST_LOG` 读取
   - 默认级别为 `info`
   - 输出格式包含时间戳、级别、模块名、消息
3. 在 `src-tauri/src/lib.rs` 的 `run()` 函数开头调用 `init_logger()`

### 验证测试
1. 在 `run()` 函数中添加 `tracing::info!("NeoLan starting...")`
2. 运行 `npm run tauri dev`
3. 查看终端输出，确认日志格式正确

### 预期结果
- 终端显示带时间戳的日志信息
- 可以通过 `RUST_LOG=debug` 环境变量调整日志级别

---

## 0.4 创建错误类型系统

### 目标
使用 `thiserror` 创建统一的错误类型

### 指令
1. 在 `src-tauri/src/` 创建 `error.rs`
2. 定义一个主错误枚举 `NeoLanError`，包含以下变体：
   - `Network` - 网络相关错误
   - `Protocol` - 协议解析错误
   - `Storage` - 存储错误
   - `Crypto` - 加密错误
   - `Config` - 配置错误
   - `Io` - IO 错误（包装 `std::io::Error`）
3. 为每个变体实现相应的错误转换 trait
4. 创建一个类型别名 `type Result<T> = std::result::Result<T, NeoLanError>`

### 验证测试
1. 在测试代码中手动构造每种错误类型
2. 确认错误可以正确显示

### 预期结果
- 统一的错误类型
- 可以使用 `?` 操作符进行错误传播

---

# 阶段 1：数据持久化层

## 1.1 创建数据库连接模块

### 目标
实现 SQLite 数据库连接和初始化

### 指令
1. 在 `src-tauri/Cargo.toml` 添加依赖：
   - `sea-orm = { version = "1", features = ["sqlx-sqlite", "runtime-tokio-rustls"] }`
   - `sea-orm-migration = "1"`
   - `tokio = { version = "1", features = ["full"] }`

2. 在 `src-tauri/src/storage/` 创建 `database.rs`
3. 实现以下函数：
   - `establish_connection() -> Result<DatabaseConnection>` - 创建数据库连接
   - 数据库文件路径：`%APPDATA%/neolan/neolan.db`（使用 `tauri::api::path::app_data_dir`）
   - 如果目录不存在则创建

### 验证测试
1. 编写单元测试调用 `establish_connection()`
2. 确认数据库文件被创建
3. 使用 SQLite 客户端工具（如 DB Browser）打开数据库，确认可以访问

### 预期结果
- 数据库文件在正确位置创建
- 连接对象可以正常使用

---

## 1.2 定义数据库实体模型

### 目标
创建 peers、messages、transfers 表的 ORM 模型

### 指令
1. 在 `src-tauri/src/storage/` 创建 `models.rs`
2. 使用 `sea-orm::Entity` derive 宏定义以下实体：
   - `Peer` - 对应 peers 表
   - `Message` - 对应 messages 表
   - `Transfer` - 对应 transfers 表
   - `Group` - 对应 groups 表
   - `Setting` - 对应 settings 表
3. 每个实体包含 `@sea-orm(primary_key)]` 和其他必要字段的注解

### 验证测试
1. 运行 `cargo check`，确认模型编译通过
2. 在测试代码中创建实体实例

### 预期结果
- 所有实体模型定义正确
- 字段类型与数据库表结构一致

---

## 1.3 创建数据库迁移

### 目标
创建数据库表结构的迁移脚本

### 指令
1. 在 `src-tauri/src/` 创建 `migration/` 目录
2. 创建迁移文件 `m20260105_000001_create_tables.rs`
3. 在迁移中定义以下表的 CREATE 语句：
   - peers（包含所有字段和索引）
   - messages（包含所有字段和索引）
   - transfers（包含所有字段和索引）
   - groups（包含所有字段和索引）
   - settings（包含所有字段和索引）
   - audit_logs（包含所有字段和索引）

### 验证测试
1. 运行迁移，确认所有表被创建
2. 使用 SQLite 工具检查表结构与 `@architecture.md` 定义一致
3. 确认所有索引被正确创建

### 预期结果
- 数据库包含所有定义的表
- 表结构与架构文档一致

---

## 1.4 实现节点数据访问层

### 目标
创建 PeerRepository 用于节点数据的 CRUD 操作

### 指令
1. 在 `src-tauri/src/storage/` 创建 `peer_repo.rs`
2. 实现 `PeerRepository` 结构体，持有 `DatabaseConnection`
3. 实现以下方法：
   - `insert(peer: &PeerModel) -> Result<()>`
   - `update(peer: &PeerModel) -> Result<()>`
   - `find_by_ip(ip: &str) -> Result<Option<PeerModel>>`
   - `find_all() -> Result<Vec<PeerModel>>`
   - `find_by_status(status: &str) -> Result<Vec<PeerModel>>`
   - `delete_by_ip(ip: &str) -> Result<()>`

### 验证测试
1. 编写集成测试：
   - 插入一个测试节点
   - 通过 IP 查询该节点
   - 更新节点状态
   - 删除节点
2. 使用 SQLite 工具确认数据正确写入和更新

### 预期结果
- 所有 CRUD 操作正常工作
- 数据正确持久化到数据库

---

## 1.5 实现消息数据访问层

### 目标
创建 MessageRepository 用于消息数据的持久化

### 指令
1. 在 `src-tauri/src/storage/` 创建 `message_repo.rs`
2. 实现 `MessageRepository` 结构体
3. 实现以下方法：
   - `insert(message: &MessageModel) -> Result<()>`
   - `find_by_peer(peer_ip: &str, limit: u64) -> Result<Vec<MessageModel>>`
   - `find_offline_messages(peer_ip: &str) -> Result<Vec<MessageModel>>`
   - `mark_as_delivered(msg_id: &str) -> Result<()>`
   - `delete_old_messages(before: DateTime<Utc>) -> Result<u64>`

### 验证测试
1. 编写集成测试插入和查询消息
2. 验证离线消息查询功能
3. 验证消息删除功能

### 预期结果
- 消息可以正确存储和检索
- 离线消息功能正常

---

## 1.6 实现配置存储模块

### 目标
创建配置的读写接口

### 指令
1. 在 `src-tauri/src/config/` 创建 `app.rs`
2. 实现 `AppConfig` 结构体，包含以下配置项：
   - 用户名、机器名
   - UDP 端口、TCP 端口范围
   - 心跳间隔、超时时间
   - 是否启用加密
3. 实现从 settings 表加载配置
4. 实现保存配置到 settings 表

### 验证测试
1. 修改配置并保存
2. 重启应用后加载配置，确认值正确
3. 使用 SQLite 工具查看 settings 表

### 预期结果
- 配置可以正确持久化和加载

---

# 阶段 2：网络通信层

## 2.1 实现协议解析器

### 目标
创建 IPMsg 协议的解析和封装功能

### 指令
1. 在 `src-tauri/src/network/` 创建 `protocol.rs`
2. 定义 `ProtocolMessage` 结构体，包含：
   - version: u8
   - packet_id: u64
   - sender_name: String
   - sender_host: String
   - msg_type: u32
   - content: String
3. 实现以下函数：
   - `parse_message(data: &[u8]) -> Result<ProtocolMessage>` - 解析字节流为消息结构
   - `serialize_message(msg: &ProtocolMessage) -> Result<Vec<u8>>` - 将消息结构序列化为字节流
   - 协议格式：`version:packet_id:sender_name:sender_host:msg_type:content`
4. 定义消息类型常量（如 MSG_ONLINE = 0x00000001）

### 验证测试
1. 创建测试消息并序列化
2. 解析序列化后的字节流，对比原始消息
3. 测试各种消息类型的解析
4. 测试边界情况（空消息、超长消息）

### 预期结果
- 消息可以正确序列化和反序列化
- 支持所有定义的消息类型

---

## 2.2 实现 UDP 传输模块

### 目标
创建 UDP socket 的封装，用于发送和接收控制消息

### 指令
1. 添加依赖：`socket2 = "0.5"`
2. 在 `src-tauri/src/network/` 创建 `udp.rs`
3. 实现 `UdpTransport` 结构体，包含：
   - socket: UdpSocket
   - port: u16
4. 实现以下方法：
   - `bind(port: u16) -> Result<Self>` - 绑定指定端口
   - `broadcast(data: &[u8]) -> Result<()>` - 发送广播消息到 255.255.255.255
   - `send_to(data: &[u8], addr: SocketAddr) -> Result<()>` - 发送单播消息
   - `recv_from(&mut buffer) -> Result<(usize, SocketAddr)>` - 接收消息
   - `set_broadcastEnabled(true)` - 启用广播

### 验证测试
1. 绑定 UDP 端口 2425
2. 发送广播消息
3. 使用网络抓包工具（如 Wireshark）确认 UDP 数据包发出
4. 测试接收功能

### 预期结果
- UDP socket 可以正常发送和接收数据
- 广播消息可以到达局域网

---

## 2.3 实现节点发现功能

### 目标
通过 UDP 广播发现局域网内的在线节点

### 指令
1. 在 `src-tauri/src/modules/peer/` 创建 `discovery.rs`
2. 实现 `PeerDiscovery` 结构体，持有 `UdpTransport`
3. 实现以下方法：
   - `new(udp: UdpTransport) -> Self`
   - `announce_online() -> Result<()>` - 发送上线广播（MSG_ONLINE）
   - `listen_incoming<F>(callback: F) where F: Fn(ProtocolMessage, SocketAddr)` - 监听并处理入站消息
4. 上线广播格式：使用协议解析器封装 STATUS_ONLINE 消息

### 验证测试
1. 启动两个应用实例
2. 观察终端日志，确认两个实例相互发现
3. 使用 Wireshark 抓包验证广播内容

### 预期结果
- 应用启动后自动发送上线广播
- 可以接收到其他节点的广播消息

---

## 2.4 实现节点管理器

### 目标
创建核心的节点管理逻辑

### 指令
1. 在 `src-tauri/src/modules/peer/` 创建 `manager.rs`
2. 实现 `PeerManager` 结构体，包含：
   - peers: HashMap<IpAddr, PeerNode>
   - db: PeerRepository
   - discovery: PeerDiscovery
3. 实现以下方法：
   - `new(db: PeerRepository, discovery: PeerDiscovery) -> Self`
   - `start(&mut self) -> Result<()>` - 启动节点发现和心跳
   - `handle_online_msg(&mut self, msg: ProtocolMessage, addr: SocketAddr) -> Result<()>`
   - `handle_offline_msg(&mut self, addr: SocketAddr) -> Result<()>`
   - `get_all_peers(&self) -> Vec<PeerNode>`
   - `add_peer(&mut self, peer: PeerNode) -> Result<()>`

### 验证测试
1. 启动多个应用实例
2. 确认每个实例的节点列表包含其他实例
3. 关闭一个实例，确认其他实例在超时后移除该节点

### 预期结果
- 节点列表正确维护所有在线节点
- 节点离线后自动从列表移除

---

## 2.5 实现心跳机制

### 目标
通过定期心跳包维持节点在线状态

### 指令
1. 在 `src-tauri/src/modules/peer/` 创建 `heartbeat.rs`
2. 实现 `HeartbeatMonitor` 结构体
3. 实现以下功能：
   - 每 30 秒发送一次心跳包（STATUS_HEARTBEAT）
   - 接收心跳包时更新节点的 last_seen 时间
   - 检查超过 60 秒未收到心跳的节点，标记为离线
4. 使用 tokio 的 `Interval` 实现定时器

### 验证测试
1. 启动应用，观察心跳日志
2. 断开网络，确认节点在超时后标记为离线
3. 恢复网络，确认节点重新上线

### 预期结果
- 心跳包定期发送
- 离线节点被正确检测

---

# 阶段 3：Tauri 命令层

## 3.1 创建节点查询命令

### 目标
实现前端获取节点列表的 Tauri 命令

### 指令
1. 在 `src-tauri/src/commands/` 创建 `peer.rs`
2. 实现 `get_peers()` Tauri 命令：
   - 返回类型：`Vec<PeerDto>`
   - 从 PeerManager 获取所有节点
   - 将 PeerNode 转换为 PeerDto（用于序列化）
3. 定义 `PeerDto` 结构体，包含：
   - ip: String
   - username: String
   - hostname: String
   - status: String
   - last_seen: i64（时间戳）
4. 在 lib.rs 的 `invoke_handler` 中注册命令

### 验证测试
1. 在前端 `src/api/` 创建 `peer.ts`，封装 Tauri invoke 调用
2. 在 Vue 组件中调用 `invoke('get_peers')`
3. 使用 `tracing::info!` 打印返回结果
4. 确认前端可以获取节点列表

### 预期结果
- 前端成功获取节点列表
- 节点信息显示在开发者控制台

---

## 3.2 创建配置相关命令

### 目标
实现获取和更新配置的命令

### 指令
1. 在 `src-tauri/src/commands/` 创建 `config.rs`
2. 实现以下命令：
   - `get_config() -> Result<AppConfigDto>` - 获取当前配置
   - `update_config(config: AppConfigDto) -> Result<()>` - 更新配置
3. 定义 `AppConfigDto` 结构体，包含所有可配置项
4. 实现配置与数据库的同步

### 验证测试
1. 从前端调用 `get_config()`
2. 修改配置并调用 `update_config()`
3. 重启应用后调用 `get_config()`，确认配置持久化

### 预期结果
- 配置可以正确读取和更新
- 配置变更在重启后保持

---

## 3.3 创建状态监听命令

### 目标
实现前端监听节点状态变化的机制

### 指令
1. 使用 Tauri 的 Event 系统实现状态推送
2. 在节点状态变更时发送事件：
   - `peer://online` - 节点上线
   - `peer://offline` - 节点离线
   - `peer://update` - 节点信息更新
3. 事件携带节点信息（JSON 格式）

### 验证测试
1. 在前端使用 `listen()` 监听事件
2. 启动/停止其他应用实例
3. 确认前端接收到相应事件

### 预期结果
- 前端实时接收到节点状态变化事件

---

# 阶段 4：前端基础 UI

## 4.1 安装前端依赖

### 目标
配置 Vue 前端的必要依赖

### 指令
1. 运行 `npm install` 添加以下依赖：
   - `pinia` - 状态管理
   - `vue-router` - 路由
   - `tailwindcss` - 样式
   - `autoprefixer` - CSS 前缀
   - `postcss` - CSS 处理
2. 配置 Tailwind CSS（创建 `tailwind.config.js`）
3. 在 `src/main.ts` 中配置 Pinia 和 Vue Router

### 验证测试
1. 运行 `npm run dev`，确认 Vite 服务器启动
2. 确认 Tailwind CSS 类可以正常使用

### 预期结果
- 前端依赖安装完成
- Tailwind CSS 配置正确

---

## 4.2 创建 Pinia Store

### 目标
创建节点和配置的状态管理

### 指令
1. 在 `src/stores/` 创建以下文件：
   - `peer.ts` - 节点状态 store
   - `config.ts` - 配置状态 store
2. 实现 `usePeerStore`：
   - state: peers (ref<Peer[]>), loading
   - actions: fetchPeers(), refreshPeers()
   - getters: onlinePeers, offlinePeers
3. 实现 `useConfigStore`：
   - state: config
   - actions: loadConfig(), updateConfig()

### 验证测试
1. 在组件中使用 store
2. 确认状态正确响应式更新

### 预期结果
- Store 可以正确管理状态
- 组件可以访问 store 数据

---

## 4.3 创建节点列表组件

### 目标
实现显示在线节点的 UI 组件

### 指令
1. 在 `src/components/` 创建 `PeerList.vue`
2. 组件功能：
   - 从 store 获取节点列表
   - 显示节点信息（用户名、IP、状态）
   - 在线节点显示绿色图标，离线显示灰色
   - 每秒自动刷新列表
3. 使用 Tailwind CSS 样式
4. 组件大小控制在 200 行以内

### 验证测试
1. 在 App.vue 中使用 PeerList 组件
2. 启动多个应用实例
3. 确认所有节点显示在列表中
4. 关闭一个实例，确认状态更新

### 预期结果
- 节点列表正确显示所有在线节点
- 状态实时更新

---

## 4.4 创建设置页面组件

### 目标
实现应用设置的 UI 界面

### 指令
1. 在 `src/views/` 创建 `SettingsView.vue`
2. 创建表单，包含以下配置项：
   - 用户名（文本输入）
   - UDP 端口（数字输入）
   - TCP 端口范围（范围选择器）
   - 心跳间隔（数字输入）
3. 实现"保存"按钮，调用 `update_config` 命令
4. 显示保存成功/失败提示

### 验证测试
1. 修改配置并保存
2. 重启应用，确认配置保持
3. 测试边界值输入（如端口范围）

### 预期结果
- 配置可以正确修改和保存
- 表单验证正常工作

---

# 阶段 5：即时消息功能（基础）

## 5.1 实现消息数据结构

### 目标
定义消息的核心数据结构

### 指令
1. 在 `src-tauri/src/modules/message/` 创建 `types.rs`
2. 定义 `Message` 结构体，包含：
   - id: Uuid
   - packet_id: String
   - sender: PeerInfo
   - receiver: PeerInfo
   - msg_type: MessageType
   - content: String
   - timestamp: DateTime<Utc>
3. 定义 `MessageType` 枚举

### 验证测试
1. 创建测试消息实例
2. 确认所有字段正确赋值

### 预期结果
- 消息结构定义完整

---

## 5.2 实现消息发送功能

### 目标
实现 UDP 发送文本消息

### 指令
1. 在 `src-tauri/src/modules/message/` 创建 `handler.rs`
2. 实现 `MessageHandler` 结构体
3. 实现 `send_text_message(&self, target_ip: IpAddr, content: &str) -> Result<()>`
   - 使用 ProtocolMessage 封装消息
   - 消息类型为 MSG_SEND (0x00000004)
   - 通过 UDP 发送到目标节点

### 验证测试
1. 从一个实例发送消息到另一个实例
2. 使用 Wireshark 抓包确认 UDP 数据包
3. 接收方确认收到消息

### 预期结果
- 消息成功发送到目标节点

---

## 5.3 实现消息接收功能

### 目标
实现接收并处理入站消息

### 指令
1. 扩展 `PeerDiscovery` 的 `listen_incoming` 功能
2. 解析接收到的消息
3. 根据 msg_type 分发到不同处理器：
   - MSG_SEND -> 调用 MessageHandler
   - STATUS_ONLINE -> PeerManager
   - STATUS_HEARTBEAT -> HeartbeatMonitor
4. 接收到 MSG_SEND 后存储到数据库

### 验证测试
1. 发送消息，确认接收方存入数据库
2. 使用 SQLite 工具查询 messages 表

### 预期结果
- 接收的消息正确存储

---

## 5.4 创建消息查询命令

### 目标
前端可以查询与特定节点的聊天记录

### 指令
1. 在 `src-tauri/src/commands/message.rs` 创建命令
2. 实现 `get_messages(peer_ip: String, limit: u64) -> Result<Vec<MessageDto>>`
3. 从 MessageRepository 查询消息
4. 转换为 DTO 返回

### 验证测试
1. 从前端调用命令
2. 确认返回正确的消息列表

### 预期结果
- 前端可以获取聊天记录

---

## 5.5 创建聊天窗口组件

### 目标
实现发送和接收消息的 UI

### 指令
1. 在 `src/components/` 创建 `ChatWindow.vue`
2. 组件功能：
   - 显示消息列表（时间、发送者、内容）
   - 输入框和发送按钮
   - 调用 `send_message` 命令
   - 定时刷新消息列表
3. 监听 Tauri 事件，实时显示接收的消息

### 验证测试
1. 在两个实例间发送消息
2. 确认消息双向显示正确
3. 测试边界情况（空消息、超长消息）

### 预期结果
- 聊天功能正常工作
- 消息实时显示

---

# 阶段 6：文件传输（基础）

## 6.1 实现文件元数据计算

### 目标
计算文件的 MD5 和大小

### 指令
1. 添加依赖：`md-5 = "0.10"`
2. 在 `src-tauri/src/utils/` 创建 `hash.rs`
3. 实现 `calculate_file_md5(path: &Path) -> Result<String>`
4. 实现 `get_file_size(path: &Path) -> Result<u64>`

### 验证测试
1. 创建测试文件
2. 计算 MD5，与工具（如certutil）对比
3. 确认文件大小正确

### 预期结果
- MD5 计算正确
- 文件大小正确

---

## 6.2 实现文件传输请求

### 目标
发起文件传输请求

### 指令
1. 在 `src-tauri/src/modules/file_transfer/` 创建 `manager.rs`
2. 实现 `FileTransferManager`
3. 实现 `send_request(path: &Path, target: IpAddr) -> Result<Uuid>`
   - 计算文件 MD5 和大小
   - 创建 FILE_SEND_REQ 消息（包含文件名、大小、MD5）
   - 通过 UDP 发送
   - 创建传输任务，状态为 Pending

### 验证测试
1. 发送文件传输请求
2. 接收方确认收到请求
3. 使用 Wireshark 抓包验证

### 预期结果
- 文件传输请求成功发送

---

## 6.3 实现文件传输响应

### 目标
接收方响应文件传输请求

### 指令
1. 实现接收方处理逻辑
2. 收到 FILE_SEND_REQ 后：
   - 解析文件信息
   - 询问用户是否接受（通过 Tauri 事件）
   - 如果接受，分配 TCP 端口，发送 FILE_SEND_RSP
3. 如果拒绝，发送拒绝响应

### 验证测试
1. 发送文件请求
2. 确认接收方弹出确认对话框
3. 测试接受和拒绝两种情况

### 预期结果
- 文件请求正确处理
- 响应正确返回

---

## 6.4 实现 TCP 文件传输

### 目标
通过 TCP 传输文件数据

### 指令
1. 在 `src-tauri/src/network/` 创建 `tcp.rs`
2. 实现 `TcpTransport`，包含：
   - `bind_available() -> Result<(TcpListener, u16)>` - 绑定可用端口
   - `connect(addr: SocketAddr) -> Result<TcpStream>`
3. 实现文件发送：
   - 分块读取文件（每块 4KB）
   - 通过 TCP 流发送
   - 更新传输进度
4. 实现文件接收：
   - 创建临时文件
   - 接收数据块并写入
   - 传输完成后计算 MD5 验证

### 验证测试
1. 传输小文件（< 1MB）
2. 传输大文件（> 10MB）
3. 验证 MD5 一致
4. 确认文件可以正常打开

### 预期结果
- 文件正确传输
- 完整性校验通过

---

## 6.5 创建文件传输 UI 组件

### 目标
显示文件传输进度和状态

### 指指令
1. 在 `src/components/` 创建 `FileTransfer.vue`
2. 显示内容：
   - 文件名
   - 传输方向（上传/下载）
   - 进度条
   - 传输速度
   - 剩余时间
   - 暂停/取消按钮
3. 监听 Tauri 事件获取进度更新

### 验证测试
1. 发送文件
2. 确认进度条正确更新
3. 测试暂停和取消功能

### 预期结果
- 文件传输 UI 正常工作

---

# 阶段 7：测试和优化

## 7.1 编写集成测试

### 目标
覆盖核心功能的集成测试

### 指令
1. 在 `src-tauri/tests/` 创建集成测试
2. 测试场景：
   - 节点发现和状态同步
   - 消息发送和接收
   - 文件传输
   - 心跳和离线检测
3. 使用 `tokio::test` 异步测试

### 验证测试
1. 运行 `cargo test`
2. 确认所有测试通过

### 预期结果
- 核心功能有测试覆盖

---

## 7.2 性能测试

### 目标
验证性能指标达标

### 指令
1. 测试消息延迟：发送消息并测量接收时间
2. 测试文件传输速度：传输 100MB 文件
3. 测试并发传输：同时传输 3 个文件
4. 记录测试结果

### 验证测试
1. 消息延迟 ≤ 300ms
2. 传输速度 ≥ 10MB/s（100Mbps 网络）
3. 并发传输正常工作

### 预期结果
- 所有性能指标达标

---

## 7.3 内存泄漏检查

### 目标
确保应用长时间运行无内存泄漏

### 指令
1. 运行应用 24 小时
2. 定期发送和接收消息
3. 监控内存占用
4. 使用 Valgrind（Linux）或类似工具检查

### 验证测试
1. 内存占用稳定
2. 无明显增长趋势

### 预期结果
- 无内存泄漏

---

# 阶段 8：打包和部署

## 8.1 配置应用元数据

### 目标
设置应用名称、图标、版本信息

### 指令
1. 编辑 `src-tauri/tauri.conf.json`
2. 设置应用名称、描述
3. 添加应用图标
4. 配置安装选项

### 验证测试
1. 运行 `npm run tauri build`
2. 检查生成的安装包

### 预期结果
- 应用元数据正确显示

---

## 8.2 构建生产版本

### 目标
生成可分发的安装包

### 指令
1. 运行 `npm run tauri build`
2. 配置 release 优化（LTO、strip）
3. 检查输出目录

### 验证测试
1. 安装生成的安装包
2. 运行应用，确认功能正常

### 预期结果
- 可以正常安装和运行

---

# 后续阶段（高级功能）

以下功能在基础功能完成后按需实现：

## 阶段 9：消息加密

- 实现 AES-256-GCM 加密
- 实现 ECDH 密钥交换
- 添加加密开关配置

## 阶段 10：离线消息

- 实现离线消息缓存
- 实现上线后推送
- 添加离线消息限制

## 阶段 11：群组功能

- 实现群组创建和管理
- 实现群组消息广播
- 添加群组 UI

## 阶段 12：断点续传

- 实现文件分片
- 记录传输进度
- 实现暂停和恢复

## 阶段 13：高级 UI

- 实现主题切换
- 实现多语言支持
- 添加表情和图片发送

---

# 附录

## A. 里程碑检查清单

- [ ] 阶段 0：基础设施完成
- [ ] 阶段 1：数据持久化完成
- [ ] 阶段 2：网络通信完成
- [ ] 阶段 3：Tauri 命令完成
- [ ] 阶段 4：前端 UI 完成
- [ ] 阶段 5：即时消息完成
- [ ] 阶段 6：文件传输完成
- [ ] 阶段 7：测试和优化完成
- [ ] 阶段 8：打包部署完成

## B. 文档更新责任

每个阶段完成后，更新以下文档：
- `memory-bank/@architecture.md` - 添加新模块、更新数据库变更
- `tech-stack.md` - 更新依赖版本

## C. 代码审查要点

每个 PR 检查：
- 文件大小是否超过限制
- 是否遵循模块化规则
- 是否有对应的测试
- 是否有错误处理

---

**文档结束**
