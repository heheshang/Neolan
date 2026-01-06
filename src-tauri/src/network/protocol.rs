// IPMsg-compatible protocol parser and serializer
//
// Protocol format:
// version:packet_id:sender_name:sender_host:msg_type:content[:ext_fields]
//
// Encoding: UTF-8
// Default UDP port: 2425

use crate::{NeoLanError, Result};
use serde::{Deserialize, Serialize};

/// Message type constants (compatible with IPMsg protocol)
pub mod msg_type {
   // IP Messenger (IPMSG) 常量与工具 (Rust)
// 基于原始 ipmsg.h / IPMSG.h 的权威映射（mode = 低 8 位，options = 高 24 位）
#![allow(non_upper_case_globals)]

/// helpers: 从 command 中取出 mode (低 8 位) 与 opts (高 24 位)
#[inline]
pub const fn get_mode(command: u32) -> u8 {
    (command & 0x000000ff) as u8
}
#[inline]
pub const fn get_opt(command: u32) -> u32 {
    command & 0xffffff00
}
#[inline]
pub const fn has_opt(command: u32, flag: u32) -> bool {
    (get_opt(command) & flag) != 0
}

/// 协议头 / 版本 / 端口
pub const IPMSG_VERSION: u16 = 0x0001;        // 协议版本
pub const IPMSG_DEFAULT_PORT: u16 = 0x0979;  // 2425

/// command (mode) — 低 8 位
pub const IPMSG_NOOPERATION: u32    = 0x00000000; // 0 无操作
pub const IPMSG_BR_ENTRY: u32       = 0x00000001; // 1 广播上线（entry）
pub const IPMSG_BR_EXIT: u32        = 0x00000002; // 2 广播下线（exit）
pub const IPMSG_ANSENTRY: u32       = 0x00000003; // 3 对 BR_ENTRY 的应答
pub const IPMSG_BR_ABSENCE: u32     = 0x00000004; // 4 广播缺席

pub const IPMSG_BR_ISGETLIST: u32   = 0x00000010; // 16 请求是否需要列表（is-getlist）
pub const IPMSG_OKGETLIST: u32      = 0x00000011; // 17 同意发送列表（ok-getlist）
pub const IPMSG_GETLIST: u32        = 0x00000012; // 18 请求列表（getlist）
pub const IPMSG_ANSLIST: u32        = 0x00000013; // 19 返回列表（anslist）
pub const IPMSG_BR_ISGETLIST2: u32  = 0x00000018; // 24 请求扩展列表（is-getlist2）

pub const IPMSG_SENDMSG: u32        = 0x00000020; // 32 发送消息
pub const IPMSG_RECVMSG: u32        = 0x00000021; // 33 接收确认
pub const IPMSG_READMSG: u32        = 0x00000030; // 48 消息已读
pub const IPMSG_DELMSG: u32         = 0x00000031; // 49 删除消息
pub const IPMSG_ANSREADMSG: u32     = 0x00000032; // 50 对已读的应答

pub const IPMSG_GETINFO: u32        = 0x00000040; // 64 请求用户信息
pub const IPMSG_SENDINFO: u32       = 0x00000041; // 65 发送用户信息

pub const IPMSG_GETABSENCEINFO: u32 = 0x00000050; // 80 请求缺席信息
pub const IPMSG_SENDABSENCEINFO: u32= 0x00000051; // 81 发送缺席信息

pub const IPMSG_GETFILEDATA: u32    = 0x00000060; // 96 请求文件数据（文件传输）
pub const IPMSG_RELEASEFILES: u32   = 0x00000061; // 97 释放文件资源
pub const IPMSG_GETDIRFILES: u32    = 0x00000062; // 98 请求目录文件列表

pub const IPMSG_GETPUBKEY: u32      = 0x00000072; // 114 请求公钥
pub const IPMSG_ANSPUBKEY: u32      = 0x00000073; // 115 应答公钥

/// option / flags（通用 / 全局）
pub const IPMSG_ABSENCEOPT: u32     = 0x00000100; // 256 缺席标志（全局）
pub const IPMSG_SERVEROPT: u32      = 0x00000200; // 512 服务器标志（全局）
pub const IPMSG_DIALUPOPT: u32      = 0x00010000; // 65536 拨号连接标志
pub const IPMSG_FILEATTACHOPT: u32  = 0x00200000; // 2097152 文件附加标志
pub const IPMSG_ENCRYPTOPT: u32     = 0x00400000; // 4194304 加密标志
pub const IPMSG_UTF8OPT: u32        = 0x00800000; // 8388608 UTF-8 编码标志

/// option for send command（发送上下文特有）
// 注意：有些值与上面的“通用”标志数值相同 —— 解释时应基于 mode（即先 GET_MODE）
pub const IPMSG_SENDCHECKOPT: u32   = 0x00000100; // 256 发送确认（send 上下文）
pub const IPMSG_SECRETOPT: u32      = 0x00000200; // 512 私密发送（send 上下文）
pub const IPMSG_BROADCASTOPT: u32   = 0x00000400; // 1024 广播发送（send 上下文）
pub const IPMSG_MULTICASTOPT: u32   = 0x00000800; // 2048 多播发送
pub const IPMSG_NOPOPUPOPT: u32     = 0x00001000; // 4096 不弹出（接收端）
pub const IPMSG_AUTORETOPT: u32     = 0x00002000; // 8192 自动回复请求
pub const IPMSG_RETRYOPT: u32       = 0x00004000; // 16384 重试选项
pub const IPMSG_PASSWORDOPT: u32    = 0x00008000; // 32768 带密码发送
pub const IPMSG_NOLOGOPT: u32       = 0x00020000; // 131072 不记录日志

// 下面给出一些常用组合构造函数作为参考：
#[inline]
pub const fn make_command(mode: u32, opts: u32) -> u32 {
    (mode & 0x000000ff) | (opts & 0xffffff00)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mode_opt_helpers() {
        let cmd = make_command(IPMSG_SENDMSG, IPMSG_FILEATTACHOPT | IPMSG_SENDCHECKOPT);
        assert_eq!(get_mode(cmd), IPMSG_SENDMSG as u8);
        assert!(has_opt(cmd, IPMSG_FILEATTACHOPT));
        assert!(has_opt(cmd, IPMSG_SENDCHECKOPT));
        assert!(!has_opt(cmd, IPMSG_ENCRYPTOPT));
    }
}
}

/// IPMsg protocol version (NeoLan uses version 1)
pub const PROTOCOL_VERSION: u8 = 1;

/// Protocol delimiter
const PROTOCOL_DELIMITER: &str = ":";

/// Minimum number of protocol fields
const MIN_FIELD_COUNT: usize = 6;

/// Maximum packet ID value
const MAX_PACKET_ID: u64 = u32::MAX as u64;

/// Maximum message content size (1MB)
const MAX_CONTENT_SIZE: usize = 1024 * 1024;

/// IPMsg-compatible protocol message
///
/// This structure represents a single message in the IPMsg protocol format.
/// All text fields are encoded in UTF-8.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolMessage {
    /// Protocol version (NeoLan uses 1)
    pub version: u8,

    /// Packet ID (monotonically increasing)
    pub packet_id: u64,

    /// Sender's username (display name)
    pub sender_name: String,

    /// Sender's hostname
    pub sender_host: String,

    /// Message type (see msg_type constants)
    pub msg_type: u32,

    /// Message content (format depends on msg_type)
    pub content: String,
}

/// File transfer request (JSON content for FILE_SEND_REQ)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileSendRequest {
    /// File name
    pub name: String,

    /// File size in bytes
    pub size: u64,

    /// MD5 hash (hex string)
    pub md5: String,
}

/// File transfer response (JSON content for FILE_SEND_RSP)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileSendResponse {
    /// true = accept, false = reject
    pub accept: bool,

    /// TCP port for data transfer (only if accept = true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

/// Parse a byte stream into a ProtocolMessage
///
/// # Arguments
/// * `data` - Raw bytes received from UDP socket
///
/// # Returns
/// * `Ok(ProtocolMessage)` - Successfully parsed message
/// * `Err(NeoLanError)` - Parse error
///
/// # Protocol Format
/// Standard: `version:packet_id:sender_name:sender_host:msg_type:content[:ext_fields]`
/// FeiQ: `1_lbt4_6#128#...#packet_id:timestamp:sender_name:sender_host:msg_type:content`
///
/// # Examples
/// ```no_run
/// # use neolan_lib::network::parse_message;
/// # use neolan_lib::NeoLanError;
/// let data = b"1:123:Alice:alice-pc:4:Hello World";
/// let msg = parse_message(data)?;
/// # Ok::<(), NeoLanError>(())
/// ```
pub fn parse_message(data: &[u8]) -> Result<ProtocolMessage> {
    // Convert bytes to UTF-8 string (using lossy conversion for FeiQ GBK encoding)
    let message_str = String::from_utf8_lossy(data);

    // Handle FeiQ hybrid format: contains '#' followed by IPMsg-compatible section
    // FeiQ format: 1_lbt4_6#128#C81F663237C8#0#0#0#311c#9:1761386707:cgc:DESKTOP-IOHG15K:6291459:...
    // The actual IPMsg section starts after the last '#'
    let parse_section = if message_str.contains('#') {
        // Find the last '#' and extract the IPMsg section after it
        if let Some(last_hash_pos) = message_str.rfind('#') {
            &message_str[last_hash_pos + 1..]
        } else {
            &message_str
        }
    } else {
        &message_str
    };

    // Split by delimiter
    let fields: Vec<&str> = parse_section
        .split(PROTOCOL_DELIMITER)
        .collect();

    // Validate minimum field count
    if fields.len() < MIN_FIELD_COUNT {
        return Err(NeoLanError::Protocol(format!(
            "Invalid message format: expected at least {} fields, got {}. Data: {}",
            MIN_FIELD_COUNT,
            fields.len(),
            parse_section
        )));
    }

    // Detect FeiQ format: packet_id:timestamp:sender_name:sender_host:msg_type:content
    // vs standard: version:packet_id:sender_name:sender_host:msg_type:content
    // FeiQ timestamp is typically a 10-digit Unix timestamp (e.g., 1761386707)
    let (version, packet_id, sender_name, sender_host) = {
        // Try to detect if fields[1] is a timestamp (10-digit number)
        if fields.len() >= 6 && fields[1].len() == 10 && fields[1].chars().all(|c| c.is_ascii_digit()) {
            // FeiQ format detected: fields[0]=packet_id, fields[1]=timestamp, fields[2]=name, fields[3]=host
            tracing::debug!("Detected FeiQ format message (with timestamp field)");
            (PROTOCOL_VERSION, fields[0], fields[2].to_string(), fields[3].to_string())
        } else {
            // Standard IPMsg format
            let v: u8 = fields[0]
                .parse()
                .map_err(|_| NeoLanError::Protocol(format!("Invalid version: {}", fields[0])))?;
            (v, fields[1], fields[2].to_string(), fields[3].to_string())
        }
    };

    // Validate version (FeiQ compatibility: only warn, don't error)
    if version != PROTOCOL_VERSION {
        tracing::warn!("Protocol version {} differs from expected {}, accepting for compatibility",
            version, PROTOCOL_VERSION);
    }

    // Parse packet ID
    let packet_id: u64 = packet_id
        .parse()
        .map_err(|_| NeoLanError::Protocol(format!("Invalid packet_id: {}", packet_id)))?;

    // Validate packet ID range
    if packet_id > MAX_PACKET_ID {
        return Err(NeoLanError::Protocol(format!(
            "Packet ID out of range: {} (max {})",
            packet_id, MAX_PACKET_ID
        )));
    }

    // Validate sender name is not empty
    if sender_name.is_empty() {
        return Err(NeoLanError::Protocol("Sender name cannot be empty".to_string()));
    }

    // Validate sender host is not empty
    if sender_host.is_empty() {
        return Err(NeoLanError::Protocol("Sender host cannot be empty".to_string()));
    }

    // Parse message type (at fields[4] for both FeiQ and standard formats)
    let msg_type: u32 = fields[4]
        .parse()
        .map_err(|_| NeoLanError::Protocol(format!("Invalid msg_type: {}", fields[4])))?;

    // Extract content (fields 5+ are joined with ":")
    // This allows content to contain ":" as well
    let content = if fields.len() > 6 {
        fields[5..].join(PROTOCOL_DELIMITER)
    } else {
        fields[5].to_string()
    };

    // Validate content size
    if content.len() > MAX_CONTENT_SIZE {
        return Err(NeoLanError::Protocol(format!(
            "Content too large: {} bytes (max {})",
            content.len(),
            MAX_CONTENT_SIZE
        )));
    }

    Ok(ProtocolMessage {
        version,
        packet_id,
        sender_name,
        sender_host,
        msg_type,
        content,
    })
}

/// Serialize a ProtocolMessage into a byte stream
///
/// # Arguments
/// * `msg` - Protocol message to serialize
///
/// # Returns
/// * `Ok(Vec<u8>)` - Serialized bytes ready for UDP transmission
/// * `Err(NeoLanError)` - Serialization error
///
/// # Protocol Format
/// `version:packet_id:sender_name:sender_host:msg_type:content`
///
/// # Examples
/// ```no_run
/// # use neolan_lib::network::{ProtocolMessage, serialize_message, msg_type};
/// # use neolan_lib::NeoLanError;
/// let msg = ProtocolMessage {
///     version: 1,
///     packet_id: 123,
///     sender_name: "Alice".to_string(),
///     sender_host: "alice-pc".to_string(),
///     msg_type: msg_type::MSG_SEND,
///     content: "Hello World".to_string(),
/// };
/// let bytes = serialize_message(&msg)?;
/// # Ok::<(), NeoLanError>(())
/// ```
pub fn serialize_message(msg: &ProtocolMessage) -> Result<Vec<u8>> {
    // Validate version
    if msg.version != PROTOCOL_VERSION {
        return Err(NeoLanError::Protocol(format!(
            "Unsupported protocol version: {} (expected {})",
            msg.version, PROTOCOL_VERSION
        )));
    }

    // Validate packet ID range
    if msg.packet_id > MAX_PACKET_ID {
        return Err(NeoLanError::Protocol(format!(
            "Packet ID out of range: {} (max {})",
            msg.packet_id, MAX_PACKET_ID
        )));
    }

    // Validate sender name
    if msg.sender_name.is_empty() {
        return Err(NeoLanError::Protocol("Sender name cannot be empty".to_string()));
    }

    // Check for delimiter in sender name
    if msg.sender_name.contains(PROTOCOL_DELIMITER) {
        return Err(NeoLanError::Protocol(
            "Sender name cannot contain ':'".to_string(),
        ));
    }

    // Validate sender host
    if msg.sender_host.is_empty() {
        return Err(NeoLanError::Protocol("Sender host cannot be empty".to_string()));
    }

    // Check for delimiter in sender host
    if msg.sender_host.contains(PROTOCOL_DELIMITER) {
        return Err(NeoLanError::Protocol(
            "Sender host cannot contain ':'".to_string(),
        ));
    }

    // Validate content size
    if msg.content.len() > MAX_CONTENT_SIZE {
        return Err(NeoLanError::Protocol(format!(
            "Content too large: {} bytes (max {})",
            msg.content.len(),
            MAX_CONTENT_SIZE
        )));
    }

    // Build protocol string
    let protocol_string = format!(
        "{}:{}:{}:{}:{}:{}",
        msg.version,
        msg.packet_id,
        msg.sender_name,
        msg.sender_host,
        msg.msg_type,
        msg.content
    );

    // Convert to bytes (UTF-8)
    Ok(protocol_string.into_bytes())
}

/// Get message type name for debugging
/// Uses IPMsg protocol standard message type names
pub fn get_message_type_name(msg_type: u32) -> &'static str {
    // Extract mode (low 8 bits) for comparison
    let mode = msg_type::get_mode(msg_type) as u32;

    match mode {
        msg_type::IPMSG_NOOPERATION => "IPMSG_NOOPERATION",
        msg_type::IPMSG_BR_ENTRY => "IPMSG_BR_ENTRY",
        msg_type::IPMSG_BR_EXIT => "IPMSG_BR_EXIT",
        msg_type::IPMSG_ANSENTRY => "IPMSG_ANSENTRY",
        msg_type::IPMSG_BR_ABSENCE => "IPMSG_BR_ABSENCE",
        msg_type::IPMSG_BR_ISGETLIST => "IPMSG_BR_ISGETLIST",
        msg_type::IPMSG_OKGETLIST => "IPMSG_OKGETLIST",
        msg_type::IPMSG_GETLIST => "IPMSG_GETLIST",
        msg_type::IPMSG_ANSLIST => "IPMSG_ANSLIST",
        msg_type::IPMSG_BR_ISGETLIST2 => "IPMSG_BR_ISGETLIST2",
        msg_type::IPMSG_SENDMSG => "IPMSG_SENDMSG",
        msg_type::IPMSG_RECVMSG => "IPMSG_RECVMSG",
        msg_type::IPMSG_READMSG => "IPMSG_READMSG",
        msg_type::IPMSG_DELMSG => "IPMSG_DELMSG",
        msg_type::IPMSG_ANSREADMSG => "IPMSG_ANSREADMSG",
        msg_type::IPMSG_GETINFO => "IPMSG_GETINFO",
        msg_type::IPMSG_SENDINFO => "IPMSG_SENDINFO",
        msg_type::IPMSG_GETABSENCEINFO => "IPMSG_GETABSENCEINFO",
        msg_type::IPMSG_SENDABSENCEINFO => "IPMSG_SENDABSENCEINFO",
        msg_type::IPMSG_GETFILEDATA => "IPMSG_GETFILEDATA",
        msg_type::IPMSG_RELEASEFILES => "IPMSG_RELEASEFILES",
        msg_type::IPMSG_GETDIRFILES => "IPMSG_GETDIRFILES",
        msg_type::IPMSG_GETPUBKEY => "IPMSG_GETPUBKEY",
        msg_type::IPMSG_ANSPUBKEY => "IPMSG_ANSPUBKEY",
        _ => "UNKNOWN",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text_message() {
        let data = b"1:123:Alice:alice-pc:32:Hello World";  // 32 = IPMSG_SENDMSG
        let msg = parse_message(data).unwrap();

        assert_eq!(msg.version, 1);
        assert_eq!(msg.packet_id, 123);
        assert_eq!(msg.sender_name, "Alice");
        assert_eq!(msg.sender_host, "alice-pc");
        assert_eq!(msg.msg_type, msg_type::IPMSG_SENDMSG);
        assert_eq!(msg.content, "Hello World");
    }

    #[test]
    fn test_serialize_and_parse() {
        let original = ProtocolMessage {
            version: 1,
            packet_id: 456,
            sender_name: "Bob".to_string(),
            sender_host: "bob-pc".to_string(),
            msg_type: msg_type::IPMSG_BR_ENTRY,
            content: "".to_string(),
        };

        let bytes = serialize_message(&original).unwrap();
        let parsed = parse_message(&bytes).unwrap();

        assert_eq!(parsed, original);
    }

    #[test]
    fn test_parse_empty_content() {
        let data = b"1:1:Alice:alice-pc:1:";  // 1 = IPMSG_BR_ENTRY
        let msg = parse_message(data).unwrap();

        assert_eq!(msg.version, 1);
        assert_eq!(msg.msg_type, msg_type::IPMSG_BR_ENTRY);
        assert_eq!(msg.content, "");
    }

    #[test]
    fn test_serialize_empty_content() {
        let msg = ProtocolMessage {
            version: 1,
            packet_id: 1,
            sender_name: "Test".to_string(),
            sender_host: "test-pc".to_string(),
            msg_type: msg_type::IPMSG_BR_ENTRY,
            content: "".to_string(),
        };

        let bytes = serialize_message(&msg).unwrap();
        let parsed = parse_message(&bytes).unwrap();

        assert_eq!(parsed.content, "");
    }

    #[test]
    fn test_content_with_colon() {
        // Content containing ":" should be preserved by joining fields 5+
        let data = b"1:1:Alice:alice-pc:32:Time: 12:30:45";  // 32 = IPMSG_SENDMSG
        let msg = parse_message(data).unwrap();

        // Fields 5+ are joined with ":"
        assert_eq!(msg.content, "Time: 12:30:45");

        // JSON encoding also works with colons
        let json_content = serde_json::json!({"time": "12:30:45"});
        let msg = ProtocolMessage {
            version: 1,
            packet_id: 1,
            sender_name: "Alice".to_string(),
            sender_host: "alice-pc".to_string(),
            msg_type: msg_type::IPMSG_SENDMSG,
            content: json_content.to_string(),
        };

        let bytes = serialize_message(&msg).unwrap();
        let parsed = parse_message(&bytes).unwrap();

        assert_eq!(parsed.content, r#"{"time":"12:30:45"}"#);
    }

    #[test]
    fn test_serialize_file_request() {
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
            msg_type: msg_type::IPMSG_GETFILEDATA,
            content: serde_json::to_string(&request).unwrap(),
        };

        let bytes = serialize_message(&msg).unwrap();
        let parsed = parse_message(&bytes).unwrap();

        assert_eq!(parsed.msg_type, msg_type::IPMSG_GETFILEDATA);

        let parsed_request: FileSendRequest = serde_json::from_str(&parsed.content).unwrap();
        assert_eq!(parsed_request.name, "document.pdf");
        assert_eq!(parsed_request.size, 1024000);
    }

    #[test]
    fn test_serialize_file_response_accept() {
        let response = FileSendResponse {
            accept: true,
            port: Some(8001),
        };

        let content = serde_json::to_string(&response).unwrap();

        assert!(content.contains(r#""accept":true"#));
        assert!(content.contains(r#""port":8001"#));
    }

    #[test]
    fn test_serialize_file_response_reject() {
        let response = FileSendResponse {
            accept: false,
            port: None,
        };

        let content = serde_json::to_string(&response).unwrap();

        assert!(content.contains(r#""accept":false"#));
        assert!(!content.contains("port"));
    }

    #[test]
    fn test_invalid_utf8() {
        let data = &[0xFF, 0xFF, 0xFF]; // Invalid UTF-8
        let result = parse_message(data);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_version() {
        // For compatibility with FeiQ and other IPMsg implementations,
        // parse_message accepts different versions (warns but doesn't error)
        let data = b"2:1:Alice:alice-pc:32:test";  // 32 = IPMSG_SENDMSG
        let result = parse_message(data);

        // Should parse successfully (version 2 is accepted for compatibility)
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert_eq!(msg.version, 2);

        // Version 1 is also accepted
        let data = b"1:1:Alice:alice-pc:32:test";
        let result = parse_message(data);
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert_eq!(msg.version, 1);
    }

    #[test]
    fn test_serialize_invalid_version() {
        let msg = ProtocolMessage {
            version: 2,
            packet_id: 1,
            sender_name: "Test".to_string(),
            sender_host: "test-pc".to_string(),
            msg_type: msg_type::IPMSG_SENDMSG,
            content: "test".to_string(),
        };

        let result = serialize_message(&msg);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_sender_name() {
        let data = b"1:1::alice-pc:32:test";  // 32 = IPMSG_SENDMSG
        let result = parse_message(data);

        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_empty_sender_name() {
        let msg = ProtocolMessage {
            version: 1,
            packet_id: 1,
            sender_name: "".to_string(),
            sender_host: "test-pc".to_string(),
            msg_type: msg_type::IPMSG_SENDMSG,
            content: "test".to_string(),
        };

        let result = serialize_message(&msg);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_message_type_name() {
        assert_eq!(get_message_type_name(msg_type::IPMSG_SENDMSG), "IPMSG_SENDMSG");
        assert_eq!(get_message_type_name(msg_type::IPMSG_BR_ENTRY), "IPMSG_BR_ENTRY");
        assert_eq!(get_message_type_name(0xFFFFFFFF), "UNKNOWN");
    }

    #[test]
    fn test_packet_id_overflow() {
        let msg = ProtocolMessage {
            version: 1,
            packet_id: u64::MAX,
            sender_name: "Test".to_string(),
            sender_host: "test-pc".to_string(),
            msg_type: msg_type::IPMSG_SENDMSG,
            content: "test".to_string(),
        };

        let result = serialize_message(&msg);
        assert!(result.is_err());
    }

    #[test]
    fn test_content_too_large() {
        let large_content = "x".repeat(MAX_CONTENT_SIZE + 1);

        let msg = ProtocolMessage {
            version: 1,
            packet_id: 1,
            sender_name: "Test".to_string(),
            sender_host: "test-pc".to_string(),
            msg_type: msg_type::IPMSG_SENDMSG,
            content: large_content,
        };

        let result = serialize_message(&msg);
        assert!(result.is_err());
    }
}
