use serde::{Deserialize, Serialize};

/// 用户登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 用户登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
}

/// VPN 节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnNode {
    pub id: String,
    pub name: String,
    pub country: String,
    pub city: String,
    pub protocol: String,
    pub endpoint: String,
    pub port: u16,
    pub public_key: String,
    pub ping: Option<u32>,
    pub load: Option<f32>,
}

/// 连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub id: String,
    pub name: String,
    pub node_id: String,
    pub protocol: String,
    pub auto_connect: bool,
    pub kill_switch: bool,
    pub dns_leak_protection: bool,
}

/// 连接状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "connecting")]
    Connecting,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "disconnecting")]
    Disconnecting,
    #[serde(rename = "error")]
    Error,
}

/// 连接统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStats {
    pub status: ConnectionStatus,
    pub connected_node: Option<String>,
    pub upload_speed: u64,      // bytes/s
    pub download_speed: u64,    // bytes/s
    pub total_uploaded: u64,    // bytes
    pub total_downloaded: u64,  // bytes
    pub connection_time: u64,   // seconds
    pub ip_address: Option<String>,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub auto_start: bool,
    pub minimize_to_tray: bool,
    pub default_protocol: String,
    pub dns_servers: Vec<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            theme: "dark".to_string(),
            language: "en".to_string(),
            auto_start: false,
            minimize_to_tray: true,
            default_protocol: "WireGuard".to_string(),
            dns_servers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
        }
    }
}

impl Default for ConnectionStats {
    fn default() -> Self {
        ConnectionStats {
            status: ConnectionStatus::Disconnected,
            connected_node: None,
            upload_speed: 0,
            download_speed: 0,
            total_uploaded: 0,
            total_downloaded: 0,
            connection_time: 0,
            ip_address: None,
        }
    }
}
