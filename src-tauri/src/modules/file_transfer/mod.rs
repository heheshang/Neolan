// File transfer module - chunking, resume, integrity checks

pub mod types;
pub mod manager;
pub mod response;

// Re-export commonly used types
pub use types::{TransferDirection, TransferStatus, TransferTask};
pub use manager::FileTransferManager;
pub use response::{FileTransferResponse, PendingRequest};
