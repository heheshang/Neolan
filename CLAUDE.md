# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NeoLan is a LAN (Local Area Network) instant messaging and file transfer system built with a decentralized P2P architecture. It uses Tauri 2.x for the desktop framework, Vue 3 + TypeScript for the frontend UI, and Rust for the backend networking and data processing.

## Development Commands

### Frontend Development
```bash
# Start Vite dev server (runs on port 1420)
npm run dev

# Type check Vue files
npm run build

# Preview production build
npm run preview
```

### Tauri Development
```bash
# Run Tauri development mode (starts both Vite and Rust)
npm run tauri dev

# Build desktop application
npm run tauri build
```

### Rust Backend (in src-tauri/)
```bash
# Run Rust tests
cd src-tauri && cargo test

# Check code without building
cd src-tauri && cargo check

# Format Rust code
cd src-tauri && cargo fmt

# Run linter
cd src-tauri && cargo clippy
```

## Architecture

### High-Level Architecture

NeoLan uses a **decentralized P2P architecture** - no central server is required. All client nodes are equal and communicate directly via LAN.

```
Frontend (Vue 3)  ←→  Tauri IPC Bridge  ←→  Rust Backend
                       (invoke/commands)
                                                    ↓
                            UDP (control) + TCP (data) Networking
```

### Technology Stack

| Layer | Technology |
|-------|-----------|
| **Desktop Framework** | Tauri 2.x |
| **Frontend UI** | Vue 3 (Composition API + `<script setup>`) + TypeScript |
| **State Management** | Pinia (planned) |
| **Build Tool** | Vite (dev server on port 1420) |
| **Backend Language** | Rust (edition 2021) |
| **Async Runtime** | tokio (planned) |
| **Storage** | SQLite (planned) |

### Planned Core Modules (Rust Backend)

The backend is organized into six core modules (defined in `neolan-design-document.md`):

| Module | Responsibility | Location (planned) |
|--------|---------------|-------------------|
| **Network** | UDP/TCP socket I/O, packet parsing/sending | `src-tauri/src/network/` |
| **Message** | Protocol handling, encryption/decryption, routing | `src-tauri/src/message/` |
| **FileTransfer** | File chunking, resume, integrity checks | `src-tauri/src/file_transfer/` |
| **Peer** | Node discovery, heartbeat, group management | `src-tauri/src/peer/` |
| **Storage** | SQLite DB, config, message history | `src-tauri/src/storage/` |
| **UI Bridge** | Tauri commands for frontend | `src-tauri/src/commands/` |

### Frontend Structure (planned)

```
src/
├── views/           # Page-level components
├── components/      # Reusable UI components
├── stores/          # Pinia stores for state
├── api/             # Tauri invoke wrappers
├── types/           # TypeScript type definitions
├── utils/           # Helper functions
└── assets/          # Static assets
```

### Tauri Command Pattern

Frontend calls Rust via `invoke()` from `@tauri-apps/api/core`:

**Frontend (Vue):**
```typescript
import { invoke } from "@tauri-apps/api/core";
const result = await invoke("command_name", { arg1: value1 });
```

**Backend (Rust) - register in `src-tauri/src/lib.rs`:**
```rust
#[tauri::command]
fn command_name(arg1: String) -> Result<String, String> {
    // ...
}

// Add to invoke_handler:
.invoke_handler(tauri::generate_handler![command_name])
```

## Communication Protocol

NeoLan is compatible with **IPMsg (飞鸽传书)** protocol with extensions:

- **Default UDP port:** 2425 (for control messages: online/offline/heartbeat)
- **TCP ports:** 8000-9000 range (for file data)
- **Encoding:** UTF-8
- **Protocol format:** `version:packet_id:sender_name:hostname:msg_type:content[:ext_fields]`

See `neolan-design-document.md` section 3 for full protocol specification.

## Key Design Decisions

1. **P2P Architecture:** No server needed - works in isolated LAN environments
2. **UDP + TCP Hybrid:** UDP for low-latency control, TCP for reliable file transfer
3. **Encryption:** AES-256-GCM for messages (planned)
4. **Local Storage:** SQLite for chat history and config (no external DB)

## Development Notes

- The Vite dev server runs on **port 1420** with `strictPort: true` - Tauri expects this fixed port
- Vite watches are configured to **ignore `src-tauri/`** to prevent duplicate rebuilds
- Rust library crate name is `neolan_lib` (suffix avoids Windows naming conflicts)
- Use `#[tauri::command]` attribute macro to expose Rust functions to frontend
- Use `serde::Serialize`/`Deserialize` for data structures passed across the boundary

## Related Documentation

- `neolan-design-document.md` - Complete technical design specification
- `飞秋（FeiQ）产品功能详细文档.md` - Feature requirements (Chinese)
- `飞秋（FeiQ）技术架构与通信协议全解析.md` - Reference implementation analysis (Chinese)
