use crate::config::PeerConfig;
use crate::error::Result;
use std::net::{IpAddr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};

/// 对等体状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerStatus {
    /// 未连接
    Disconnected,
    /// 握手中
    Handshaking,
    /// 已连接
    Connected,
}

/// 对等体信息
#[derive(Debug, Clone)]
pub struct Peer {
    /// 公钥
    pub public_key: String,
    /// 允许的 IP 地址
    pub allowed_ips: String,
    /// 对等体端点
    pub endpoint: Option<SocketAddr>,
    /// 预共享密钥
    pub psk: Option<String>,
    /// 状态
    pub status: PeerStatus,
    /// 最后握手时间戳
    pub last_handshake: u64,
    /// 接收字节数
    pub bytes_received: u64,
    /// 发送字节数
    pub bytes_sent: u64,
}

impl Peer {
    /// 从配置创建对等体
    pub fn from_config(config: PeerConfig) -> Result<Self> {
        let endpoint = config.endpoint.and_then(|ep| ep.parse::<SocketAddr>().ok());

        Ok(Peer {
            public_key: config.public_key,
            allowed_ips: config.allowed_ips,
            endpoint,
            psk: config.psk,
            status: PeerStatus::Disconnected,
            last_handshake: 0,
            bytes_received: 0,
            bytes_sent: 0,
        })
    }

    /// 更新对等体状态
    pub fn set_status(&mut self, status: PeerStatus) {
        self.status = status;
        if status == PeerStatus::Connected {
            self.last_handshake = current_timestamp();
        }
    }

    /// 获取对等体信息摘要
    pub fn summary(&self) -> String {
        format!(
            "Peer {{ key: {}, ips: {}, status: {:?}, rx: {} bytes, tx: {} bytes }}",
            &self.public_key[..8],
            self.allowed_ips,
            self.status,
            self.bytes_received,
            self.bytes_sent
        )
    }
}

/// 获取当前时间戳（秒）
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_creation() {
        let config = PeerConfig {
            public_key: "test_key".to_string(),
            allowed_ips: "10.8.0.2/32".to_string(),
            endpoint: None,
            psk: None,
        };

        let peer = Peer::from_config(config).unwrap();
        assert_eq!(peer.status, PeerStatus::Disconnected);
        assert_eq!(peer.bytes_received, 0);
        assert_eq!(peer.bytes_sent, 0);
    }

    #[test]
    fn test_peer_status_update() {
        let config = PeerConfig {
            public_key: "test_key".to_string(),
            allowed_ips: "10.8.0.2/32".to_string(),
            endpoint: None,
            psk: None,
        };

        let mut peer = Peer::from_config(config).unwrap();
        peer.set_status(PeerStatus::Connected);
        assert_eq!(peer.status, PeerStatus::Connected);
        assert!(peer.last_handshake > 0);
    }
}
