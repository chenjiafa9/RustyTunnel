use crate::config::ServerConfig;
use crate::crypto;
use crate::device::TunDevice;
use crate::error::Result;
use crate::peer::{Peer, PeerStatus};
use log::{info, warn};
use std::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::RwLock;

/// VPN 服务器
pub struct VpnServer {
    /// 服务器配置
    config: ServerConfig,
    /// 对等体列表
    peers: Arc<RwLock<Vec<Peer>>>,
    /// TUN 设备
    device: TunDevice,
    /// UDP 套接字
    socket: Option<UdpSocket>,
}

impl VpnServer {
    /// 创建新的 VPN 服务器
    pub fn new(config: ServerConfig) -> Result<Self> {
        let device = TunDevice::new(&config.interface.name, &config.interface.address);

        // 从配置创建对等体
        let mut peers = Vec::new();
        for peer_config in &config.peers {
            let peer = Peer::from_config(peer_config.clone())?;
            info!("Loaded peer: {}", peer.summary());
            peers.push(peer);
        }

        Ok(VpnServer {
            config,
            peers: Arc::new(RwLock::new(peers)),
            device,
            socket: None,
        })
    }

    /// 启动服务器
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting VPN server on port {}", self.config.interface.listen_port);

        // 配置 TUN 设备
        self.setup_device().await?;

        // 绑定 UDP 套接字
        let addr = format!("0.0.0.0:{}", self.config.interface.listen_port);
        let socket = UdpSocket::bind(&addr)
            .map_err(|e| crate::error::Error::NetworkError(format!("Failed to bind socket: {}", e)))?;
        socket.set_nonblocking(true)
            .map_err(|e| crate::error::Error::NetworkError(format!("Failed to set nonblocking: {}", e)))?;

        self.socket = Some(socket);
        info!("VPN server started successfully");

        Ok(())
    }

    /// 停止服务器
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping VPN server");

        // 清理设备
        self.cleanup_device().await?;

        info!("VPN server stopped");
        Ok(())
    }

    /// 设置 TUN 设备
    async fn setup_device(&self) -> Result<()> {
        info!("Setting up TUN device: {}", self.device.name);

        // 配置 IP 地址
        self.device.set_address()?;

        // 启用设备
        self.device.up()?;

        // 启用 IP 转发
        TunDevice::enable_forwarding()?;

        // 添加路由
        for peer in self.peers.read().await.iter() {
            self.device.add_route(&peer.allowed_ips)?;
        }

        info!("TUN device configured successfully");
        Ok(())
    }

    /// 清理 TUN 设备
    async fn cleanup_device(&self) -> Result<()> {
        info!("Cleaning up TUN device");

        // 删除路由
        for peer in self.peers.read().await.iter() {
            if let Err(e) = self.device.remove_route(&peer.allowed_ips) {
                warn!("Failed to remove route: {}", e);
            }
        }

        // 禁用设备
        if let Err(e) = self.device.down() {
            warn!("Failed to disable device: {}", e);
        }

        // 删除 IP 地址
        if let Err(e) = self.device.remove_address() {
            warn!("Failed to remove address: {}", e);
        }

        info!("TUN device cleanup completed");
        Ok(())
    }

    /// 获取对等体列表
    pub async fn get_peers(&self) -> Vec<Peer> {
        self.peers.read().await.clone()
    }

    /// 更新对等体状态
    pub async fn update_peer_status(&self, public_key: &str, status: PeerStatus) -> Result<()> {
        let mut peers = self.peers.write().await;
        for peer in peers.iter_mut() {
            if peer.public_key == public_key {
                peer.set_status(status);
                info!("Updated peer status: {} -> {:?}", &public_key[..8], status);
                return Ok(());
            }
        }
        Err(crate::error::Error::Other(format!(
            "Peer not found: {}",
            public_key
        )))
    }

    /// 获取服务器统计信息
    pub async fn get_stats(&self) -> ServerStats {
        let peers = self.peers.read().await;
        let total_bytes_received: u64 = peers.iter().map(|p| p.bytes_received).sum();
        let total_bytes_sent: u64 = peers.iter().map(|p| p.bytes_sent).sum();
        let connected_peers = peers.iter().filter(|p| p.status == PeerStatus::Connected).count();

        ServerStats {
            total_peers: peers.len(),
            connected_peers,
            total_bytes_received,
            total_bytes_sent,
        }
    }
}

/// 服务器统计信息
#[derive(Debug, Clone)]
pub struct ServerStats {
    pub total_peers: usize,
    pub connected_peers: usize,
    pub total_bytes_received: u64,
    pub total_bytes_sent: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{InterfaceConfig, PeerConfig};

    #[tokio::test]
    async fn test_server_creation() {
        let config = ServerConfig {
            interface: InterfaceConfig {
                name: "wg0".to_string(),
                private_key: "test_key".to_string(),
                address: "10.8.0.1/24".to_string(),
                listen_port: 51820,
            },
            peers: vec![],
        };

        let server = VpnServer::new(config).unwrap();
        assert_eq!(server.device.name, "wg0");
    }
}
