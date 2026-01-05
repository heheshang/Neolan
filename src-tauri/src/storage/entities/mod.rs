// src-tauri/src/storage/entities/mod.rs
pub mod peers;
pub mod messages;
pub mod transfers;
pub mod groups;
pub mod settings;
pub mod audit_logs;

pub use peers::Entity as Peers;
pub use messages::Entity as Messages;
pub use transfers::Entity as Transfers;
pub use groups::Entity as Groups;
pub use settings::Entity as Settings;
pub use audit_logs::Entity as AuditLogs;
