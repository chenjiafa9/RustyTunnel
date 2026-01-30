use clap::{Parser, Subcommand};
use log::info;
use rusty_tunnel_server::{config::ServerConfig, crypto, error::Result, server::VpnServer};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "RustyTunnel Server")]
#[command(about = "A high-performance WireGuard VPN server written in Rust")]
#[command(version = "1.0.0")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 启动 VPN 服务器
    Server {
        /// 配置文件路径
        #[arg(short, long, default_value = "server.toml")]
        config: PathBuf,
    },

    /// 生成密钥对
    Keygen {
        /// 生成的密钥对数量
        #[arg(short, long, default_value = "1")]
        count: usize,
    },

    /// 生成示例配置文件
    GenConfig {
        /// 输出文件路径
        #[arg(short, long, default_value = "server.toml")]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();

    match args.command {
        Commands::Server { config } => {
            run_server(config).await?;
        }
        Commands::Keygen { count } => {
            generate_keys(count)?;
        }
        Commands::GenConfig { output } => {
            generate_config(output)?;
        }
    }

    Ok(())
}

/// 运行服务器
async fn run_server(config_path: PathBuf) -> Result<()> {
    info!("Loading configuration from: {:?}", config_path);

    let config = ServerConfig::from_file(&config_path)?;
    info!("Configuration loaded successfully");

    let mut server = VpnServer::new(config)?;

    // 设置 Ctrl+C 处理
    let server_handle = tokio::spawn(async move {
        match server.start().await {
            Ok(_) => {
                // 保持运行直到收到信号
                tokio::signal::ctrl_c()
                    .await
                    .expect("Failed to listen for Ctrl+C");
                info!("Received Ctrl+C, shutting down...");
                if let Err(e) = server.stop().await {
                    eprintln!("Error stopping server: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to start server: {}", e);
            }
        }
    });

    server_handle.await.ok();
    Ok(())
}

/// 生成密钥对
fn generate_keys(count: usize) -> Result<()> {
    info!("Generating {} keypair(s)...", count);

    for i in 1..=count {
        let (priv_key, pub_key) = crypto::generate_keypair()?;
        println!("\n[Keypair {}]", i);
        println!("PrivateKey = \"{}\"", priv_key);
        println!("PublicKey = \"{}\"", pub_key);
    }

    Ok(())
}

/// 生成示例配置文件
fn generate_config(output: PathBuf) -> Result<()> {
    let (priv_key, _) = crypto::generate_keypair()?;

    let config_content = format!(
        r#"# RustyTunnel Server Configuration

[interface]
name = "wg0"
private_key = "{}"
address = "10.8.0.1/24"
listen_port = 51820

# Example peer configuration
[[peers]]
public_key = "REPLACE_WITH_CLIENT_PUBLIC_KEY"
allowed_ips = "10.8.0.2/32"
endpoint = "client.example.com:51820"  # Optional

# Add more peers as needed
# [[peers]]
# public_key = "ANOTHER_CLIENT_PUBLIC_KEY"
# allowed_ips = "10.8.0.3/32"
"#,
        priv_key
    );

    std::fs::write(&output, config_content)
        .map_err(|e| rusty_tunnel_server::error::Error::ConfigError(format!(
            "Failed to write config file: {}",
            e
        )))?;

    info!("Configuration file generated: {:?}", output);
    println!("Configuration file created at: {:?}", output);
    println!("Please edit it and add your peer configurations.");

    Ok(())
}
