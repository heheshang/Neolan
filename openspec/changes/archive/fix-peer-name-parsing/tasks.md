## 1. 分析 FeiQ 消息格式
- [x] 1.1 确认 FeiQ BR_ENTRY 消息的正确字段映射
- [x] 1.2 确认标准 IPMsg 格式字段映射

## 2. 修复 protocol.rs 中的解析逻辑
- [x] 2.1 更新 FeiQ 格式检测逻辑（检测 fields[1] 的 10 位时间戳）
- [x] 2.2 修正字段映射：hostname 从 fields[3], msg_type 从 fields[4]
- [x] 2.3 对于 BR_ENTRY 消息，从 content (fields[5+]) 提取 username

## 3. 更新 PeerManager 处理逻辑
- [x] 3.1 确保从 ProtocolMessage 正确提取 username
- [x] 3.2 验证 PeerNode 创建时使用正确的字段

## 4. 添加测试用例
- [x] 4.1 添加 FeiQ 格式中文用户名解析测试
- [x] 4.2 添加标准 IPMsg 格式兼容性测试
- [x] 4.3 验证 display_name 计算正确

## 5. 验证和测试
- [x] 5.1 运行 cargo test 验证测试通过 (18 passed)
- [x] 5.2 使用实际 FeiQ 消息验证解析结果
- [x] 5.3 验证前端用户列表正确显示中文用户名

## 实施总结

### 完成的修改

**src-tauri/src/network/protocol.rs**
- 更新 FeiQ 格式检测：从检查 fields[0] 改为检查 fields[1] 是否为 10 位时间戳
- 修正字段映射：
  - hostname: fields[3]
  - msg_type: fields[4]
  - content (username for BR_ENTRY): fields[5+]
- 添加非数字 packet_id 的容错处理（使用时间戳回退）
- 添加调试日志输出

**测试用例**
- `test_parse_feiq_chinese_username`: 验证中文用户名 "陈俞辛" 正确解析
- `test_parse_feiq_with_regular_username`: 验证 ASCII 用户名 "Alice" 正确解析

### 测试结果
所有 18 个协议测试通过：
- test_parse_feiq_chinese_username ... ok
- test_parse_feiq_with_regular_username ... ok
- 其他 16 个测试 ... ok

### 关键发现
- FeiQ 格式: `version:timestamp:packet_id:hostname:msg_type:content`
- 标准 IPMsg 格式: `version:packet_id:sender_name:sender_host:msg_type:content`
- FeiQ BR_ENTRY 消息的 username 在 content 字段，不在 sender_name 字段
