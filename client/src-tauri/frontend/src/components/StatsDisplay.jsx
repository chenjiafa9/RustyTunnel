import React from 'react'
import { Download, Upload, Activity } from 'lucide-react'

export default function StatsDisplay({ stats }) {
  const formatSpeed = (bytes) => {
    if (bytes === 0) return '0 B/s'
    const k = 1024
    const sizes = ['B/s', 'KB/s', 'MB/s', 'GB/s']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
  }

  return (
    <div className="grid grid-cols-3 gap-4">
      <div className="bg-slate-700/50 rounded-lg p-4">
        <div className="flex items-center gap-2 text-slate-400 mb-2">
          <Download className="w-5 h-5 text-green-400" />
          <span className="text-sm">Download</span>
        </div>
        <p className="text-white font-bold text-lg">
          {formatSpeed(stats?.download_speed || 0)}
        </p>
      </div>

      <div className="bg-slate-700/50 rounded-lg p-4">
        <div className="flex items-center gap-2 text-slate-400 mb-2">
          <Upload className="w-5 h-5 text-blue-400" />
          <span className="text-sm">Upload</span>
        </div>
        <p className="text-white font-bold text-lg">
          {formatSpeed(stats?.upload_speed || 0)}
        </p>
      </div>

      <div className="bg-slate-700/50 rounded-lg p-4">
        <div className="flex items-center gap-2 text-slate-400 mb-2">
          <Activity className="w-5 h-5 text-purple-400" />
          <span className="text-sm">Ping</span>
        </div>
        <p className="text-white font-bold text-lg">
          {stats?.ping || '--'} ms
        </p>
      </div>
    </div>
  )
}
