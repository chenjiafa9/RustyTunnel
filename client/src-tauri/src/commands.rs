use crate::error::{ApiError, Result};
use crate::models::*;
use crate::state::AppState;
use tauri::State;
use uuid::Uuid;

/// 用户登录
#[tauri::command]
pub async fn login(
    username: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<LoginResponse> {
    // 模拟认证 - 实际应用中应连接真实后端
    if username.is_empty() || password.is_empty() {
        return Err(ApiError::bad_request("Username and password required"));
    }

    // 简单验证示例
    if username == "demo" && password == "demo" {
        let user = UserInfo {
            id: Uuid::new_v4().to_string(),
            username: username.clone(),
            email: "demo@example.com".to_string(),
        };

        let token = format!("token_{}", Uuid::new_v4());
        state.set_user(user.clone());
        state.set_token(token.clone());

        Ok(LoginResponse { token, user })
    } else {
        Err(ApiError::invalid_credentials())
    }
}

/// 用户登出
#[tauri::command]
pub fn logout(state: State<'_, AppState>) -> Result<()> {
    state.clear_auth();
    Ok(())
}

/// 获取当前用户信息
#[tauri::command]
pub fn get_current_user(state: State<'_, AppState>) -> Result<UserInfo> {
    state
        .get_user()
        .ok_or_else(|| ApiError::new(401, "Not authenticated"))
}

/// 获取 VPN 节点列表
#[tauri::command]
pub fn get_vpn_nodes() -> Result<Vec<VpnNode>> {
    Ok(vec![
        VpnNode {
            id: "us-1".to_string(),
            name: "New York".to_string(),
            country: "United States".to_string(),
            city: "New York".to_string(),
            protocol: "WireGuard".to_string(),
            endpoint: "us1.example.com".to_string(),
            port: 51820,
            public_key: "public_key_us1".to_string(),
            ping: Some(45),
            load: Some(35.5),
        },
        VpnNode {
            id: "uk-1".to_string(),
            name: "London".to_string(),
            country: "United Kingdom".to_string(),
            city: "London".to_string(),
            protocol: "WireGuard".to_string(),
            endpoint: "uk1.example.com".to_string(),
            port: 51820,
            public_key: "public_key_uk1".to_string(),
            ping: Some(25),
            load: Some(42.0),
        },
        VpnNode {
            id: "jp-1".to_string(),
            name: "Tokyo".to_string(),
            country: "Japan".to_string(),
            city: "Tokyo".to_string(),
            protocol: "WireGuard".to_string(),
            endpoint: "jp1.example.com".to_string(),
            port: 51820,
            public_key: "public_key_jp1".to_string(),
            ping: Some(120),
            load: Some(28.0),
        },
        VpnNode {
            id: "sg-1".to_string(),
            name: "Singapore".to_string(),
            country: "Singapore".to_string(),
            city: "Singapore".to_string(),
            protocol: "WireGuard".to_string(),
            endpoint: "sg1.example.com".to_string(),
            port: 51820,
            public_key: "public_key_sg1".to_string(),
            ping: Some(80),
            load: Some(55.0),
        },
    ])
}

/// 获取连接统计信息
#[tauri::command]
pub fn get_connection_stats(state: State<'_, AppState>) -> Result<ConnectionStats> {
    Ok(state.get_stats())
}

/// 更新连接统计信息
#[tauri::command]
pub fn update_connection_stats(
    stats: ConnectionStats,
    state: State<'_, AppState>,
) -> Result<()> {
    state.update_stats(stats);
    Ok(())
}

/// 启动连接
#[tauri::command]
pub async fn start_connection(
    node_id: String,
    state: State<'_, AppState>,
) -> Result<ConnectionStats> {
    let mut stats = state.get_stats();
    stats.status = crate::models::ConnectionStatus::Connecting;
    stats.connected_node = Some(node_id);
    state.update_stats(stats.clone());

    // 模拟连接延迟
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    stats.status = crate::models::ConnectionStatus::Connected;
    stats.ip_address = Some("10.8.0.2".to_string());
    state.update_stats(stats.clone());

    Ok(stats)
}

/// 停止连接
#[tauri::command]
pub async fn stop_connection(state: State<'_, AppState>) -> Result<ConnectionStats> {
    let mut stats = state.get_stats();
    stats.status = crate::models::ConnectionStatus::Disconnecting;
    state.update_stats(stats.clone());

    // 模拟断开延迟
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    stats.status = crate::models::ConnectionStatus::Disconnected;
    stats.connected_node = None;
    stats.ip_address = None;
    stats.upload_speed = 0;
    stats.download_speed = 0;
    state.update_stats(stats.clone());

    Ok(stats)
}

/// 获取应用设置
#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<AppSettings> {
    Ok(state.settings.lock().unwrap().clone())
}

/// 更新应用设置
#[tauri::command]
pub fn update_settings(
    settings: AppSettings,
    state: State<'_, AppState>,
) -> Result<AppSettings> {
    *state.settings.lock().unwrap() = settings.clone();
    Ok(settings)
}

/// 测试连接到节点
#[tauri::command]
pub async fn test_node_connection(node_id: String) -> Result<u32> {
    // 模拟 ping 测试
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    Ok(rand::random::<u32>() % 200)
}
