# NeoLan 局域网即时通信系统设计文档

## 文档信息

| 项目 | 内容 |
|------|------|
| 文档名称 | NeoLan 局域网即时通信系统设计文档 |
| 版本号 | V1.0 |
| 编制日期 | 2026-01-05 |
| 文档状态 | 初稿 |

---

# 1. 文档概述

## 1.1 文档目的

本文档是NeoLan局域网即时通信系统的全面设计文档，旨在为开发团队提供清晰的技术实现指导。文档涵盖产品定位、技术架构、通信协议、功能模块、数据模型、安全设计、性能优化等核心内容，确保系统设计的一致性、可扩展性和可维护性。

## 1.2 产品定位

NeoLan是一款面向企业内部局域网环境的即时通信与文件协作系统，采用去中心化P2P架构，无需中心服务器即可实现高效通信。产品定位于：

- **场景定位**：企业办公局域网、团队协作环境、临时项目组网络
- **用户定位**：企业员工、项目团队成员、办公网络用户
- **价值定位**：零部署成本、即装即用、数据私有可控、高速文件传输

## 1.3 核心特性

| 特性类别 | 核心特性 |
|---------|---------|
| 架构特性 | 去中心化P2P、无服务器依赖、跨平台兼容 |
| 通信特性 | 即时消息、群组聊天、离线消息、端到端加密 |
| 传输特性 | 高速文件传输、断点续传、文件夹传输、超大文件支持 |
| 协作特性 | 屏幕共享、远程协助、共享剪贴板 |
| 管理特性 | 好友分组、权限控制、黑名单、日志审计 |

---

# 2. 技术架构设计

## 2.1 整体架构

NeoLan采用去中心化P2P（Peer-to-Peer）架构，所有客户端节点地位对等，通过局域网直接交互。

```
┌─────────────────────────────────────────────────────────────────┐
│                        局域网环境                                │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐           │
│  │ 节点 A  │◄─┼─────────┼─►│ 节点 B  │           │
│  │ NeoLan  │  │ UDP/TCP │  │ NeoLan  │  ...      │
│  └─────────┘  └─────────┘  └─────────┘           │
│       ▲                                    ▲     │
│       └──────────── P2P通信 ───────────────┘     │
│                                                  │
│  无中心服务器 · 节点对等 · 直接通信               │
└─────────────────────────────────────────────────────────────────┘
```

## 2.2 核心模块架构

系统由六大核心模块构成，模块间职责清晰、松耦合设计：

### 2.2.1 模块架构图

```
┌────────────────────────────────────────────────────────────────┐
│                        UI交互模块                               │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐                  │
│  │ 好友列表  │  │ 聊天窗口  │  │ 设置界面  │                  │
│  └───────────┘  └───────────┘  └───────────┘                  │
└────────────────────────────────────────────────────────────────┘
                            ▲
                            │
┌────────────────────────────────────────────────────────────────┐
│                      业务协调层                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │ 好友管理模块  │  │ 消息处理模块  │  │ 文件传输模块  │        │
│  └──────────────┘  └──────────────┘  └──────────────┘        │
└────────────────────────────────────────────────────────────────┘
                            ▲
                            │
┌────────────────────────────────────────────────────────────────┐
│                      网络通信模块                               │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐                  │
│  │ UDP通信   │  │ TCP通信   │  │ 协议解析  │                  │
│  └───────────┘  └───────────┘  └───────────┘                  │
└────────────────────────────────────────────────────────────────┘
                            ▲
                            │
┌────────────────────────────────────────────────────────────────┐
│                      数据存储层                                 │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐                  │
│  │ 配置存储  │  │ 消息存储  │  │ 传输记录  │                  │
│  └───────────┘  └───────────┘  └───────────┘                  │
└────────────────────────────────────────────────────────────────┘
```

### 2.2.2 模块职责定义

| 模块名称 | 核心职责 | 技术选型 |
|---------|---------|---------|
| 网络通信模块 | UDP/TCP协议封装、节点发现、数据收发 | 原生Socket、异步I/O |
| 消息处理模块 | 协议解析、消息封装、加密解密、消息路由 | 协议处理器、加密库 |
| 文件传输模块 | 文件分片、断点续传、传输控制、完整性校验 | 分片算法、MD5校验 |
| 好友管理模块 | 节点发现、状态同步、分组管理、权限控制 | 心跳机制、状态机 |
| UI交互模块 | 界面渲染、事件处理、用户交互反馈 | 跨平台UI框架 |
| 存储模块 | 数据持久化、缓存管理、数据导入导出 | SQLite/文件存储 |

## 2.3 技术栈选型

### 2.3.1 前端技术栈

| 技术领域 | 选型方案 | 选型理由 |
|---------|---------|---------|
| 桌面框架 | Tauri 2.x | 轻量级、安全性高、Rust后端 |
| UI框架 | Vue 3 + TypeScript | 生态成熟、组件化开发、类型安全 |
| 状态管理 | Pinia | Vue官方推荐、轻量简洁 |
| 样式方案 | Tailwind CSS | 原子化CSS、快速开发、样式一致性 |
| 构建工具 | Vite | 快速热更新、原生ESM支持 |

### 2.3.2 后端技术栈

| 技术领域 | 选型方案 | 选型理由 |
|---------|---------|---------|
| 后端语言 | Rust | 内存安全、高性能、跨平台 |
| 网络库 | tokio + tokio-util | 异步运行时、高并发处理 |
| 序列化 | serde + serde_json | 类型安全、性能优异 |
| 加密库 | aes-gcm + rand | AES-256-GCM加密、安全随机数 |
| 数据库 | SQLite | 轻量级、无需服务器、事务支持 |

---

# 3. 通信协议设计

## 3.1 协议概述

NeoLan基于飞鸽传书(IPMsg)协议设计，兼容IPMsg节点通信，同时扩展加密传输、大文件传输等能力。

### 3.1.1 协议基础属性

| 属性 | 配置 |
|------|------|
| 控制协议 | UDP（低延迟） |
| 数据协议 | TCP（可靠传输） |
| 默认端口 | UDP: 2425, TCP: 8000-9000（动态分配） |
| 字符编码 | UTF-8 |
| 协议版本 | 2.0（兼容IPMsg 1.0） |

## 3.2 协议数据格式

### 3.2.1 基础协议格式

```
版本号:包编号:发送方用户名:发送方机器名:消息类型:消息内容[:扩展字段]
```

### 3.2.2 字段定义

| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| 版本号 | String | 协议版本，"1"兼容IPMsg，"2"为NeoLan扩展版 | "2" |
| 包编号 | u64 | 唯一标识，用于去重和回执确认 | 1736102400 |
| 发送方用户名 | String | 用户昵称 | "张三-技术部" |
| 发送方机器名 | String | 设备主机名 | "DESKTOP-ABC123" |
| 消息类型 | u32 | 消息类型枚举 | 0x00000004 |
| 消息内容 | String | 业务数据（文本/JSON/Base64） | "Hello World" |
| 加密标识 | String(扩展) | "1"加密，"0"未加密 | "1" |
| 密钥索引 | String(扩展) | 加密密钥索引 | "0" |

## 3.3 消息类型定义

基于 IPMsg 协议标准（兼容 IPMsg/飞鸽传书/飞秋）：

### 3.3.1 协议常量

| 常量名 | 值 | 说明 |
|--------|-----|------|
| IPMSG_VERSION | 0x0001 | 协议版本 |
| IPMSG_DEFAULT_PORT | 2425 (0x0979) | 默认 UDP 端口 |

### 3.3.2 核心消息类型（mode - 低 8 位）

| 类型值 | 消息名称 | 传输协议 | 需要回执 | 说明 |
|--------|---------|---------|---------|------|
| 0x00000000 | IPMSG_NOOPERATION | - | 否 | 无操作 |
| 0x00000001 | IPMSG_BR_ENTRY | UDP | 否 | 广播上线 |
| 0x00000002 | IPMSG_BR_EXIT | UDP | 否 | 广播下线 |
| 0x00000003 | IPMSG_ANSENTRY | UDP | 否 | 对 BR_ENTRY 的应答 |
| 0x00000004 | IPMSG_BR_ABSENCE | UDP | 否 | 广播缺席 |
| 0x00000010 | IPMSG_BR_ISGETLIST | UDP | 否 | 请求列表 |
| 0x00000011 | IPMSG_OKGETLIST | UDP | 否 | 同意发送列表 |
| 0x00000012 | IPMSG_GETLIST | UDP | 否 | 请求列表 |
| 0x00000013 | IPMSG_ANSLIST | UDP | 否 | 返回列表 |
| 0x00000020 | IPMSG_SENDMSG | UDP/TCP | 是 | 发送消息 |
| 0x00000021 | IPMSG_RECVMSG | UDP | 否 | 接收确认 |
| 0x00000030 | IPMSG_READMSG | UDP | 否 | 消息已读 |
| 0x00000031 | IPMSG_DELMSG | UDP | 否 | 删除消息 |
| 0x00000060 | IPMSG_GETFILEDATA | UDP | 是 | 请求文件数据 |
| 0x00000061 | IPMSG_RELEASEFILES | UDP | 否 | 释放文件资源 |

### 3.3.3 选项标志（options - 高 24 位）

| 类型值 | 标志名称 | 说明 |
|--------|---------|------|
| 0x00000100 | IPMSG_SENDCHECKOPT | 发送确认 |
| 0x00000400 | IPMSG_BROADCASTOPT | 广播发送 |
| 0x00200000 | IPMSG_FILEATTACHOPT | 文件附加标志 |
| 0x00400000 | IPMSG_ENCRYPTOPT | 加密标志 |
| 0x00800000 | IPMSG_UTF8OPT | UTF-8 编码标志 |

**注**：选项标志通过位或运算与消息类型组合使用，例如：
- 加密消息：`IPMSG_SENDMSG | IPMSG_ENCRYPTOPT`
- 带文件的消息：`IPMSG_SENDMSG | IPMSG_FILEATTACHOPT`

### 3.3.4 扩展消息类型（NeoLan新增）

| 类型值 | 消息名称 | 传输协议 | 说明 |
|--------|---------|---------|------|
| 0x00010000 | CLIPBOARD_SYNC | UDP | 剪贴板同步 |
| 0x00020000 | SCREEN_SHARE_REQ | UDP | 屏幕共享请求 |
| 0x00040000 | SCREEN_SHARE_DATA | TCP | 屏幕共享数据 |
| 0x00080000 | REMOTE_CTRL | TCP | 远程控制指令 |
| 0x00100000 | ENCRYPTED_MSG | UDP/TCP | 加密消息 |
| 0x00200000 | VOICE_CALL_REQ | UDP | 语音通话请求 |
| 0x00400000 | VOICE_CALL_DATA | UDP | 语音通话数据 |

## 3.4 消息协议示例

### 3.4.1 上线广播消息

```
发送内容：
1:1736102400001:张三-技术部:DESKTOP-ABC123:0x00000001:

字段解析：
- 版本号：1（兼容IPMsg）
- 包编号：1736102400001（时间戳+随机数）
- 发送方用户名：张三-技术部
- 发送方机器名：DESKTOP-ABC123
- 消息类型：0x00000001（上线广播）
- 消息内容：空
```

### 3.4.2 文本消息

```
发送内容：
1:1736102400002:张三-技术部:DESKTOP-ABC123:0x00000020:今天下午3点开项目例会

字段解析：
- 版本号：1（兼容 IPMsg）
- 包编号：1736102400002
- 发送方用户名：张三-技术部
- 发送方机器名：DESKTOP-ABC123
- 消息类型：0x00000020（IPMSG_SENDMSG 文本消息）
- 消息内容：今天下午3点开项目例会
```

### 3.4.3 文件传输请求

```
发送内容：
1:1736102400003:张三-技术部:DESKTOP-ABC123:0x00000060:{"name":"项目文档.docx","size":1024000,"md5":"d41d8cd98f00b204e9800998ecf8427e"}

字段解析：
- 版本号：1（兼容 IPMsg）
- 包编号：1736102400003
- 发送方用户名：张三-技术部
- 发送方机器名：DESKTOP-ABC123
- 消息类型：0x00000060（IPMSG_GETFILEDATA 文件传输请求）
- 消息内容：JSON格式的文件元数据
  {
    "name": "项目文档.docx",           // 文件名
    "size": 1024000,                   // 文件大小（字节）
    "md5": "d41d8cd98f00b204e9800998ecf8427e"  // MD5校验值
  }
```

### 3.4.4 加密文本消息

```
发送内容：
1:1736102400004:张三-技术部:DESKTOP-ABC123:0x00400020:U2FsdGVkX1+vupppZksvRf5pq5g5XjFRlipRkwB0K1Y96Qsv2Lm+31cmzaAILwytJHoXyYvlVhPp0KWuV6qHO+Q==:1:0

字段解析：
- 版本号：1（兼容 IPMsg）
- 包编号：1736102400004
- 发送方用户名：张三-技术部
- 发送方机器名：DESKTOP-ABC123
- 消息类型：0x00400020（加密文本消息，IPMSG_SENDMSG | IPMSG_ENCRYPTOPT）
- 消息内容：Base64编码的AES-256加密数据
- 加密标识：1（已加密）
- 密钥索引：0（密钥库索引）
```

---

# 4. 功能模块设计

## 4.1 网络通信模块

### 4.1.1 模块职责

- UDP通信：处理控制消息（上线、离线、心跳、消息回执）
- TCP通信：处理文件数据、大文本消息
- 节点发现：UDP广播扫描在线节点
- 连接管理：维护TCP连接池

### 4.1.2 UDP通信设计

```rust
// UDP通信核心结构
pub struct UdpTransport {
    socket: UdpSocket,
    port: u16,
    buffer: [u8; 65535],
}

impl UdpTransport {
    // 发送广播消息
    pub async fn broadcast(&self, data: &[u8]) -> Result<()> {
        self.socket.send_to(data, "255.255.255.255:2425").await?;
        Ok(())
    }

    // 发送单播消息
    pub async fn send_to(&self, data: &[u8], target: &SocketAddr) -> Result<()> {
        self.socket.send_to(data, target).await?;
        Ok(())
    }

    // 接收消息（循环）
    pub async fn recv_loop(&self) -> Result<(Vec<u8>, SocketAddr)> {
        let (len, addr) = self.socket.recv_from(&mut self.buffer).await?;
        Ok((self.buffer[..len].to_vec(), addr))
    }
}
```

### 4.1.3 TCP通信设计

```rust
// TCP传输管理器
pub struct TcpTransportManager {
    connections: HashMap<SocketAddr, TcpStream>,
    port_range: Range<u16>,
}

impl TcpTransportManager {
    // 创建TCP服务器
    pub async fn bind(&self, port: u16) -> Result<TcpListener> {
        let listener = TcpListener::bind(("0.0.0.0", port)).await?;
        Ok(listener)
    }

    // 连接到目标节点
    pub async fn connect(&mut self, addr: SocketAddr) -> Result<TcpStream> {
        if let Some(stream) = self.connections.get(&addr) {
            return Ok(stream.try_clone()?);
        }
        let stream = TcpStream::connect(addr).await?;
        self.connections.insert(addr, stream.try_clone()?);
        Ok(stream)
    }
}
```

## 4.2 消息处理模块

### 4.2.1 模块职责

- 协议解析：解析接收的消息包
- 消息封装：封装待发送的消息
- 加密解密：AES-256加密处理
- 消息路由：分发消息到对应处理器
- 离线消息：缓存和推送离线消息

### 4.2.2 消息处理器设计

```rust
// 消息处理器接口
pub trait MessageHandler: Send + Sync {
    async fn handle(&self, msg: ParsedMessage) -> Result<()>;
}

// 解析后的消息结构
pub struct ParsedMessage {
    pub version: u8,
    pub packet_id: u64,
    pub sender_name: String,
    pub sender_host: String,
    pub msg_type: u32,
    pub content: String,
    pub encrypted: bool,
    pub key_index: Option<u32>,
    pub source_addr: SocketAddr,
}

// 消息路由器
pub struct MessageRouter {
    handlers: HashMap<u32, Box<dyn MessageHandler>>,
}

impl MessageRouter {
    pub fn register(&mut self, msg_type: u32, handler: Box<dyn MessageHandler>) {
        self.handlers.insert(msg_type, handler);
    }

    pub async fn route(&self, msg: ParsedMessage) -> Result<()> {
        if let Some(handler) = self.handlers.get(&msg.msg_type) {
            handler.handle(msg).await?;
        }
        Ok(())
    }
}
```

### 4.2.3 加密处理设计

```rust
// 加密管理器
pub struct CryptoManager {
    keys: Vec<[u8; 32]>, // AES-256密钥库
}

impl CryptoManager {
    // 加密消息
    pub fn encrypt(&self, plaintext: &[u8], key_index: usize) -> Result<Vec<u8>> {
        let key = &self.keys[key_index];
        let cipher = Aes256Gcm::new(key.into());
        let nonce = Aes256Gcm::generate_nonce(&mut rand::thread_rng());
        let ciphertext = cipher.encrypt(&nonce, plaintext)?;
        // 返回 nonce + ciphertext
        let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    // 解密消息
    pub fn decrypt(&self, data: &[u8], key_index: usize) -> Result<Vec<u8>> {
        let key = &self.keys[key_index];
        let cipher = Aes256Gcm::new(key.into());
        let (nonce, ciphertext) = data.split_at(12);
        let plaintext = cipher.decrypt(nonce.into(), ciphertext)?;
        Ok(plaintext)
    }
}
```

## 4.3 文件传输模块

### 4.3.1 模块职责

- 文件分片：将文件拆分为固定大小的分片
- 传输控制：控制传输速率、并发数
- 断点续传：记录传输进度，支持断点恢复
- 完整性校验：MD5校验确保文件完整

### 4.3.2 文件分片传输设计

```rust
// 文件分片配置
pub struct ChunkConfig {
    pub chunk_size: usize,     // 分片大小（默认4KB）
    pub concurrent: usize,     // 并发传输数（默认3）
    pub retry_times: u8,       // 重试次数（默认3）
}

// 文件传输任务
pub struct FileTransferTask {
    pub task_id: Uuid,
    pub file_path: PathBuf,
    pub file_size: u64,
    pub file_md5: String,
    pub chunks: Vec<Chunk>,
    pub transferred: HashSet<usize>,
    pub status: TransferStatus,
}

// 分片信息
pub struct Chunk {
    pub index: usize,
    pub offset: u64,
    pub size: usize,
    pub md5: String,
}

// 文件传输管理器
pub struct FileTransferManager {
    config: ChunkConfig,
    tasks: HashMap<Uuid, FileTransferTask>,
    tcp_mgr: Arc<TcpTransportManager>,
}

impl FileTransferManager {
    // 发送文件
    pub async fn send_file(&mut self, path: &Path, target: SocketAddr) -> Result<Uuid> {
        // 1. 计算文件MD5
        let md5 = self.calculate_md5(path)?;

        // 2. 创建传输任务
        let task = FileTransferTask {
            task_id: Uuid::new_v4(),
            file_path: path.to_path_buf(),
            file_size: fs::metadata(path)?.len(),
            file_md5: md5,
            chunks: self.split_file(path)?,
            transferred: HashSet::new(),
            status: TransferStatus::Pending,
        };

        // 3. 发送传输请求
        self.send_file_request(&task, target).await?;

        // 4. 保存任务
        let task_id = task.task_id;
        self.tasks.insert(task_id, task);

        Ok(task_id)
    }

    // 接收文件
    pub async fn receive_file(&mut self, task_id: Uuid, save_path: &Path) -> Result<()> {
        let task = self.tasks.get(&task_id).ok_or("Task not found")?;

        // 1. 创建临时文件
        let temp_file = fs::File::create(save_path.with_extension("tmp"))?;

        // 2. 接收分片并写入
        for chunk in &task.chunks {
            let data = self.receive_chunk(task_id, chunk.index).await?;
            // 写入到对应偏移位置
            // ...
        }

        // 3. 校验完整性
        let md5 = self.calculate_md5(save_path)?;
        if md5 != task.file_md5 {
            return Err("MD5 mismatch".into());
        }

        // 4. 重命名为正式文件
        fs::rename(save_path.with_extension("tmp"), save_path)?;

        Ok(())
    }
}
```

### 4.3.3 断点续传设计

```rust
// 断点续传状态
pub struct ResumeState {
    pub task_id: Uuid,
    pub transferred_chunks: HashSet<usize>,
    pub last_offset: u64,
}

impl FileTransferManager {
    // 暂停传输
    pub async fn pause(&mut self, task_id: Uuid) -> Result<()> {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.status = TransferStatus::Paused;
            // 保存断点状态
            self.save_resume_state(task)?;
        }
        Ok(())
    }

    // 恢复传输
    pub async fn resume(&mut self, task_id: Uuid) -> Result<()> {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            // 从断点位置继续传输
            for chunk in &task.chunks {
                if !task.transferred.contains(&chunk.index) {
                    self.send_chunk(task_id, chunk).await?;
                }
            }
            task.status = TransferStatus::Transferring;
        }
        Ok(())
    }
}
```

## 4.4 好友管理模块

### 4.4.1 模块职责

- 节点发现：UDP广播发现在线节点
- 状态同步：维护节点在线状态
- 分组管理：创建、编辑好友分组
- 黑名单：管理黑名单用户

### 4.4.2 节点发现设计

```rust
// 好友节点信息
#[derive(Clone, Debug)]
pub struct PeerNode {
    pub ip: IpAddr,
    pub port: u16,
    pub username: String,
    pub hostname: String,
    pub avatar: Option<String>,
    pub status: PeerStatus,
    pub groups: Vec<String>,
    pub last_seen: SystemTime,
}

// 节点状态
#[derive(Clone, Debug, PartialEq)]
pub enum PeerStatus {
    Online,
    Offline,
    Away,
    Busy,
    Invisible,
}

// 好友管理器
pub struct PeerManager {
    pub peers: HashMap<IpAddr, PeerNode>,
    pub groups: HashMap<String, Vec<IpAddr>>,
    pub blacklist: HashSet<IpAddr>,
    heartbeat_interval: Duration,
    heartbeat_timeout: Duration,
}

impl PeerManager {
    // 启动节点发现
    pub async fn start_discovery(&mut self) -> Result<()> {
        // 1. 发送上线广播
        self.broadcast_online().await?;

        // 2. 启动心跳任务
        let peers = self.peers.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;
                // 发送心跳包
                Self::send_heartbeat(&peers).await;
            }
        });

        // 3. 启动超时检测
        let peers = self.peers.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                // 检测超时节点
                Self::check_timeout(&mut peers.clone()).await;
            }
        });

        Ok(())
    }

    // 处理节点上线
    pub async fn handle_peer_online(&mut self, node: PeerNode) {
        self.peers.insert(node.ip, node.clone());
        // 回传上线消息（双向发现）
        self.respond_online(node.ip).await;
    }

    // 处理节点离线
    pub async fn handle_peer_offline(&mut self, ip: IpAddr) {
        self.peers.remove(&ip);
    }

    // 处理心跳包
    pub async fn handle_heartbeat(&mut self, ip: IpAddr) {
        if let Some(peer) = self.peers.get_mut(&ip) {
            peer.last_seen = SystemTime::now();
        }
    }
}
```

## 4.5 存储模块

### 4.5.1 模块职责

- 配置存储：用户配置、系统设置
- 消息存储：聊天记录、离线消息
- 传输记录：文件传输历史
- 缓存管理：临时数据缓存

### 4.5.2 数据库设计

```sql
-- 好友表
CREATE TABLE peers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ip TEXT NOT NULL UNIQUE,
    port INTEGER NOT NULL,
    username TEXT NOT NULL,
    hostname TEXT NOT NULL,
    avatar TEXT,
    nickname TEXT,
    groups TEXT,
    last_seen INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 消息表
CREATE TABLE messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    msg_id TEXT NOT NULL UNIQUE,
    sender_ip TEXT NOT NULL,
    sender_name TEXT NOT NULL,
    receiver_ip TEXT NOT NULL,
    msg_type INTEGER NOT NULL,
    content TEXT NOT NULL,
    is_encrypted INTEGER NOT NULL DEFAULT 0,
    is_offline INTEGER NOT NULL DEFAULT 0,
    sent_at INTEGER NOT NULL,
    received_at INTEGER,
    created_at INTEGER NOT NULL
);

-- 传输记录表
CREATE TABLE transfers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id TEXT NOT NULL UNIQUE,
    direction TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    file_md5 TEXT NOT NULL,
    peer_ip TEXT NOT NULL,
    peer_name TEXT NOT NULL,
    status TEXT NOT NULL,
    transferred_size INTEGER NOT NULL DEFAULT 0,
    started_at INTEGER,
    completed_at INTEGER,
    created_at INTEGER NOT NULL
);

-- 配置表
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 分组表
CREATE TABLE groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
```

### 4.5.3 存储接口设计

```rust
// 存储管理器
pub struct StorageManager {
    db: Connection,
}

impl StorageManager {
    // 保存消息
    pub fn save_message(&self, msg: &StoredMessage) -> Result<()> {
        self.db.execute(
            "INSERT INTO messages (msg_id, sender_ip, sender_name, receiver_ip, msg_type, content, is_encrypted, is_offline, sent_at, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                msg.msg_id,
                msg.sender_ip,
                msg.sender_name,
                msg.receiver_ip,
                msg.msg_type,
                msg.content,
                msg.is_encrypted,
                msg.is_offline,
                msg.sent_at,
                msg.created_at,
            ],
        )?;
        Ok(())
    }

    // 查询聊天记录
    pub fn get_messages(&self, peer_ip: &str, limit: usize) -> Result<Vec<StoredMessage>> {
        let mut stmt = self.db.prepare(
            "SELECT * FROM messages WHERE (sender_ip = ?1 OR receiver_ip = ?1) ORDER BY sent_at DESC LIMIT ?2"
        )?;
        let messages = stmt.query_map(params![peer_ip, limit], |row| {
            Ok(StoredMessage {
                id: row.get(0)?,
                msg_id: row.get(1)?,
                sender_ip: row.get(2)?,
                sender_name: row.get(3)?,
                receiver_ip: row.get(4)?,
                msg_type: row.get(5)?,
                content: row.get(6)?,
                is_encrypted: row.get(7)?,
                is_offline: row.get(8)?,
                sent_at: row.get(9)?,
                received_at: row.get(10)?,
                created_at: row.get(11)?,
            })
        })?;
        // ...
    }

    // 保存传输记录
    pub fn save_transfer(&self, transfer: &TransferRecord) -> Result<()> {
        // ...
    }
}
```

---

# 5. 数据模型设计

## 5.1 核心数据结构

### 5.1.1 消息数据结构

```rust
// 消息数据结构
pub struct Message {
    pub id: u64,
    pub packet_id: String,
    pub sender: PeerInfo,
    pub receiver: PeerInfo,
    pub msg_type: MessageType,
    pub content: MessageContent,
    pub encrypted: bool,
    pub timestamp: DateTime<Utc>,
    pub status: MessageStatus,
}

pub enum MessageContent {
    Text(String),
    File(FileMetadata),
    Image(ImageData),
    Clipboard(ClipboardData),
    System(SystemMessage),
}

pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Read,
    Failed,
}
```

### 5.1.2 文件传输数据结构

```rust
// 文件传输任务
pub struct FileTransfer {
    pub id: Uuid,
    pub direction: TransferDirection,
    pub file: FileMetadata,
    pub peer: PeerInfo,
    pub status: TransferStatus,
    pub progress: TransferProgress,
    pub created_at: DateTime<Utc>,
}

pub enum TransferDirection {
    Upload,
    Download,
}

pub enum TransferStatus {
    Pending,
    WaitingAccept,
    Transferring,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

pub struct TransferProgress {
    pub transferred_bytes: u64,
    pub total_bytes: u64,
    pub speed: f64,        // bytes/s
    pub eta: Option<Duration>,
    pub chunks_completed: usize,
    pub chunks_total: usize,
}
```

### 5.1.3 好友节点数据结构

```rust
// 节点信息
pub struct PeerInfo {
    pub ip: IpAddr,
    pub port: u16,
    pub username: String,
    pub hostname: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub status: PeerStatus,
    pub groups: Vec<String>,
    pub capabilities: PeerCapabilities,
    pub last_seen: DateTime<Utc>,
    pub is_blocked: bool,
}

pub struct PeerCapabilities {
    pub supports_encryption: bool,
    pub supports_large_file: bool,
    pub supports_folder: bool,
    pub supports_clipboard: bool,
    pub supports_screen_share: bool,
    pub supports_voice_call: bool,
    pub max_message_size: usize,
    pub max_file_size: u64,
}
```

## 5.2 状态机设计

### 5.2.1 节点状态机

```
                    ┌───────────┐
                    │   初始    │
                    └─────┬─────┘
                          │
                          ▼
                    ┌───────────┐
                    │   离线    │◄─────────────────┐
                    └─────┬─────┘                  │
                          │                        │
                          │ 上线广播               │ 离线通知
                          ▼                        │
                    ┌───────────┐                  │
                    │   在线    │──────────────────┤
                    └─────┬─────┘  状态变更        │
                          │                        │
         ┌────────────────┼────────────────┐      │
         │                │                │      │
         ▼                ▼                ▼      │
    ┌─────────┐     ┌─────────┐     ┌─────────┐  │
    │  离开   │     │  忙碌   │     │ 隐身   │───┘
    └────┬────┘     └────┬────┘     └────┬────┘
         │                │                │
         └────────────────┴────────────────┘
                          │
                          ▼ 恢复操作
                    ┌───────────┐
                    │   在线    │
                    └───────────┘
```

### 5.2.2 文件传输状态机

```
                    ┌───────────┐
                    │   初始    │
                    └─────┬─────┘
                          │
                          ▼
                    ┌───────────┐
                    │   等待    │◄────────┐
                    │  接受     │         │
                    └─────┬─────┘         │
                          │              │
         ┌────────────────┼────────┐    │
         │ 拒绝           │ 接受   │    │
         ▼                ▼        │    │
    ┌─────────┐     ┌───────────┐  │    │
    │  取消   │     │ 传输中    │  │    │
    └─────────┘     └─────┬─────┘  │    │
                          │        │    │
         ┌────────────────┼────────┐    │
         │ 暂停           │ 完成   │    │
         ▼                ▼        │    │
    ┌─────────┐     ┌─────────┐  │    │
    │  暂停   │     │  完成   │  │    │
    └────┬────┘     └─────────┘  │    │
         │                      │    │
         │ 恢复                 │    │
         └──────────────────────┘    │
                                   │
                                   ▼
                            ┌─────────┐
                            │  失败   │
                            └─────────┘
```

---

# 6. 安全设计

## 6.1 加密机制

### 6.1.1 加密算法

| 加密场景 | 算法 | 密钥长度 | 说明 |
|---------|------|---------|------|
| 消息加密 | AES-256-GCM | 256位 | 认证加密，防篡改 |
| 密钥交换 | ECDH | 256位 | 椭圆曲线Diffie-Hellman |
| 签名验证 | Ed25519 | 256位 | 数字签名 |

### 6.1.2 密钥管理

```rust
// 密钥管理器
pub struct KeyManager {
    pub master_key: [u8; 32],
    pub peer_keys: HashMap<IpAddr, PeerKey>,
}

pub struct PeerKey {
    pub shared_secret: [u8; 32],
    pub key_index: u32,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

impl KeyManager {
    // 密钥协商（ECDH）
    pub fn negotiate_key(&mut self, peer_pubkey: &[u8]) -> Result<[u8; 32]> {
        let secret = self.ecdh_exchange(peer_pubkey)?;
        let shared_key = self.derive_key(&secret)?;
        Ok(shared_key)
    }

    // 密钥轮换
    pub fn rotate_key(&mut self, peer: IpAddr) -> Result<()> {
        // 生成新密钥
        let new_key = self.generate_key()?;
        // 更新密钥索引
        if let Some(peer_key) = self.peer_keys.get_mut(&peer) {
            peer_key.shared_secret = new_key;
            peer_key.key_index += 1;
        }
        Ok(())
    }
}
```

### 6.1.3 消息加密流程

```
发送方流程：
1. 生成随机nonce（12字节）
2. 使用AES-256-GCM加密消息
3. 附加认证标签（16字节）
4. 拼接：nonce + 密文 + 标签
5. Base64编码后传输

接收方流程：
1. Base64解码数据
2. 分离nonce、密文、标签
3. 使用AES-256-GCM解密
4. 验证认证标签
5. 输出明文消息
```

## 6.2 权限控制

### 6.2.1 权限模型

```rust
// 权限定义
#[derive(BitFlags, Clone, Copy, PartialEq)]
pub enum Permission {
    SendMessage   = 0b00000001,
    SendFile      = 0b00000010,
    ReceiveFile   = 0b00000100,
    RemoteAssist  = 0b00001000,
    ScreenShare   = 0b00010000,
    ClipboardSync = 0b00100000,
    VoiceCall     = 0b01000000,
}

// 权限规则
pub struct PermissionRule {
    pub peer: PeerSelector,
    pub permissions: BitFlags<Permission>,
    pub expires_at: Option<DateTime<Utc>>,
}

pub enum PeerSelector {
    All,
    Group(String),
    Specific(IpAddr),
    Except(Vec<IpAddr>),
}

// 权限管理器
pub struct PermissionManager {
    pub rules: Vec<PermissionRule>,
}

impl PermissionManager {
    pub fn check_permission(&self, peer: &IpAddr, perm: Permission) -> bool {
        for rule in &self.rules {
            if rule.matches(peer) && rule.permissions.contains(perm) {
                return true;
            }
        }
        false
    }
}
```

## 6.3 安全审计

### 6.3.1 审计日志

```rust
// 审计事件
pub enum AuditEvent {
    PeerOnline { ip: IpAddr, username: String },
    PeerOffline { ip: IpAddr },
    MessageSent { to: IpAddr, encrypted: bool },
    MessageReceived { from: IpAddr, encrypted: bool },
    FileTransfer { direction: String, file: String, peer: IpAddr },
    PermissionDenied { peer: IpAddr, action: String },
    EncryptionError { peer: IpAddr, error: String },
}

// 审计日志记录
pub struct AuditLogger {
    pub log_file: PathBuf,
    pub rotation_size: u64,
}

impl AuditLogger {
    pub fn log(&self, event: AuditEvent) -> Result<()> {
        let entry = serde_json::to_string(&event)?;
        let timestamp = Utc::now().to_rfc3339();
        writeln!(self.log_file, "[{}] {}", timestamp, entry)?;
        Ok(())
    }
}
```

---

# 7. 性能设计

## 7.1 性能指标

| 指标类别 | 指标项 | 目标值 |
|---------|-------|--------|
| 消息传输 | 消息延迟 | ≤300ms |
| 消息传输 | 消息吞吐 | ≥1000 msg/s |
| 文件传输 | 传输速度 | ≥10MB/s（100Mbps网络） |
| 文件传输 | 并发传输 | ≥3个文件 |
| 节点发现 | 发现时间 | ≤3秒 |
| 内存占用 | 运行内存 | ≤200MB |
| 启动时间 | 冷启动 | ≤2秒 |

## 7.2 性能优化策略

### 7.2.1 网络优化

```rust
// 连接池管理
pub struct ConnectionPool {
    connections: Vec<PooledConnection>,
    max_size: usize,
    idle_timeout: Duration,
}

// 消息批量发送
pub struct MessageBatcher {
    buffer: Vec<Message>,
    max_batch_size: usize,
    max_batch_delay: Duration,
}

impl MessageBatcher {
    pub async fn flush(&mut self) -> Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }
        // 批量发送消息
        for batch in self.buffer.chunks(self.max_batch_size) {
            self.send_batch(batch).await?;
        }
        self.buffer.clear();
        Ok(())
    }
}
```

### 7.2.2 存储优化

```rust
// 消息缓存
pub struct MessageCache {
    cache: LruCache<String, Message>,
    max_size: usize,
    persist_interval: Duration,
}

// 异步写入
pub struct AsyncWriter {
    queue: Vec<Message>,
    batch_size: usize,
    flush_interval: Duration,
}

impl AsyncWriter {
    pub async fn start(&mut self) {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(self.flush_interval).await;
                self.flush_to_disk().await;
            }
        });
    }
}
```

### 7.2.3 内存优化

- 文件分片大小：4KB-8KB自适应
- 消息缓冲区：动态扩容
- 连接池：复用TCP连接
- 图片压缩：自动缩放和压缩

---

# 8. 部署方案

## 8.1 系统要求

### 8.1.1 硬件要求

| 配置项 | 最低要求 | 推荐配置 |
|-------|---------|---------|
| CPU | 双核 1.5GHz | 四核 2.0GHz+ |
| 内存 | 2GB | 4GB+ |
| 存储 | 100MB | 500MB+ |
| 网络 | 100Mbps | 1Gbps |

### 8.1.2 软件要求

| 平台 | 支持版本 |
|------|---------|
| Windows | Windows 10/11 |
| macOS | macOS 11+ |
| Linux | Ubuntu 20.04+, Debian 11+ |

## 8.2 部署模式

### 8.2.1 单机部署

```
┌─────────────────────────────────┐
│         用户电脑                 │
│  ┌───────────────────────────┐  │
│  │     NeoLan 客户端         │  │
│  │  ┌─────┐ ┌─────┐ ┌─────┐  │  │
│  │  │ UI  │ │ 网络 │ │ 存储 │  │  │
│  │  └─────┘ └─────┘ └─────┘  │  │
│  └───────────────────────────┘  │
│                                 │
│  无需服务器 · 即装即用           │
└─────────────────────────────────┘
```

### 8.2.2 局域网部署

```
                    ┌─────────────────────────────────────┐
                    │          局域网 (LAN)               │
                    │                                     │
                    │  ┌─────────┐  ┌─────────┐           │
                    │  │ 电脑 A  │  │ 电脑 B  │           │
                    │  │NeoLan   │  │NeoLan   │           │
                    │  └────┬────┘  └────┬────┘           │
                    │       │            │                │
                    │       └─────┬──────┘                │
                    │             │                       │
                    │  ┌──────────▼──────────┐            │
                    │  │   交换机/路由器     │            │
                    │  └─────────────────────┘            │
                    │                                     │
                    │  无服务器 · P2P直连                  │
                    └─────────────────────────────────────┘
```

## 8.3 配置管理

### 8.3.1 配置文件结构

```toml
# neolan.conf

[network]
# UDP通信端口
udp_port = 2425
# TCP端口范围
tcp_port_range = [8000, 9000]
# 心跳间隔（秒）
heartbeat_interval = 30
# 心跳超时（秒）
heartbeat_timeout = 60

[security]
# 启用加密
enable_encryption = true
# 自动接受密钥协商
auto_key_exchange = true
# 密钥轮换周期（小时）
key_rotation_interval = 24

[transfer]
# 分片大小（字节）
chunk_size = 4096
# 并发传输数
max_concurrent_transfers = 3
# 传输限速（字节/秒，0=不限速）
max_transfer_speed = 0
# 临时文件目录
temp_dir = "%TEMP%/neolan"

[ui]
# 主题
theme = "light"
# 语言
language = "zh-CN"
# 开机自启动
auto_start = true
# 最小化到托盘
minimize_to_tray = true

[storage]
# 数据库路径
db_path = "%APPDATA%/neolan/neolan.db"
# 消息保留天数
message_retention_days = 365
# 日志级别
log_level = "info"
```

---

# 9. 附录

## 9.1 术语表

| 术语 | 说明 |
|------|------|
| P2P | Peer-to-Peer，对等网络架构 |
| IPMsg | IP Messenger，飞鸽传书协议 |
| UDP | User Datagram Protocol，用户数据报协议 |
| TCP | Transmission Control Protocol，传输控制协议 |
| AES | Advanced Encryption Standard，高级加密标准 |
| GCM | Galois/Counter Mode，一种认证加密模式 |
| ECDH | Elliptic Curve Diffie-Hellman，椭圆曲线密钥交换 |
| MD5 | Message Digest Algorithm 5，消息摘要算法 |

## 9.2 参考资料

1. 飞鸽传书(IPMsg)官方协议文档
2. AES-256-GCM加密标准（NIST SP 800-38D）
3. RFC 3550: RTP: A Transport Protocol for Real-time Applications
4. RFC 7714: AES-GCM Cipher for RTP/RTCP

## 9.3 版本历史

| 版本号 | 日期 | 修订说明 |
|-------|------|---------|
| V1.0 | 2026-01-05 | 初始版本，完整设计文档 |

---

**文档结束**
