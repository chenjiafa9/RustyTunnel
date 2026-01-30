# RustyTunnel - 高性能 WireGuard VPN 系统

一个使用 Rust 开发的完整 VPN 解决方案，包含高性能服务器端和跨平台桌面客户端。支持 WireGuard 协议、多用户管理、安全加密和灵活的配置。

## 项目特性

### 服务器端
- ✅ **WireGuard 协议** - 基于 boringtun 的用户空间实现
- ✅ **多用户支持** - 支持多个客户端同时连接
- ✅ **高性能** - 异步 I/O，支持数百 Mbps 吞吐量
- ✅ **灵活配置** - TOML 格式配置文件，易于管理
- ✅ **自动密钥生成** - 内置密钥对生成工具
- ✅ **系统集成** - systemd 服务支持
- ✅ **跨平台** - Linux 原生支持

### 客户端
- ✅ **现代 UI** - React + Tailwind CSS 深色科技风格
- ✅ **用户认证** - 安全的登录系统
- ✅ **服务器管理** - 添加、编辑、删除 VPN 配置
- ✅ **连接控制** - 一键连接/断开
- ✅ **实时统计** - 流量监控和连接状态
- ✅ **跨平台** - Windows 10/11 支持（Tauri）

## 快速开始

### 前置要求

#### 服务器端
- Linux (Ubuntu 20.04 LTS 或更新)
- Rust 1.93.0+
- build-essential, pkg-config, libssl-dev

#### 客户端
- Node.js 22.13.0+
- pnpm 10.4.1+

### 编译

```bash
# 服务器端
cd server
cargo build --release

# 客户端
cd ../client
pnpm install
pnpm build
```

### 运行

#### 服务器

```bash
# 生成密钥
./target/release/rusty-tunnel-server keygen --count 2

# 生成配置文件
./target/release/rusty-tunnel-server gen-config --output server.toml

# 启动服务器
sudo ./target/release/rusty-tunnel-server server --config server.toml
```

#### 客户端

```bash
# 开发模式
pnpm dev

# 生产构建
pnpm build
```

## 配置示例

### 服务器配置

```toml
[interface]
name = "wg0"
private_key = "YOUR_SERVER_PRIVATE_KEY"
address = "10.8.0.1/24"
listen_port = 51820

[[peers]]
public_key = "CLIENT1_PUBLIC_KEY"
allowed_ips = "10.8.0.2/32"
```

## 技术栈

### 服务器
- Rust 1.93.0
- boringtun 0.6.0 (WireGuard)
- tokio 1.36.0 (异步运行时)
- x25519-dalek (密钥交换)

### 客户端
- React 19.2
- TypeScript 5.6
- Tailwind CSS 4.1
- shadcn/ui

## 部署

详见 [DEPLOYMENT.md](./DEPLOYMENT.md)

## 许可证

MIT License

## 更新日志

### v1.0.0 (2026-01-30)
- 🎉 初始版本发布
- ✅ 完整的服务器端实现
- ✅ 现代化的客户端 UI
- ✅ 多用户支持
- ✅ 完整的部署文档
