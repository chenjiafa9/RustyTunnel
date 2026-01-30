import React, { useState, useEffect } from 'react'
import { X, Save } from 'lucide-react'
import { api } from '../api'

export default function SettingsPanel({ onClose }) {
  const [settings, setSettings] = useState(null)
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    loadSettings()
  }, [])

  const loadSettings = async () => {
    try {
      const settings = await api.getSettings()
      setSettings(settings)
    } catch (err) {
      console.error('Failed to load settings:', err)
    }
  }

  const handleSave = async () => {
    if (!settings) return
    setLoading(true)
    try {
      await api.updateSettings(settings)
      alert('Settings saved successfully')
      onClose()
    } catch (err) {
      console.error('Failed to save settings:', err)
      alert('Failed to save settings')
    } finally {
      setLoading(false)
    }
  }

  if (!settings) {
    return null
  }

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-slate-800 rounded-lg shadow-2xl max-w-md w-full mx-4 max-h-96 overflow-y-auto">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-slate-700 sticky top-0 bg-slate-800">
          <h2 className="text-xl font-bold text-white">Settings</h2>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-white transition"
          >
            <X className="w-6 h-6" />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 space-y-6">
          {/* Theme */}
          <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">
              Theme
            </label>
            <select
              value={settings.theme}
              onChange={(e) =>
                setSettings({ ...settings, theme: e.target.value })
              }
              className="input"
            >
              <option value="dark">Dark</option>
              <option value="light">Light</option>
            </select>
          </div>

          {/* Language */}
          <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">
              Language
            </label>
            <select
              value={settings.language}
              onChange={(e) =>
                setSettings({ ...settings, language: e.target.value })
              }
              className="input"
            >
              <option value="en">English</option>
              <option value="zh">中文</option>
              <option value="es">Español</option>
            </select>
          </div>

          {/* Default Protocol */}
          <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">
              Default Protocol
            </label>
            <select
              value={settings.default_protocol}
              onChange={(e) =>
                setSettings({ ...settings, default_protocol: e.target.value })
              }
              className="input"
            >
              <option value="WireGuard">WireGuard</option>
              <option value="OpenVPN">OpenVPN</option>
              <option value="IKEv2">IKEv2</option>
            </select>
          </div>

          {/* Auto Start */}
          <div className="flex items-center justify-between">
            <label className="text-sm font-medium text-slate-300">
              Auto Start
            </label>
            <input
              type="checkbox"
              checked={settings.auto_start}
              onChange={(e) =>
                setSettings({ ...settings, auto_start: e.target.checked })
              }
              className="w-5 h-5 rounded"
            />
          </div>

          {/* Minimize to Tray */}
          <div className="flex items-center justify-between">
            <label className="text-sm font-medium text-slate-300">
              Minimize to Tray
            </label>
            <input
              type="checkbox"
              checked={settings.minimize_to_tray}
              onChange={(e) =>
                setSettings({ ...settings, minimize_to_tray: e.target.checked })
              }
              className="w-5 h-5 rounded"
            />
          </div>

          {/* DNS Servers */}
          <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">
              DNS Servers
            </label>
            <textarea
              value={settings.dns_servers.join('\n')}
              onChange={(e) =>
                setSettings({
                  ...settings,
                  dns_servers: e.target.value
                    .split('\n')
                    .filter((s) => s.trim()),
                })
              }
              className="input h-24 resize-none"
              placeholder="One DNS server per line"
            />
          </div>
        </div>

        {/* Footer */}
        <div className="flex gap-3 p-6 border-t border-slate-700 bg-slate-800/50 sticky bottom-0">
          <button onClick={onClose} className="btn-secondary flex-1">
            Cancel
          </button>
          <button
            onClick={handleSave}
            disabled={loading}
            className="btn-primary flex-1 flex items-center justify-center gap-2"
          >
            <Save className="w-5 h-5" />
            {loading ? 'Saving...' : 'Save'}
          </button>
        </div>
      </div>
    </div>
  )
}
