use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 对等体配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerConfig {
    /// 对等体公钥
    pub public_key: String,
    /// 允许的 IP 地址范围
    pub allowed_ips: String,
    /// 对等体端点（可选，用于客户端连接）
    pub endpoint: Option<String>,
    /// 预共享密钥（可选）
    pub psk: Option<String>,
}

/// 接口配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceConfig {
    /// 接口名称
    pub name: String,
    /// 私钥
    pub private_key: String,
    /// 接口地址
    pub address: String,
    /// 监听端口
    pub listen_port: u16,
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// 接口配置
    pub interface: InterfaceConfig,
    /// 对等体列表
    pub peers: Vec<PeerConfig>,
}

/// 客户端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    /// 私钥
    pub private_key: String,
    /// 本地地址
    pub address: String,
    /// 服务器公钥
    pub server_public_key: String,
    /// 服务器端点
    pub server_endpoint: String,
    /// 预共享密钥（可选）
    pub psk: Option<String>,
    /// DNS 服务器（可选）
    pub dns: Option<Vec<String>>,
}

impl ServerConfig {
    /// 从文件加载服务器配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| Error::ConfigError(format!("Failed to read config file: {}", e)))?;
        toml::from_str(&content)
            .map_err(|e| Error::ConfigError(format!("Failed to parse config: {}", e)))
    }

    /// 保存配置到文件
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::ConfigError(format!("Failed to serialize config: {}", e)))?;
        fs::write(path, content)
            .map_err(|e| Error::ConfigError(format!("Failed to write config file: {}", e)))?;
        Ok(())
    }
}

impl ClientConfig {
    /// 从文件加载客户端配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| Error::ConfigError(format!("Failed to read config file: {}", e)))?;
        toml::from_str(&content)
            .map_err(|e| Error::ConfigError(format!("Failed to parse config: {}", e)))
    }

    /// 保存配置到文件
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::ConfigError(format!("Failed to serialize config: {}", e)))?;
        fs::write(path, content)
            .map_err(|e| Error::ConfigError(format!("Failed to write config file: {}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_serialization() {
        let config = ServerConfig {
            interface: InterfaceConfig {
                name: "wg0".to_string(),
                private_key: "test_key".to_string(),
                address: "10.8.0.1/24".to_string(),
                listen_port: 51820,
            },
            peers: vec![PeerConfig {
                public_key: "peer_key".to_string(),
                allowed_ips: "10.8.0.2/32".to_string(),
                endpoint: None,
                psk: None,
            }],
        };

        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("wg0"));
        assert!(toml_str.contains("51820"));
    }
}
