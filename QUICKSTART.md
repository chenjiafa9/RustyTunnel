# RustyTunnel 快速开始指南

## 5 分钟快速上手

### 第一步：准备环境

#### Linux 服务器
```bash
# 更新系统
sudo apt-get update && sudo apt-get upgrade -y

# 安装依赖
sudo apt-get install -y build-essential pkg-config libssl-dev
```

#### 客户端机器
- Windows 10/11 或 Linux
- 任何现代浏览器

### 第二步：编译服务器

```bash
cd /home/ubuntu/RustyTunnel/server

# 编译 Release 版本（优化性能）
cargo build --release

# 编译完成后，二进制文件位于：
# target/release/rusty-tunnel-server
```

### 第三步：生成密钥和配置

```bash
cd /home/ubuntu/RustyTunnel

# 生成 2 个客户端密钥对
./target/release/rusty-tunnel-server keygen --count 2

# 输出示例：
# [Keypair 1]
# PrivateKey = "UTrpNihvTOuwYeY0wSflfWTqdnzV1HXH1+IYCayYKgU="
# PublicKey = "eaVlTxKjEmAVb2wqMywYuu4V573TvghGKHf4tdBICjg="
#
# [Keypair 2]
# PrivateKey = "fj6YnNwHdIDneONs43XT4HQhJxEkiGcJTwNdUY90q9c="
# PublicKey = "sodZ5+6lcJrt0IEQM500FOLwK2HVyf+EzZonI9xEwwU="

# 保存这些密钥对，稍后会用到

# 生成服务器配置文件
./target/release/rusty-tunnel-server gen-config --output server.toml
```

### 第四步：配置服务器

编辑 `server.toml` 文件，将客户端公钥添加进去：

```bash
nano server.toml
```

编辑后的内容应该类似于：

```toml
[interface]
name = "wg0"
private_key = "M1g8Q4PUQ+dPGXgypt+NpI3OK3MTHSi6/zdKFuv6c/U="
address = "10.8.0.1/24"
listen_port = 51820

[[peers]]
public_key = "eaVlTxKjEmAVb2wqMywYuu4V573TvghGKHf4tdBICjg="
allowed_ips = "10.8.0.2/32"

[[peers]]
public_key = "sodZ5+6lcJrt0IEQM500FOLwK2HVyf+EzZonI9xEwwU="
allowed_ips = "10.8.0.3/32"
```

### 第五步：启动服务器

```bash
# 启用 IP 转发
sudo sysctl -w net.ipv4.ip_forward=1

# 允许防火墙通过
sudo ufw allow 51820/udp

# 启动 VPN 服务器
sudo ./target/release/rusty-tunnel-server server --config server.toml

# 输出应该显示：
# [INFO] Starting VPN server on port 51820
# [INFO] Setting up TUN device: wg0
# [INFO] TUN device configured successfully
# [INFO] VPN server started successfully
```

### 第六步：使用客户端

#### 方式 1：Web 客户端

```bash
cd /home/ubuntu/rusty-tunnel-client-gui/client

# 开发模式
pnpm dev

# 访问 http://localhost:3000
```

#### 方式 2：生产构建

```bash
# 构建
pnpm build

# 预览
pnpm preview
```

### 第七步：客户端配置

1. 打开客户端 Web 界面
2. 点击"登录"（演示模式，任意邮箱和密码）
3. 点击"服务器"标签
4. 点击"添加服务器"按钮
5. 填入以下信息：
   - **服务器名称**: My VPN
   - **服务器地址**: your.server.com（或服务器 IP）
   - **端口**: 51820
   - **服务器公钥**: 从 server.toml 中的 private_key 对应的公钥
6. 点击"保存"
7. 回到"连接"标签
8. 点击大圆形按钮连接

## 常见问题

### Q: 如何获取服务器公钥？

从 server.toml 的 private_key 计算：

```bash
# 使用以下命令生成公钥
echo "YOUR_PRIVATE_KEY" | base64 -d | xxd -p

# 或使用在线工具转换
```

### Q: 连接失败怎么办？

1. 检查防火墙：`sudo ufw status`
2. 检查端口：`sudo netstat -ulnp | grep 51820`
3. 检查 IP 转发：`cat /proc/sys/net/ipv4/ip_forward`
4. 查看日志：`sudo journalctl -xe`

### Q: 如何停止服务器？

按 `Ctrl+C` 停止服务器进程

### Q: 如何卸载？

```bash
# 停止服务
sudo systemctl stop rusty-tunnel

# 删除文件
sudo rm -rf /opt/rusty-tunnel
```

## 下一步

- 查看 [DEPLOYMENT.md](./DEPLOYMENT.md) 了解生产部署
- 查看 [README.md](./README.md) 了解完整功能
- 查看源代码了解实现细节

## 获取帮助

- 检查日志：`RUST_LOG=debug ./rusty-tunnel-server server --config server.toml`
- 监控网络：`sudo tcpdump -i eth0 -n udp port 51820`
- 查看文档：`./rusty-tunnel-server --help`

## 性能测试

连接后，可以测试 VPN 连接质量：

```bash
# 在客户端测试
ping 8.8.8.8

# 测试速度
iperf3 -c server_ip
```

## 安全建议

1. **定期更新** - 保持系统和依赖最新
2. **强密码** - 使用强密码保护客户端
3. **防火墙** - 只开放必要的端口
4. **备份** - 定期备份配置文件
5. **监控** - 监控异常连接

## 故障排查

### 服务器无法启动

```bash
# 检查权限
sudo -l

# 检查端口占用
sudo lsof -i :51820

# 检查配置文件
cat server.toml
```

### 客户端无法连接

```bash
# 检查网络连接
ping your.server.com

# 检查防火墙
sudo ufw status

# 检查路由
ip route show
```

### 流量无法转发

```bash
# 启用 IP 转发
sudo sysctl -w net.ipv4.ip_forward=1

# 检查 iptables
sudo iptables -L -n

# 添加 NAT 规则
sudo iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
```

## 更多信息

- 官方文档: [DEPLOYMENT.md](./DEPLOYMENT.md)
- 项目主页: https://github.com/chenjiafa9/RustyTunnel
- 问题报告: https://github.com/chenjiafa9/RustyTunnel/issues

祝您使用愉快！🚀
