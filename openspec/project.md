# Project Context

## Purpose

NeoLan is a LAN (Local Area Network) instant messaging and file transfer system built with a **decentralized P2P architecture**. The project aims to provide:

- Serverless peer-to-peer communication over local networks
- Real-time instant messaging with message delivery confirmation
- Reliable file transfer with resume capability and integrity checks
- Compatibility with IPMsg (飞鸽传书/飞秋 FeiQ) protocol
- Desktop application experience using Tauri framework

## Tech Stack

| Layer | Technology |
|-------|-----------|
| **Desktop Framework** | Tauri 2.x |
| **Frontend UI** | Vue 3 (Composition API + `<script setup>`) + TypeScript |
| **State Management** | Pinia (planned) |
| **Build Tool** | Vite (dev server on port 1420) |
| **Backend Language** | Rust (edition 2021) |
| **Async Runtime** | tokio (planned) |
| **Storage** | SQLite (planned) |
| **Network Protocol** | UDP (control) + TCP (data) |

### Primary Dependencies

**Frontend:**
- Vue 3 with Composition API
- TypeScript
- Tauri API (`@tauri-apps/api/core`)

**Backend (Rust):**
- `tokio` for async runtime
- `serde` for serialization/deserialization
- `tauri` for desktop framework
- Network sockets via standard library

## Project Conventions

### Code Style

**Rust Backend:**
- Rust 2021 edition
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- `#[tauri::command]` attribute for functions exposed to frontend
- `serde::Serialize`/`Deserialize` for IPC data structures

**Frontend (Vue 3 + TypeScript):**
- Composition API with `<script setup>` syntax
- TypeScript strict mode
- Import Tauri APIs from `@tauri-apps/api/core`

**Naming Conventions:**
- Rust: `snake_case` for functions and variables, `PascalCase` for types/structs
- TypeScript: `camelCase` for functions/variables, `PascalCase` for components/types

### Architecture Patterns

**Modular Rust Backend:**
Six core modules with clear separation of concerns:

| Module | Responsibility | Location |
|--------|---------------|-------------------|
| **Network** | UDP/TCP socket I/O, packet parsing/sending | `src-tauri/src/network/` |
| **Message** | Protocol handling, encryption/decryption, routing | `src-tauri/src/modules/message/` |
| **FileTransfer** | File chunking, resume, integrity checks | `src-tauri/src/modules/file_transfer/` |
| **Peer** | Node discovery, heartbeat, group management | `src-tauri/src/modules/peer/` |
| **Storage** | SQLite DB, config, message history | `src-tauri/src/storage/` |
| **Commands** | Tauri commands for frontend | `src-tauri/src/commands/` |

**Tauri IPC Pattern:**
```
Frontend (Vue)  →  Tauri IPC Bridge  →  Rust Backend
    invoke()    →   invoke_handler   →   module logic
```

**Communication Protocol:**
- Default UDP port: 2425 (control messages: online/offline/heartbeat)
- TCP ports: 8000-9000 range (file data)
- Encoding: UTF-8
- Protocol format: `version:packet_id:sender_name:hostname:msg_type:content[:ext_fields]`

### Testing Strategy

- Rust: `cargo test` for unit tests
- Frontend: planned integration testing
- Focus on network protocol compatibility with IPMsg/FeiQ

### Git Workflow

- Main branch: `main`
- Commit messages use conventional format (feat, fix, docs, refactor, etc.)
- Example: `feat(message): implement message confirmation mechanism`

## Domain Context

**P2P LAN Messaging:**
- No central server required - all nodes are equal
- Works in isolated LAN environments
- UDP for low-latency control messages (presence, heartbeat)
- TCP for reliable file transfer

**IPMsg/FeiQ Protocol Compatibility:**
- Existing protocol widely used in Chinese LAN environments
- Supports presence detection, messaging, and file transfer
- Extension fields for NeoLan-specific features

## Important Constraints

1. **Vite Dev Server Port:** Must run on port 1420 with `strictPort: true` - Tauri expects this fixed port
2. **Vite Watches:** Configured to ignore `src-tauri/` to prevent duplicate rebuilds
3. **Rust Crate Name:** Uses `neolan_lib` suffix to avoid Windows naming conflicts (neolan conflicts with neolan_lib)
4. **No External Dependencies:** Designed to work without internet connectivity or external services
5. **LAN-Only:** Protocol designed for local network communication only

## External Dependencies

**Protocol Compatibility:**
- IPMsg (飞鸽传书) - open-source LAN messaging protocol
- FeiQ (飞秋) - Chinese variant with extended features

**No external APIs or cloud services required** - fully self-contained LAN application.
