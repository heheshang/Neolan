// Network communication layer - UDP/TCP sockets, packet parsing/sending

pub mod protocol;
pub mod udp;

// Re-export commonly used types
pub use protocol::{
    parse_message,
    serialize_message,
    FileSendRequest,
    FileSendResponse,
    ProtocolMessage,
    PROTOCOL_VERSION,
    msg_type,
    get_message_type_name,
};

pub use udp::{UdpTransport, DEFAULT_UDP_PORT};
