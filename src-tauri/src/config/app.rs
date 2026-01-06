// src-tauri/src/config/app.rs
use crate::error::{NeoLanError, Result};
use crate::storage::entities::settings;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

/// 应用程序配置
///
/// 存储应用程序的核心配置参数，包括网络设置、用户信息等
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    /// 用户名（显示给其他节点）
    pub username: String,

    /// 主机名
    pub hostname: String,

    /// 绑定 IP 地址
    pub bind_ip: String,

    /// UDP 端口（控制消息：上线/下线/心跳）
    pub udp_port: u16,

    /// TCP 端口范围（文件传输）
    pub tcp_port_start: u16,
    pub tcp_port_end: u16,

    /// 心跳间隔（秒）
    pub heartbeat_interval: u64,

    /// 超时时间（秒）- 超过此时间未活动的节点视为离线
    pub peer_timeout: u64,

    /// 是否启用加密（AES-256-GCM）
    pub encryption_enabled: bool,

    /// 加密密钥（Base64 编码）
    pub encryption_key: Option<String>,

    /// 离线消息保留天数
    pub offline_message_retention_days: u32,

    /// 是否自动接受文件传输
    pub auto_accept_files: bool,

    /// 文件保存目录
    pub file_save_dir: String,

    /// 日志级别：trace, debug, info, warn, error
    pub log_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            username: whoami::username(),
            hostname: whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string()),
            bind_ip: "0.0.0.0".to_string(),
            udp_port: 2425,
            tcp_port_start: 8000,
            tcp_port_end: 9000,
            heartbeat_interval: 60,
            peer_timeout: 180,
            encryption_enabled: false,
            encryption_key: None,
            offline_message_retention_days: 30,
            auto_accept_files: false,
            file_save_dir: dirs::download_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .to_string_lossy()
                .to_string(),
            log_level: "info".to_string(),
        }
    }
}

/// 配置存储键名常量
mod keys {
    pub const CONFIG: &str = "app_config";
}

/// 配置仓库
///
/// 负责从 settings 表加载和保存配置
#[derive(Clone)]
pub struct ConfigRepository {
    db: DatabaseConnection,
}

impl ConfigRepository {
    /// 创建新的 ConfigRepository
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 加载应用配置
    ///
    /// 如果数据库中没有配置，则返回默认配置
    pub async fn load_app_config(&self) -> Result<AppConfig> {
        let setting = settings::Entity::find()
            .filter(settings::Column::Key.eq(keys::CONFIG))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to load config: {}", e)))?;

        match setting {
            Some(s) => {
                // 从 JSON 反序列化配置
                serde_json::from_str(&s.value).map_err(|e| {
                    NeoLanError::Config(format!("Failed to parse config JSON: {}", e))
                })
            }
            None => Ok(AppConfig::default()),
        }
    }

    /// 保存应用配置
    pub async fn save_app_config(&self, config: &AppConfig) -> Result<()> {
        let json_value = serde_json::to_string(config).map_err(|e| {
            NeoLanError::Config(format!("Failed to serialize config: {}", e))
        })?;

        // 检查是否已存在配置
        let existing = settings::Entity::find()
            .filter(settings::Column::Key.eq(keys::CONFIG))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to query config: {}", e)))?;

        let now = chrono::Utc::now().naive_utc();

        if let Some(existing_setting) = existing {
            // 更新现有配置
            let mut active_model: settings::ActiveModel = existing_setting.into();
            active_model.value = Set(json_value);
            active_model.updated_at = Set(now);

            settings::Entity::update(active_model)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to update config: {}", e)))?;
        } else {
            // 插入新配置
            let active_model = settings::ActiveModel {
                key: Set(keys::CONFIG.to_string()),
                value: Set(json_value),
                updated_at: Set(now),
            };

            settings::Entity::insert(active_model)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to insert config: {}", e)))?;
        }

        Ok(())
    }

    /// 重置为默认配置
    pub async fn reset_to_default(&self) -> Result<()> {
        self.save_app_config(&AppConfig::default()).await
    }

    /// 获取单个配置值
    ///
    /// # 参数
    /// - `key`: 配置键名
    ///
    /// # 返回
    /// 配置值的 JSON 字符串
    pub async fn get_value(&self, key: &str) -> Result<Option<String>> {
        let setting = settings::Entity::find()
            .filter(settings::Column::Key.eq(key))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to get value: {}", e)))?;

        Ok(setting.map(|s| s.value))
    }

    /// 设置单个配置值
    ///
    /// # 参数
    /// - `key`: 配置键名
    /// - `value`: 配置值（JSON 字符串）
    pub async fn set_value(&self, key: &str, value: &str) -> Result<()> {
        let now = chrono::Utc::now().naive_utc();

        // 检查是否已存在
        let existing = settings::Entity::find()
            .filter(settings::Column::Key.eq(key))
            .one(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to query value: {}", e)))?;

        if let Some(existing_setting) = existing {
            // 更新
            let mut active_model: settings::ActiveModel = existing_setting.into();
            active_model.value = Set(value.to_string());
            active_model.updated_at = Set(now);

            settings::Entity::update(active_model)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to update value: {}", e)))?;
        } else {
            // 插入
            let active_model = settings::ActiveModel {
                key: Set(key.to_string()),
                value: Set(value.to_string()),
                updated_at: Set(now),
            };

            settings::Entity::insert(active_model)
                .exec(&self.db)
                .await
                .map_err(|e| NeoLanError::Storage(format!("Failed to insert value: {}", e)))?;
        }

        Ok(())
    }

    /// 删除配置值
    pub async fn delete_value(&self, key: &str) -> Result<()> {
        let result = settings::Entity::delete_many()
            .filter(settings::Column::Key.eq(key))
            .exec(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to delete value: {}", e)))?;

        if result.rows_affected == 0 {
            return Err(NeoLanError::Config(format!("Config key not found: {}", key)));
        }

        Ok(())
    }

    /// 获取所有配置项
    pub async fn get_all_settings(&self) -> Result<Vec<settings::Model>> {
        let result = settings::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| NeoLanError::Storage(format!("Failed to get all settings: {}", e)))?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();

        // 验证默认值
        assert!(!config.username.is_empty());
        assert!(!config.hostname.is_empty());
        assert_eq!(config.udp_port, 2425);
        assert_eq!(config.tcp_port_start, 8000);
        assert_eq!(config.tcp_port_end, 9000);
        assert_eq!(config.heartbeat_interval, 60);
        assert_eq!(config.peer_timeout, 180);
        assert_eq!(config.offline_message_retention_days, 30);
        assert!(!config.encryption_enabled);
        assert!(!config.auto_accept_files);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();

        // 测试序列化
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"username\""));
        assert!(json.contains("\"udp_port\""));

        // 测试反序列化
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, config);
    }

    #[test]
    fn test_config_validation() {
        let mut config = AppConfig::default();

        // 验证端口范围
        assert!(config.tcp_port_start < config.tcp_port_end);
        assert!(config.udp_port > 0);

        // 验证超时设置
        assert!(config.peer_timeout > config.heartbeat_interval);
    }
}
