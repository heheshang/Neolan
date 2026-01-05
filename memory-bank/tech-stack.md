# NeoLan 技术栈文档

> **本文档定义 NeoLan 项目的完整技术栈选型、依赖管理和模块化开发规范**

## 文档版本

| 版本号 | 日期 | 修订说明 |
|--------|------|---------|
| V1.0 | 2026-01-05 | 初始版本 |

---

# 0. 【强制规则】代码生成前置条件

## ⚠️ Always - 必须始终应用

```markdown
# 写任何代码前必须完整阅读：

1. memory-bank/@architecture.md
   - 包含完整数据库结构
   - 包含模块组织架构
   - 包含文件拆分规则

2. memory-bank/@neolan-design-document.md
   - 包含完整功能设计
   - 包含通信协议定义
   - 包含数据模型设计

# 每完成一个重大功能或里程碑后，必须更新：
1. memory-bank/@architecture.md
   - 数据库表结构变更
   - 新增模块/组件
   - 架构调整记录
```

## 🚫 禁止事项 - Monolithic Files

```markdown
# 严格禁止单体巨文件（Monolithic Files）

❌ 禁止：
- 单个 Rust 文件超过 500 行
- 单个 Vue 组件超过 300 行
- 单个 TypeScript 文件超过 300 行
- 一个文件包含多个不相关的模块/结构体

✅ 要求：
- 按功能拆分为多个文件
- 每个文件只包含一个核心模块/组件
- 使用 mod.rs 进行模块导出和组织
- 遵循单一职责原则
```

---

# 1. 技术栈总览

## 1.1 技术分层

```
┌─────────────────────────────────────────────────────────────┐
│  Frontend Layer (用户界面)                                   │
│  Vue 3 + TypeScript + Vite + Tailwind CSS                  │
└─────────────────────────────────────────────────────────────┘
                            ▲
                      Tauri IPC Bridge
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Backend Layer (业务逻辑)                                    │
│  Rust + Tokio + Sea-ORM + Serde                            │
└─────────────────────────────────────────────────────────────┘
                            ▲
                      Network Layer
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Transport Layer (网络传输)                                  │
│  UDP (控制消息) + TCP (文件数据)                            │
└─────────────────────────────────────────────────────────────┘
                            ▲
                      Storage Layer
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Storage Layer (数据持久化)                                  │
│  SQLite + Local File System                                │
└─────────────────────────────────────────────────────────────┘
```

## 1.2 技术选型决策表

| 决策点 | 选型方案 | 替代方案 | 选择理由 |
|--------|---------|---------|---------|
| 桌面框架 | **Tauri 2.x** | Electron | 更小的包体积、Rust 后端安全性、更低资源占用 |
| 前端框架 | **Vue 3** | React | 更简洁的 API、Composition API 类型友好 |
| 语言 | **TypeScript** | JavaScript | 类型安全、更好的 IDE 支持 |
| 构建工具 | **Vite** | Webpack | 更快的 HMR、原生 ESM 支持 |
| 样式方案 | **Tailwind CSS** | CSS Modules | 原子化 CSS、快速开发、样式一致性 |
| 后端语言 | **Rust** | Go/C++ | 内存安全、高性能、跨平台 |
| 异步运行时 | **Tokio** | async-std | 生态成熟、性能优异 |
| 数据库 | **SQLite** | PostgreSQL/MySQL | 无服务器、轻量级、嵌入式 |
| ORM | **Sea-ORM** | Diesel | 异步优先、类似 TypeScript ORM |
| 序列化 | **Serde** | - | Rust 事实标准、类型安全 |
| 加密 | **AES-256-GCM** | - | 认证加密、NIST 标准 |
| 密钥交换 | **ECDH (P-256)** | - | 椭圆曲线、前向安全 |

---

# 2. Frontend 技术栈

## 2.1 核心依赖

### package.json

```json
{
  "name": "neolan",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc --noEmit && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  },
  "dependencies": {
    "vue": "^3.5.13",
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-opener": "^2"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.2.1",
    "typescript": "~5.6.2",
    "vite": "^6.0.3",
    "vue-tsc": "^2.1.10",
    "@tauri-apps/cli": "^2"
  }
}
```

## 2.2 依赖详解

| 依赖名 | 版本 | 用途 | 是否必需 |
|--------|------|------|---------|
| **vue** | ^3.5.13 | 前端框架 | ✅ 必需 |
| **@tauri-apps/api** | ^2 | Tauri 前端 API | ✅ 必需 |
| **@tauri-apps/plugin-opener** | ^2 | 打开外部链接 | ✅ 必需 |

### 开发依赖

| 依赖名 | 版本 | 用途 |
|--------|------|------|
| **@vitejs/plugin-vue** | ^5.2.1 | Vue 3 SFC 支持 |
| **typescript** | ~5.6.2 | TypeScript 编译器 |
| **vite** | ^6.0.3 | 构建工具 |
| **vue-tsc** | ^2.1.10 | Vue 类型检查 |
| **@tauri-apps/cli** | ^2 | Tauri CLI |

## 2.3 计划添加的依赖

```json
{
  "dependencies": {
    // 状态管理
    "pinia": "^2.2.0",

    // 路由
    "vue-router": "^4.4.0",

    // UI 组件库（可选）
    "@headlessui/vue": "^1.7.0",
    "@heroicons/vue": "^2.1.0",

    // 样式
    "tailwindcss": "^3.4.0",
    "autoprefixer": "^10.4.0",
    "postcss": "^8.4.0",

    // 工具库
    "date-fns": "^3.0.0",
    "lodash-es": "^4.17.21"
  },
  "devDependencies": {
    // TypeScript 类型
    "@types/lodash-es": "^4.17.0",

    // 代码质量
    "eslint": "^9.0.0",
    "eslint-plugin-vue": "^9.0.0",
    "@typescript-eslint/eslint-plugin": "^7.0.0",
    "@typescript-eslint/parser": "^7.0.0",
    "prettier": "^3.2.0"
  }
}
```

---

# 3. Backend 技术栈

## 3.1 核心依赖

### Cargo.toml

```toml
[package]
name = "neolan"
version = "0.1.0"
edition = "2021"

[lib]
name = "neolan_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
# Tauri 核心依赖
tauri = { version = "2", features = ["devtools"] }
tauri-plugin-opener = "2"

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# 异步运行时（计划添加）
# tokio = { version = "1", features = ["full"] }
# tokio-util = { version = "0.7", features = ["codec"] }

# 网络库（计划添加）
# socket2 = "0.5"

# 数据库（计划添加）
# sea-orm = { version = "1", features = ["sqlx-sqlite", "runtime-tokio-rustls"] }
# sea-orm-migration = "1"

# 加密（计划添加）
# aes-gcm = "0.10"
# rand = "0.8"
# x25519-dalek = "2"

# 日志（计划添加）
# tracing = "0.1"
# tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 工具库（计划添加）
# thiserror = "1"
# anyhow = "1"
# uuid = { version = "1", features = ["v4", "serde"] }
# chrono = { version = "0.4", features = ["serde"] }
```

## 3.2 依赖详解

### 当前依赖

| 依赖名 | 用途 |
|--------|------|
| **tauri** | Tauri 框架核心 |
| **tauri-plugin-opener** | 打开外部链接插件 |
| **serde** | 序列化/反序列化框架 |
| **serde_json** | JSON 支持 |

### 计划添加的依赖

| 依赖名 | 版本 | 用途 | 何时添加 |
|--------|------|------|---------|
| **tokio** | 1.x | 异步运行时 | 网络功能开发前 |
| **tokio-util** | 0.7 | 编解码工具 | 网络功能开发前 |
| **socket2** | 0.5 | Socket 底层操作 | 网络功能开发前 |
| **sea-orm** | 1.x | ORM 框架 | 存储功能开发前 |
| **sea-orm-migration** | 1.x | 数据库迁移 | 存储功能开发前 |
| **aes-gcm** | 0.10 | AES-256-GCM 加密 | 加密功能开发前 |
| **rand** | 0.8 | 安全随机数 | 加密功能开发前 |
| **x25519-dalek** | 2.x | ECDH 密钥交换 | 加密功能开发前 |
| **tracing** | 0.1 | 结构化日志 | 项目启动时 |
| **tracing-subscriber** | 0.3 | 日志订阅器 | 项目启动时 |
| **thiserror** | 1.x | 错误处理derive | 项目启动时 |
| **anyhow** | 1.x | 错误处理 | 项目启动时 |
| **uuid** | 1.x | UUID 生成 | 项目启动时 |
| **chrono** | 0.4 | 时间处理 | 项目启动时 |

---

# 4. 模块化开发规范

## 4.1 Rust 模块化规则

### 文件拆分原则

```rust
// ✅ 正确：按功能拆分
src-tauri/src/
├── modules/
│   ├── peer/
│   │   ├── mod.rs         // 模块导出
│   │   ├── manager.rs     // PeerManager (100-200 行)
│   │   ├── discovery.rs   // 节点发现逻辑 (50-100 行)
│   │   ├── state.rs       // 状态管理 (50-100 行)
│   │   └── types.rs       // 数据结构定义 (50-100 行)
│   └── message/
│       ├── mod.rs
│       ├── handler.rs     // MessageHandler
│       ├── router.rs      // MessageRouter
│       ├── crypto.rs      // 加密/解密
│       └── types.rs

// ❌ 错误：单体文件
src-tauri/src/
├── peer.rs        // 1000+ 行，包含所有节点相关代码
├── message.rs     // 800+ 行，包含所有消息相关代码
└── transfer.rs    // 1200+ 行，包含所有传输相关代码
```

### 模块大小限制

| 文件类型 | 推荐行数 | 最大行数 |
|---------|---------|---------|
| Rust 模块文件 | 100-200 | 500 |
| Rust 类型定义 | 50-100 | 200 |
| Vue 组件 | 100-200 | 300 |
| TypeScript 文件 | 100-200 | 300 |

### 模块职责示例

```rust
// ✅ 单一职责：每个文件只做一件事
// src-tauri/src/modules/peer/manager.rs
// 职责：节点的增删改查、状态管理

pub struct PeerManager {
    peers: HashMap<IpAddr, PeerNode>,
    config: PeerConfig,
}

impl PeerManager {
    pub fn new(config: PeerConfig) -> Self { /* ... */ }
    pub async fn add_peer(&mut self, peer: PeerNode) -> Result<()> { /* ... */ }
    pub async fn remove_peer(&mut self, ip: IpAddr) -> Result<()> { /* ... */ }
    pub fn get_peer(&self, ip: IpAddr) -> Option<&PeerNode> { /* ... */ }
    pub fn list_peers(&self) -> Vec<&PeerNode> { /* ... */ }
}

// src-tauri/src/modules/peer/discovery.rs
// 职责：UDP 广播发现、节点探测

pub struct PeerDiscovery {
    socket: UdpSocket,
}

impl PeerDiscovery {
    pub async fn broadcast(&self) -> Result<()> { /* ... */ }
    pub async fn scan(&self) -> Vec<PeerNode> { /* ... */ }
}

// src-tauri/src/modules/peer/state.rs
// 职责：节点状态机、心跳检测

pub struct PeerStateMachine {
    states: HashMap<IpAddr, PeerState>,
}

impl PeerStateMachine {
    pub fn transition(&mut self, ip: IpAddr, new_state: PeerState) { /* ... */ }
}
```

## 4.2 Vue 组件模块化规则

### 组件拆分原则

```vue
<!-- ✅ 正确：组件拆分 -->
<!-- src/components/PeerList.vue (150 行) -->
<script setup lang="ts">
// 只负责节点列表展示
const props = defineProps<{ peers: Peer[] }>();
const emit = defineEmits<{ select: [peer: Peer] }>();
</script>

<!-- src/components/PeerListItem.vue (80 行) -->
<script setup lang="ts">
// 只负责单个节点项展示
const props = defineProps<{ peer: Peer }>();
</script>

<!-- ❌ 错误：巨文件组件 -->
<!-- src/components/PeerList.vue (500+ 行) -->
<script setup lang="ts">
// 节点列表、节点详情、聊天窗口、消息发送... 全堆在一起
</script>
```

### 组件职责示例

```
src/components/
├── PeerList.vue          # 节点列表容器 (150 行)
├── PeerListItem.vue      # 单个节点项 (80 行)
├── ChatWindow.vue        # 聊天窗口容器 (200 行)
│   ├── MessageList.vue   # 消息列表 (120 行)
│   ├── MessageItem.vue   # 单条消息 (60 行)
│   ├── MessageInput.vue  # 输入框 (100 行)
│   └── FileTransfer.vue  # 文件传输 (180 行)
└── Settings/
    ├── NetworkSettings.vue   # 网络设置 (150 行)
    ├── SecuritySettings.vue  # 安全设置 (120 行)
    └── UISettings.vue        # UI 设置 (100 行)
```

---

# 5. 开发工具链

## 5.1 代码编辑器

**推荐**: Visual Studio Code

### 必需扩展

| 扩展名 | 用途 |
|--------|------|
| **Vue - Official** | Vue 3 语法支持 |
| **Tauri** | Tauri 框架支持 |
| **rust-analyzer** | Rust 语言支持 |
| **TypeScript Vue Plugin** | Vue TypeScript 支持 |

### 推荐扩展

| 扩展名 | 用途 |
|--------|------|
| **ESLint** | JavaScript/TypeScript 代码检查 |
| **Prettier** | 代码格式化 |
| **Error Lens** | 内联错误显示 |
| **GitLens** | Git 增强 |

## 5.2 命令行工具

```bash
# Rust 工具链
rustup           # Rust 工具链管理器
cargo            # Rust 包管理器和构建工具
rustfmt          # Rust 代码格式化
cargo clippy     # Rust 代码检查

# Node.js 工具链
npm / pnpm       # 包管理器
vite             # 前端构建工具
vue-tsc          # Vue TypeScript 类型检查

# Tauri 工具
tauri            # Tauri CLI
```

## 5.3 开发服务器

| 服务 | 端口 | 说明 |
|------|------|------|
| Vite Dev Server | 1420 | 前端开发服务器（固定端口） |
| Vite HMR | 1421 | 热模块替换 |
| UDP (控制消息) | 2425 | IPMsg 协议默认端口 |
| TCP (文件数据) | 8000-9000 | 动态分配范围 |

---

# 6. 构建与部署

## 6.1 开发模式

```bash
# 启动开发模式（Vite + Tauri 热重载）
npm run tauri dev

# 等价于：
# 1. 启动 Vite 开发服务器（端口 1420）
# 2. 编译 Rust 后端（debug 模式）
# 3. 启动 Tauri 应用窗口
```

## 6.2 生产构建

```bash
# 构建生产版本
npm run tauri build

# 输出位置：
# - Windows: src-tauri/target/release/bundle/nsis/
# - macOS: src-tauri/target/release/bundle/dmg/
# - Linux: src-tauri/target/release/bundle/deb/ 或 appimage/
```

## 6.3 构建优化

### Rust 优化

```toml
# Cargo.toml

[profile.release]
# 启用链接时优化
lto = true
# 代码量优化
codegen-units = 1
# 优化等级
opt-level = "z"  # 或 "s" 用于更小体积
# 去除调试信息
strip = true
```

### Frontend 优化

```javascript
// vite.config.ts
export default defineConfig({
  build: {
    // 代码分割
    rollupOptions: {
      output: {
        manualChunks: {
          'vue-vendor': ['vue', 'vue-router'],
          'tauri-vendor': ['@tauri-apps/api'],
        },
      },
    },
    // 压缩
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,  // 移除 console
      },
    },
  },
});
```

---

# 7. 版本管理

## 7.1 版本号规范

遵循语义化版本 (Semantic Versioning): `MAJOR.MINOR.PATCH`

- **MAJOR**: 不兼容的 API 变更
- **MINOR**: 向后兼容的功能新增
- **PATCH**: 向后兼容的问题修复

## 7.2 依赖版本策略

| 依赖类型 | 版本策略 | 说明 |
|---------|---------|------|
| Tauri | `^2.0.0` | 主版本锁定，接受次版本更新 |
| Vue | `^3.5.13` | 主版本锁定 |
| Rust 依赖 | 具体版本或 `~1.0.0` | 更保守，避免破坏性更新 |

---

# 8. 性能优化

## 8.1 前端优化

| 技术 | 说明 | 预期收益 |
|------|------|---------|
| 虚拟滚动 | 大列表渲染 | 内存占用 -60% |
| 懒加载 | 路由级别代码分割 | 首屏加载 -40% |
| 缓存策略 | IndexedDB 缓存消息 | 二次加载 -80% |

## 8.2 后端优化

| 技术 | 说明 | 预期收益 |
|------|------|---------|
| 连接池 | 复用 TCP 连接 | 连接建立 -90% |
| 消息批量 | 批量发送消息 | 网络往返 -70% |
| 异步 I/O | Tokio 异步运行时 | 吞吐量 +300% |

---

# 9. 安全性

## 9.1 加密技术栈

| 场景 | 算法 | 密钥长度 | 库 |
|------|------|---------|-----|
| 消息加密 | AES-256-GCM | 256 位 | aes-gcm |
| 密钥交换 | ECDH (P-256) | 256 位 | x25519-dalek |
| 消息认证 | Poly1305 (GCM 内置) | 128 位 | aes-gcm |
| 随机数 | ChaCha20 | - | rand |

## 9.2 安全依赖

```toml
[dependencies]
# 加密
aes-gcm = "0.10"          # AES-256-GCM 认证加密
rand = "0.8"              # 安全随机数
x25519-dalek = "2"        # ECDH 密钥交换

# 密码学哈希
sha2 = "0.10"             # SHA-256
```

---

# 10. 更新日志

| 日期 | 版本 | 变更内容 |
|------|------|---------|
| 2026-01-05 | V1.0 | 初始版本，定义技术栈选型和模块化规范 |

---

**文档结束**
