import React, { useState, useEffect } from 'react'
import LoginPage from './pages/LoginPage'
import DashboardPage from './pages/DashboardPage'
import { api } from './api'

export default function App() {
  const [user, setUser] = useState(null)
  const [token, setToken] = useState(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    checkAuth()
  }, [])

  const checkAuth = async () => {
    try {
      const user = await api.getCurrentUser()
      setUser(user)
    } catch (err) {
      setUser(null)
    } finally {
      setLoading(false)
    }
  }

  const handleLoginSuccess = (user, token) => {
    setUser(user)
    setToken(token)
  }

  const handleLogout = async () => {
    try {
      await api.logout()
      setUser(null)
      setToken(null)
    } catch (err) {
      console.error('Logout failed:', err)
    }
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-slate-900 flex items-center justify-center">
        <div className="text-white text-xl">Loading...</div>
      </div>
    )
  }

  return user ? (
    <DashboardPage user={user} onLogout={handleLogout} />
  ) : (
    <LoginPage onLoginSuccess={handleLoginSuccess} />
  )
}
