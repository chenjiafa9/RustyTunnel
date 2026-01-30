# RustyTunnel 部署指南

## 系统要求

### 服务器端
- **操作系统**: Linux (Ubuntu 20.04 LTS 或更新版本)
- **权限**: root 或 sudo 权限
- **网络**: 公网 IP 地址，UDP 端口 51820 开放
- **依赖**: libssl-dev, build-essential

### 客户端
- **操作系统**: Windows 10/11
- **架构**: x86_64
- **.NET Runtime**: 可选（如果使用 .NET 版本）

## 服务器部署步骤

### 1. 安装依赖

```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev
```

### 2. 编译服务器

```bash
cd /home/ubuntu/RustyTunnel
cargo build --release
```

编译后的二进制文件位置: `target/release/rusty-tunnel-server`

### 3. 生成密钥和配置

#### 生成密钥对
```bash
./target/release/rusty-tunnel-server keygen --count 3
```

输出示例:
```
[Keypair 1]
PrivateKey = "UTrpNihvTOuwYeY0wSflfWTqdnzV1HXH1+IYCayYKgU="
PublicKey = "eaVlTxKjEmAVb2wqMywYuu4V573TvghGKHf4tdBICjg="
```

#### 生成配置文件
```bash
./target/release/rusty-tunnel-server gen-config --output server.toml
```

### 4. 配置服务器

编辑 `server.toml` 文件，添加客户端信息:

```toml
[interface]
name = "wg0"
private_key = "YOUR_SERVER_PRIVATE_KEY"
address = "10.8.0.1/24"
listen_port = 51820

[[peers]]
public_key = "CLIENT1_PUBLIC_KEY"
allowed_ips = "10.8.0.2/32"

[[peers]]
public_key = "CLIENT2_PUBLIC_KEY"
allowed_ips = "10.8.0.3/32"
```

### 5. 启动服务器

```bash
sudo ./target/release/rusty-tunnel-server server --config server.toml
```

### 6. 防火墙配置

```bash
# 允许 UDP 51820 端口
sudo ufw allow 51820/udp

# 启用 IP 转发
sudo sysctl -w net.ipv4.ip_forward=1

# 永久启用 IP 转发
echo "net.ipv4.ip_forward=1" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

### 7. 系统服务配置（可选）

创建 systemd 服务文件 `/etc/systemd/system/rusty-tunnel.service`:

```ini
[Unit]
Description=RustyTunnel VPN Server
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/rusty-tunnel
ExecStart=/opt/rusty-tunnel/rusty-tunnel-server server --config /opt/rusty-tunnel/server.toml
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

启用服务:
```bash
sudo systemctl enable rusty-tunnel
sudo systemctl start rusty-tunnel
sudo systemctl status rusty-tunnel
```

## 客户端配置

### 1. 客户端配置文件格式

创建 `client.toml`:

```toml
private_key = "YOUR_CLIENT_PRIVATE_KEY"
address = "10.8.0.2/24"
server_public_key = "SERVER_PUBLIC_KEY"
server_endpoint = "your.server.com:51820"
dns = ["8.8.8.8", "8.8.4.4"]
```

### 2. 启动客户端

```bash
./target/release/rusty-tunnel-client client --config client.toml
```

## 故障排查

### 检查服务器状态

```bash
# 查看 WireGuard 接口
sudo ip link show wg0
sudo ip addr show wg0

# 查看路由
sudo ip route show

# 检查监听端口
sudo netstat -ulnp | grep 51820

# 查看日志
sudo journalctl -u rusty-tunnel -f
```

### 常见问题

#### 1. 权限错误
确保使用 `sudo` 运行服务器

#### 2. 端口被占用
```bash
sudo lsof -i :51820
```

#### 3. 握手失败
- 检查公钥是否正确
- 确保客户端能连接到服务器 IP:端口
- 检查防火墙规则

#### 4. 无法转发流量
- 确保 IP 转发已启用: `cat /proc/sys/net/ipv4/ip_forward`
- 检查 iptables 规则

## 性能优化

### 调整 MTU
```bash
sudo ip link set dev wg0 mtu 1420
```

### 启用 UDP GRO（如果支持）
```bash
sudo ethtool -K eth0 rx-udp-gro-forwarding on
```

## 安全建议

1. **定期更新**: 保持系统和依赖项最新
2. **防火墙**: 只开放必要的端口
3. **密钥管理**: 安全存储私钥，定期轮换
4. **日志监控**: 监控异常连接尝试
5. **备份**: 定期备份配置文件

## 卸载

```bash
# 停止服务
sudo systemctl stop rusty-tunnel
sudo systemctl disable rusty-tunnel

# 清理设备
sudo ip link del dev wg0

# 移除文件
sudo rm -rf /opt/rusty-tunnel
sudo rm /etc/systemd/system/rusty-tunnel.service
sudo systemctl daemon-reload
```

## 技术支持

如遇问题，请检查以下内容:
1. 系统日志: `sudo journalctl -xe`
2. 应用日志: 启用 `RUST_LOG=debug` 环境变量
3. 网络连接: 使用 `tcpdump` 监控 UDP 流量

```bash
# 启用调试日志
RUST_LOG=debug sudo ./target/release/rusty-tunnel-server server --config server.toml

# 监控网络流量
sudo tcpdump -i eth0 -n udp port 51820
```
