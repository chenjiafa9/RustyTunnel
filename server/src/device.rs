use crate::error::{Error, Result};
use std::process::Command;

/// TUN 设备管理器
pub struct TunDevice {
    /// 设备名称
    pub name: String,
    /// 设备地址
    pub address: String,
}

impl TunDevice {
    /// 创建新的 TUN 设备
    pub fn new(name: &str, address: &str) -> Self {
        TunDevice {
            name: name.to_string(),
            address: address.to_string(),
        }
    }

    /// 启用设备
    pub fn up(&self) -> Result<()> {
        self.run_command(&["ip", "link", "set", "dev", &self.name, "up"])?;
        Ok(())
    }

    /// 禁用设备
    pub fn down(&self) -> Result<()> {
        self.run_command(&["ip", "link", "set", "dev", &self.name, "down"])?;
        Ok(())
    }

    /// 配置 IP 地址
    pub fn set_address(&self) -> Result<()> {
        self.run_command(&["ip", "addr", "add", &self.address, "dev", &self.name])?;
        Ok(())
    }

    /// 删除 IP 地址
    pub fn remove_address(&self) -> Result<()> {
        self.run_command(&["ip", "addr", "del", &self.address, "dev", &self.name])?;
        Ok(())
    }

    /// 添加路由
    pub fn add_route(&self, route: &str) -> Result<()> {
        self.run_command(&["ip", "route", "add", route, "dev", &self.name])?;
        Ok(())
    }

    /// 删除路由
    pub fn remove_route(&self, route: &str) -> Result<()> {
        self.run_command(&["ip", "route", "del", route, "dev", &self.name])?;
        Ok(())
    }

    /// 启用 IP 转发
    pub fn enable_forwarding() -> Result<()> {
        Self::run_sysctl("net.ipv4.ip_forward", "1")?;
        Ok(())
    }

    /// 禁用 IP 转发
    pub fn disable_forwarding() -> Result<()> {
        Self::run_sysctl("net.ipv4.ip_forward", "0")?;
        Ok(())
    }

    /// 运行系统命令
    fn run_command(&self, args: &[&str]) -> Result<()> {
        let output = Command::new(args[0])
            .args(&args[1..])
            .output()
            .map_err(|e| Error::DeviceError(format!("Failed to run command: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::DeviceError(format!(
                "Command failed: {}",
                stderr
            )));
        }

        Ok(())
    }

    /// 运行 sysctl 命令
    fn run_sysctl(key: &str, value: &str) -> Result<()> {
        let output = Command::new("sysctl")
            .args(&["-w", &format!("{}={}", key, value)])
            .output()
            .map_err(|e| Error::DeviceError(format!("Failed to run sysctl: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::DeviceError(format!(
                "sysctl failed: {}",
                stderr
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tun_device_creation() {
        let device = TunDevice::new("wg0", "10.8.0.1/24");
        assert_eq!(device.name, "wg0");
        assert_eq!(device.address, "10.8.0.1/24");
    }
}
