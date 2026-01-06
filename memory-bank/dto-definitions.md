# NeoLan DTO 定义和转换逻辑

## 文档版本

| 版本号 | 日期 | 修订说明 |
|--------|------|---------|
| V1.0 | 2026-01-05 | 初始版本 |

---

# 概述

DTO (Data Transfer Object) 用于 Tauri 前后端之间的数据传输。与数据库实体 Model 不同，DTO 需要实现 `Serialize` 以便通过 IPC 传递。

---

# 1. PeerDto

## 用途
节点信息传输对象，用于前端显示节点列表

## Rust 定义

```rust
// src-tauri/src/dto/peer.rs
use serde::{Deserialize, Serialize};
use crate::storage::entities::peers::Model as PeerModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerDto {
    pub id: i32,
    pub ip: String,
    pub port: i32,
    pub username: Option<String>,
    pub hostname: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub groups: Vec<String>,          // 转换：JSON字符串 -> Vec<String>
    pub last_seen: i64,                // 转换：DateTime -> 毫秒时间戳
    pub created_at: i64,               // 转换：DateTime -> 毫秒时间戳
    pub updated_at: Option<i64>,       // 转换：Option<DateTime> -> Option<i64>
}
```

## 转换实现

```rust
impl From<PeerModel> for PeerDto {
    fn from(model: PeerModel) -> Self {
        Self {
            id: model.id,
            ip: model.ip,
            port: model.port,
            username: model.username,
            hostname: model.hostname,
            nickname: model.nickname,
            avatar: model.avatar,
            groups: parse_groups_json(model.groups),  // JSON 解析
            last_seen: model.last_seen.timestamp_millis(),
            created_at: model.created_at.timestamp_millis(),
            updated_at: model.updated_at.map(|dt| dt.timestamp_millis()),
        }
    }
}

fn parse_groups_json(json: Option<String>) -> Vec<String> {
    json.as_ref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default()
}
```

## TypeScript 类型

```typescript
// src/types/peer.ts
export interface PeerDto {
  id: number;
  ip: string;
  port: number;
  username: string | null;
  hostname: string | null;
  nickname: string | null;
  avatar: string | null;
  groups: string[];
  last_seen: number;  // 毫秒时间戳
  created_at: number;
  updated_at: number | null;
}
```

---

# 2. MessageDto

## 用途
消息传输对象，用于前端显示聊天记录

## Rust 定义

```rust
// src-tauri/src/dto/message.rs
use serde::{Deserialize, Serialize};
use crate::storage::entities::messages::Model as MessageModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageDto {
    pub id: i32,
    pub msg_id: String,
    pub sender_ip: String,
    pub sender_name: String,
    pub receiver_ip: String,
    pub msg_type: i32,
    pub content: String,
    pub is_encrypted: bool,
    pub is_offline: bool,
    pub sent_at: i64,
    pub received_at: Option<i64>,
    pub created_at: i64,
}
```

## 转换实现

```rust
impl From<MessageModel> for MessageDto {
    fn from(model: MessageModel) -> Self {
        Self {
            id: model.id,
            msg_id: model.msg_id,
            sender_ip: model.sender_ip,
            sender_name: model.sender_name,
            receiver_ip: model.receiver_ip,
            msg_type: model.msg_type,
            content: model.content,
            is_encrypted: model.is_encrypted,
            is_offline: model.is_offline,
            sent_at: model.sent_at.timestamp_millis(),
            received_at: model.received_at.map(|dt| dt.timestamp_millis()),
            created_at: model.created_at.timestamp_millis(),
        }
    }
}
```

## TypeScript 类型

```typescript
// src/types/message.ts
export interface MessageDto {
  id: number;
  msg_id: string;
  sender_ip: string;
  sender_name: string;
  receiver_ip: string;
  msg_type: number;
  content: string;
  is_encrypted: boolean;
  is_offline: boolean;
  sent_at: number;
  received_at: number | null;
  created_at: number;
}
```

---

# 3. TransferDto

## 用途
文件传输状态传输对象

## Rust 定义

```rust
// src-tauri/src/dto/transfer.rs
use serde::{Deserialize, Serialize};
use crate::storage::entities::transfers::Model as TransferModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferDto {
    pub id: i32,
    pub task_id: String,
    pub direction: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_md5: String,
    pub peer_ip: String,
    pub peer_name: String,
    pub status: String,
    pub transferred_size: i64,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub created_at: i64,
}
```

## 转换实现

```rust
impl From<TransferModel> for TransferDto {
    fn from(model: TransferModel) -> Self {
        Self {
            id: model.id,
            task_id: model.task_id,
            direction: model.direction,
            file_name: model.file_name,
            file_size: model.file_size,
            file_md5: model.file_md5,
            peer_ip: model.peer_ip,
            peer_name: model.peer_name,
            status: model.status,
            transferred_size: model.transferred_size,
            started_at: model.started_at.map(|dt| dt.timestamp_millis()),
            completed_at: model.completed_at.map(|dt| dt.timestamp_millis()),
            created_at: model.created_at.timestamp_millis(),
        }
    }
}
```

## TypeScript 类型

```typescript
// src/types/transfer.ts
export interface TransferDto {
  id: number;
  task_id: string;
  direction: 'upload' | 'download';
  file_name: string;
  file_size: number;
  file_md5: string;
  peer_ip: string;
  peer_name: string;
  status: 'pending' | 'transferring' | 'paused' | 'completed' | 'failed' | 'cancelled';
  transferred_size: number;
  started_at: number | null;
  completed_at: number | null;
  created_at: number;
}
```

---

# 4. ProtocolDto

## 用途
IPMsg 协议消息传输对象，用于网络层和消息层之间的数据传递

## Rust 定义

```rust
// src-tauri/src/network/protocol.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolMessage {
    /// 协议版本 (NeoLan 使用 1)
    pub version: u8,

    /// 数据包 ID (单调递增)
    pub packet_id: u64,

    /// 发送方用户名 (显示名称)
    pub sender_name: String,

    /// 发送方主机名
    pub sender_host: String,

    /// 消息类型 (见 msg_type 常量)
    pub msg_type: u32,

    /// 消息内容 (格式取决于 msg_type)
    pub content: String,
}
```

## 协议常量

```rust
pub mod msg_type {
    /// 协议版本
    pub const IPMSG_VERSION: u16 = 0x0001;        // 协议版本
    pub const IPMSG_DEFAULT_PORT: u16 = 0x0979;  // 2425

    // 消息类型 (mode - 低 8 位)
    pub const IPMSG_NOOPERATION: u32    = 0x00000000; // 0 无操作
    pub const IPMSG_BR_ENTRY: u32       = 0x00000001; // 1 广播上线
    pub const IPMSG_BR_EXIT: u32        = 0x00000002; // 2 广播下线
    pub const IPMSG_ANSENTRY: u32       = 0x00000003; // 3 对 BR_ENTRY 的应答
    pub const IPMSG_BR_ABSENCE: u32     = 0x00000004; // 4 广播缺席
    pub const IPMSG_BR_ISGETLIST: u32   = 0x00000010; // 16 请求列表
    pub const IPMSG_OKGETLIST: u32      = 0x00000011; // 17 同意发送列表
    pub const IPMSG_GETLIST: u32        = 0x00000012; // 18 请求列表
    pub const IPMSG_ANSLIST: u32        = 0x00000013; // 19 返回列表
    pub const IPMSG_SENDMSG: u32        = 0x00000020; // 32 发送消息
    pub const IPMSG_RECVMSG: u32        = 0x00000021; // 33 接收确认
    pub const IPMSG_READMSG: u32        = 0x00000030; // 48 消息已读
    pub const IPMSG_DELMSG: u32         = 0x00000031; // 49 删除消息
    pub const IPMSG_GETFILEDATA: u32    = 0x00000060; // 96 请求文件数据
    pub const IPMSG_RELEASEFILES: u32   = 0x00000061; // 97 释放文件资源

    // 选项标志 (options - 高 24 位)
    pub const IPMSG_FILEATTACHOPT: u32  = 0x00200000; // 2097152 文件附加
    pub const IPMSG_ENCRYPTOPT: u32     = 0x00400000; // 4194304 加密
    pub const IPMSG_UTF8OPT: u32        = 0x00800000; // 8388608 UTF-8 编码
    pub const IPMSG_SENDCHECKOPT: u32   = 0x00000100; // 256 发送确认
    pub const IPMSG_BROADCASTOPT: u32   = 0x00000400; // 1024 广播发送
}
```

## 辅助函数

```rust
// 从 command 中提取 mode (低 8 位)
pub const fn get_mode(command: u32) -> u8 {
    (command & 0x000000ff) as u8
}

// 从 command 中提取 opts (高 24 位)
pub const fn get_opt(command: u32) -> u32 {
    command & 0xffffff00
}

// 检查是否包含某个选项
pub const fn has_opt(command: u32, flag: u32) -> bool {
    (get_opt(command) & flag) != 0
}
```

## TypeScript 类型

```typescript
// src/types/protocol.ts
export interface ProtocolMessage {
  version: number;
  packet_id: number;
  sender_name: string;
  sender_host: string;
  msg_type: number;
  content: string;
}

export namespace MsgType {
  // 消息类型常量
  export const NOOPERATION = 0x00000000;
  export const BR_ENTRY = 0x00000001;
  export const BR_EXIT = 0x00000002;
  export const ANSENTRY = 0x00000003;
  export const BR_ABSENCE = 0x00000004;
  export const SENDMSG = 0x00000020;
  export const RECVMSG = 0x00000021;
  export const GETFILEDATA = 0x00000060;

  // 选项标志
  export const FILEATTACHOPT = 0x00200000;
  export const ENCRYPTOPT = 0x00400000;
  export const UTF8OPT = 0x00800000;
}
```

## 协议格式

```
标准格式: version:packet_id:sender_name:sender_host:msg_type:content[:ext_fields]
示例: 1:123:Alice:alice-pc:32:Hello World
```

---

# 5. FileSendRequest

## 用途
文件传输请求数据结构 (JSON 格式，用于 ProtocolMessage.content)

## Rust 定义

```rust
// src-tauri/src/network/protocol.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileSendRequest {
    /// 文件名
    pub name: String,

    /// 文件大小 (字节)
    pub size: u64,

    /// MD5 哈希 (十六进制字符串)
    pub md5: String,
}
```

## TypeScript 类型

```typescript
// src/types/file.ts
export interface FileSendRequest {
  name: string;
  size: number;
  md5: string;
}
```

---

# 6. FileSendResponse

## 用途
文件传输响应数据结构 (JSON 格式，用于 ProtocolMessage.content)

## Rust 定义

```rust
// src-tauri/src/network/protocol.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileSendResponse {
    /// true = 接受, false = 拒绝
    pub accept: bool,

    /// TCP 数据传输端口 (仅当 accept = true 时)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}
```

## TypeScript 类型

```typescript
// src/types/file.ts
export interface FileSendResponse {
  accept: boolean;
  port?: number;
}
```

---

# 7. AppConfigDto

## 用途
应用配置传输对象

## Rust 定义

```rust
// src-tauri/src/dto/config.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfigDto {
    // 用户信息
    pub username: String,
    pub hostname: String,

    // 网络配置
    pub udp_port: u16,
    pub tcp_port_range: (u16, u16),

    // 心跳配置
    pub heartbeat_interval: u64,  // 秒
    pub heartbeat_timeout: u64,   // 秒

    // 安全配置
    pub enable_encryption: bool,
}
```

## TypeScript 类型

```typescript
// src/types/config.ts
export interface AppConfigDto {
  username: string;
  hostname: string;
  udp_port: number;
  tcp_port_range: [number, number];
  heartbeat_interval: number;
  heartbeat_timeout: number;
  enable_encryption: boolean;
}
```

---

# 8. Tauri 事件 Payload 定义

## 节点状态变化事件

### peer://online

```typescript
// 前端监听
import { listen } from '@tauri-apps/api/event';

listen<PeerDto>('peer://online', (event) => {
    const newPeer = event.payload;
    console.log('Peer online:', newPeer.username);
});
```

### peer://offline

```typescript
listen<{ ip: string }>('peer://offline', (event) => {
    const { ip } = event.payload;
    console.log('Peer offline:', ip);
});
```

### peer://update

```typescript
listen<PeerDto>('peer://update', (event) => {
    const updatedPeer = event.payload;
    console.log('Peer updated:', updatedPeer.username);
});
```

## 消息事件

### message://received

```typescript
listen<MessageDto>('message://received', (event) => {
    const message = event.payload;
    console.log('New message:', message.content);
});
```

## 文件传输事件

### file://request

**用途**：接收方收到文件传输请求时触发

**Payload 格式**：

```typescript
// src/types/file.ts
export interface FileTransferRequest {
  request_id: string;      // 请求 ID（UUID）
  from_ip: string;          // 发送方 IP
  from_name: string;        // 发送方用户名
  file_name: string;        // 文件名
  file_size: number;        // 文件大小（字节）
  file_md5: string;         // 文件 MD5
}

// 前端监听
listen<FileTransferRequest>('file://request', async (event) => {
    const request = event.payload;

    // 显示确认对话框
    const accept = confirm(
        `${request.from_name} 想要发送文件 "${request.file_name}" (${formatSize(request.file_size)})\n\n` +
        `MD5: ${request.file_md5}\n\n` +
        `是否接受？`
    );

    if (accept) {
        // 响应接受
        await invoke('accept_file_transfer', {
            requestId: request.request_id,
            accept: true
        });
    } else {
        // 响应拒绝
        await invoke('accept_file_transfer', {
            requestId: request.request_id,
            accept: false
        });
    }
});
```

**Tauri 命令定义**：

```rust
// src-tauri/src/commands/file_transfer.rs

#[tauri::command]
async fn accept_file_transfer(
    request_id: String,
    accept: bool,
    app: tauri::AppHandle,
) -> Result<(), String> {
    if accept {
        // 分配 TCP 端口并发送响应
        let tcp_port = allocate_tcp_port().await?;
        send_file_response(&request_id, true, tcp_port).await?;
    } else {
        // 发送拒绝响应
        send_file_response(&request_id, false, 0).await?;
    }
    Ok(())
}

fn allocate_tcp_port() -> Result<u16, String> {
    // 在 8000-9000 范围内分配可用端口
    for port in 8000..9000 {
        if is_port_available(port) {
            return Ok(port);
        }
    }
    Err("No available port".to_string())
}
```

### transfer://progress

**用途**：文件传输过程中定期更新进度

```typescript
listen<TransferProgressEvent>('transfer://progress', (event) => {
    const { task_id, progress, speed } = event.payload;
    console.log(`Transfer ${task_id}: ${progress}%`);
});

interface TransferProgressEvent {
    task_id: string;
    progress: number;  // 0-100
    speed: number;     // bytes/s
    transferred: number;
    total: number;
}
```

### transfer://completed

```typescript
listen<TransferDto>('transfer://completed', (event) => {
    const transfer = event.payload;
    console.log('Transfer completed:', transfer.file_name);
});
```

---

# 9. DTO 模块导出

## Rust 模块结构

```rust
// src-tauri/src/dto/mod.rs
pub mod peer;
pub mod message;
pub mod transfer;
pub mod config;

pub use peer::PeerDto;
pub use message::MessageDto;
pub use transfer::TransferDto;
pub use config::AppConfigDto;
```

## 在 lib.rs 中注册

```rust
// src-tauri/src/lib.rs
mod dto;

// 在 invoke_handler 中使用
#[tauri::command]
async fn get_peers() -> Result<Vec<PeerDto>, String> {
    // ...
    Ok(peers.into_iter().map(PeerDto::from).collect())
}
```

---

# 10. 转换规则总结

| Model 类型 | DTO 类型 | 转换规则 |
|-----------|---------|---------|
| `DateTime<Utc>` | `i64` | `timestamp_millis()` |
| `Option<DateTime<Utc>>` | `Option<i64>` | `map(\|dt\| dt.timestamp_millis())` |
| `Option<String>` (JSON) | `Vec<T>` | `serde_json::from_str()` |
| `bool` | `bool` | 直接映射 |
| `String` | `String` | 直接映射 |
| `i32/i64` | `number` | 直接映射 |

---

# 11. TypeScript 类型同步

为了保持前后端类型一致，建议：

1. **手动维护**：在 `src/types/` 目录手动维护 TypeScript 类型定义
2. **自动生成**：使用 `ts-rs` crate 从 Rust 生成 TypeScript 类型

```toml
# Cargo.toml
[dependencies]
ts-rs = { version = "7", features = ["chrono-impl"] }
```

```rust
// 使用 ts-rs 自动生成
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
pub struct PeerDto {
    // ...
}

// 生成: tauri src-tauri/src/dto/peer.rs --out-dir src/types/
```

---

**文档结束**
