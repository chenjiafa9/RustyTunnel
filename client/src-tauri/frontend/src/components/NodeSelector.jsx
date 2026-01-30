import React, { useState, useEffect } from 'react'
import { MapPin, Wifi, Zap } from 'lucide-react'
import { api } from '../api'

export default function NodeSelector({ nodes, selectedNode, onSelectNode }) {
  const [pings, setPings] = useState({})
  const [testingNodes, setTestingNodes] = useState(new Set())

  useEffect(() => {
    testAllNodes()
  }, [nodes])

  const testAllNodes = async () => {
    for (const node of nodes) {
      testNode(node.id)
    }
  }

  const testNode = async (nodeId) => {
    setTestingNodes((prev) => new Set(prev).add(nodeId))
    try {
      const ping = await api.testNodeConnection(nodeId)
      setPings((prev) => ({ ...prev, [nodeId]: ping }))
    } catch (err) {
      console.error('Ping test failed:', err)
    } finally {
      setTestingNodes((prev) => {
        const next = new Set(prev)
        next.delete(nodeId)
        return next
      })
    }
  }

  const getPingColor = (ping) => {
    if (ping < 50) return 'text-green-400'
    if (ping < 100) return 'text-yellow-400'
    return 'text-red-400'
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
      {nodes.map((node) => (
        <button
          key={node.id}
          onClick={() => onSelectNode(node)}
          className={`p-4 rounded-lg border-2 transition-all text-left ${
            selectedNode?.id === node.id
              ? 'border-blue-500 bg-blue-500/10'
              : 'border-slate-600 bg-slate-700/50 hover:border-slate-500'
          }`}
        >
          <div className="flex items-start justify-between mb-2">
            <div>
              <h4 className="font-semibold text-white">{node.name}</h4>
              <p className="text-slate-400 text-sm flex items-center gap-1">
                <MapPin className="w-4 h-4" />
                {node.country}
              </p>
            </div>
            {pings[node.id] !== undefined && (
              <div className={`text-sm font-semibold flex items-center gap-1 ${getPingColor(pings[node.id])}`}>
                <Zap className="w-4 h-4" />
                {testingNodes.has(node.id) ? '...' : `${pings[node.id]}ms`}
              </div>
            )}
          </div>

          <div className="flex items-center justify-between text-xs text-slate-400">
            <span className="flex items-center gap-1">
              <Wifi className="w-4 h-4" />
              {node.protocol}
            </span>
            {node.load !== undefined && (
              <span>Load: {node.load.toFixed(1)}%</span>
            )}
          </div>
        </button>
      ))}
    </div>
  )
}
