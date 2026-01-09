# Change: Fix Peer Name Parsing from IPMsg Messages

## Why

用户反馈从 IPMsg/FeiQ 消息中解析的用户名显示不正确。从日志可以看到：

```
1_lbt4_41#128#24F5AAD7C96A#0#0#0#311c#9:1767153479:t0250254:DESKTOP-IOHG15K:6291459:陈俞辛
```

当前代码将 `fields[2]=DESKTOP-IOHG15K` 当作用户名，但实际用户名 `陈俞辛` 在 content 字段中。

**FeiQ 协议特殊格式：**
- FeiQ 的 BR_ENTRY 消息中，**用户名放在 content 字段**，而不是 sender_name 字段
- sender_name 字段通常为空或包含其他标识符
- hostname 在 fields[2] 位置

## What Changes

- **修复 FeiQ BR_ENTRY 消息的 username 提取**：从 content 字段提取用户名而不是 sender_name 字段
- **保持标准 IPMsg 格式兼容**：标准 IPMsg 格式仍然从 sender_name 字段读取
- **添加测试用例**：验证中文用户名正确解析

## Impact

- Affected specs: Update `peer-management` spec with correct FeiQ parsing requirements
- Affected code:
  - Backend: `src-tauri/src/network/protocol.rs` - 修复 `parse_message` 返回的 sender_name
  - Backend: `src-tauri/src/modules/peer/manager.rs` - 更新 PeerNode 创建逻辑
  - Backend: 添加 FeiQ 格式解析测试
- Risk: Low - 只修改 FeiQ 格式解析，不影响标准 IPMsg

## Technical Details

### Current Incorrect Parsing

```
1767153479:t0250254:DESKTOP-IOHG15K:6291459:陈俞辛
```

Current code treats:
- fields[2] = `DESKTOP-IOHG15K` as sender_name ❌ (this is hostname)
- fields[3] = `6291459` as sender_host ❌ (this is msg_type)
- fields[4] = `陈俞辛` as content ❌ (this IS the username!)

### Correct FeiQ Format

| Field | Value | Description |
|-------|-------|-------------|
| fields[0] | `1767153479` | Timestamp (10 digits) |
| fields[1] | `t0250254` | Packet ID |
| fields[2] | `DESKTOP-IOHG15K` | Hostname |
| fields[3] | `6291459` | Message Type |
| fields[4] | `陈俞辛` | Content = **Username** (for BR_ENTRY) |

### Fix Strategy

When FeiQ format is detected (10-digit timestamp):
1. Extract hostname from fields[2]
2. Extract msg_type from fields[3]
3. For BR_ENTRY messages (6291459), extract username from content field
4. Set sender_name = content (the actual username)
5. Set sender_host = fields[2] (the hostname)
