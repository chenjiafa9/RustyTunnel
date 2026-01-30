import React, { useState, useEffect } from 'react'
import { Power, Settings, LogOut, Zap, Download, Upload, Clock } from 'lucide-react'
import { api } from '../api'
import NodeSelector from '../components/NodeSelector'
import StatsDisplay from '../components/StatsDisplay'
import SettingsPanel from '../components/SettingsPanel'

export default function DashboardPage({ user, onLogout }) {
  const [isConnected, setIsConnected] = useState(false)
  const [selectedNode, setSelectedNode] = useState(null)
  const [stats, setStats] = useState(null)
  const [showSettings, setShowSettings] = useState(false)
  const [loading, setLoading] = useState(false)
  const [nodes, setNodes] = useState([])

  useEffect(() => {
    loadNodes()
    loadStats()
    const interval = setInterval(loadStats, 1000)
    return () => clearInterval(interval)
  }, [])

  const loadNodes = async () => {
    try {
      const nodes = await api.getVpnNodes()
      setNodes(nodes)
      if (nodes.length > 0) {
        setSelectedNode(nodes[0])
      }
    } catch (err) {
      console.error('Failed to load nodes:', err)
    }
  }

  const loadStats = async () => {
    try {
      const stats = await api.getConnectionStats()
      setStats(stats)
    } catch (err) {
      console.error('Failed to load stats:', err)
    }
  }

  const handleConnect = async () => {
    if (!selectedNode) return
    setLoading(true)
    try {
      const result = await api.startConnection(selectedNode.id)
      setStats(result)
      setIsConnected(result.status === 'connected')
    } catch (err) {
      console.error('Connection failed:', err)
    } finally {
      setLoading(false)
    }
  }

  const handleDisconnect = async () => {
    setLoading(true)
    try {
      const result = await api.stopConnection()
      setStats(result)
      setIsConnected(false)
    } catch (err) {
      console.error('Disconnection failed:', err)
    } finally {
      setLoading(false)
    }
  }

  const formatSpeed = (bytes) => {
    if (bytes === 0) return '0 B/s'
    const k = 1024
    const sizes = ['B/s', 'KB/s', 'MB/s', 'GB/s']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
  }

  const formatBytes = (bytes) => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
  }

  const formatTime = (seconds) => {
    const hours = Math.floor(seconds / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)
    const secs = seconds % 60
    return `${hours}h ${minutes}m ${secs}s`
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
      {/* Header */}
      <div className="bg-slate-800/50 border-b border-slate-700 backdrop-blur">
        <div className="max-w-6xl mx-auto px-6 py-4 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold text-white">RustyTunnel VPN</h1>
            <p className="text-slate-400 text-sm">Welcome, {user?.username}</p>
          </div>
          <div className="flex gap-3">
            <button
              onClick={() => setShowSettings(true)}
              className="btn-secondary flex items-center gap-2"
            >
              <Settings className="w-5 h-5" />
              Settings
            </button>
            <button
              onClick={onLogout}
              className="btn-secondary flex items-center gap-2"
            >
              <LogOut className="w-5 h-5" />
              Logout
            </button>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-6xl mx-auto px-6 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Left Column - Connection Control */}
          <div className="lg:col-span-2">
            <div className="card">
              <div className="flex items-center justify-between mb-6">
                <h2 className="text-xl font-bold text-white">Connection</h2>
                <div className={`badge ${isConnected ? 'badge-success' : 'badge-error'}`}>
                  {isConnected ? 'Connected' : 'Disconnected'}
                </div>
              </div>

              {/* Connection Button */}
              <div className="flex justify-center mb-8">
                <button
                  onClick={isConnected ? handleDisconnect : handleConnect}
                  disabled={loading}
                  className={`w-32 h-32 rounded-full flex items-center justify-center text-white font-bold text-lg transition-all duration-300 ${
                    isConnected
                      ? 'bg-red-600 hover:bg-red-700'
                      : 'bg-green-600 hover:bg-green-700'
                  } disabled:opacity-50`}
                >
                  <Power className="w-12 h-12" />
                </button>
              </div>

              {/* Node Selector */}
              {!isConnected && (
                <div className="mb-6">
                  <h3 className="text-lg font-semibold text-white mb-4">Select Server</h3>
                  <NodeSelector
                    nodes={nodes}
                    selectedNode={selectedNode}
                    onSelectNode={setSelectedNode}
                  />
                </div>
              )}

              {/* Connected Node Info */}
              {isConnected && selectedNode && (
                <div className="bg-slate-700/50 rounded-lg p-4 mb-6">
                  <p className="text-slate-300 text-sm mb-2">Connected to:</p>
                  <p className="text-white font-semibold text-lg">
                    {selectedNode.name} ({selectedNode.country})
                  </p>
                  <p className="text-slate-400 text-sm mt-1">
                    {selectedNode.endpoint}:{selectedNode.port}
                  </p>
                </div>
              )}

              {/* IP Address */}
              {stats?.ip_address && (
                <div className="bg-slate-700/50 rounded-lg p-4">
                  <p className="text-slate-300 text-sm mb-1">Your IP Address</p>
                  <p className="text-white font-mono text-lg">{stats.ip_address}</p>
                </div>
              )}
            </div>
          </div>

          {/* Right Column - Stats */}
          <div className="space-y-6">
            {/* Speed Stats */}
            <div className="card">
              <h3 className="text-lg font-bold text-white mb-4">Speed</h3>
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2 text-slate-300">
                    <Download className="w-5 h-5 text-green-400" />
                    Download
                  </div>
                  <span className="text-white font-semibold">
                    {formatSpeed(stats?.download_speed || 0)}
                  </span>
                </div>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2 text-slate-300">
                    <Upload className="w-5 h-5 text-blue-400" />
                    Upload
                  </div>
                  <span className="text-white font-semibold">
                    {formatSpeed(stats?.upload_speed || 0)}
                  </span>
                </div>
              </div>
            </div>

            {/* Data Stats */}
            <div className="card">
              <h3 className="text-lg font-bold text-white mb-4">Data Usage</h3>
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <span className="text-slate-300">Downloaded</span>
                  <span className="text-white font-semibold">
                    {formatBytes(stats?.total_downloaded || 0)}
                  </span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-slate-300">Uploaded</span>
                  <span className="text-white font-semibold">
                    {formatBytes(stats?.total_uploaded || 0)}
                  </span>
                </div>
              </div>
            </div>

            {/* Connection Time */}
            {isConnected && (
              <div className="card">
                <div className="flex items-center gap-2 text-slate-300 mb-2">
                  <Clock className="w-5 h-5" />
                  <span>Connection Time</span>
                </div>
                <p className="text-white font-semibold text-lg">
                  {formatTime(stats?.connection_time || 0)}
                </p>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Settings Panel */}
      {showSettings && (
        <SettingsPanel onClose={() => setShowSettings(false)} />
      )}
    </div>
  )
}
