import { invoke } from '@tauri-apps/api/tauri'

export const api = {
  // Auth
  login: (username, password) => invoke('login', { username, password }),
  logout: () => invoke('logout'),
  getCurrentUser: () => invoke('get_current_user'),

  // VPN Nodes
  getVpnNodes: () => invoke('get_vpn_nodes'),
  testNodeConnection: (nodeId) => invoke('test_node_connection', { nodeId }),

  // Connection
  getConnectionStats: () => invoke('get_connection_stats'),
  updateConnectionStats: (stats) => invoke('update_connection_stats', { stats }),
  startConnection: (nodeId) => invoke('start_connection', { nodeId }),
  stopConnection: () => invoke('stop_connection'),

  // Settings
  getSettings: () => invoke('get_settings'),
  updateSettings: (settings) => invoke('update_settings', { settings }),
}
