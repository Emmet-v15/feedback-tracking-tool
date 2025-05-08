import './App.css'
import { Routes, Route, Navigate } from 'react-router-dom'
import LoginPage from './LoginPage.tsx'
import RegisterPage from './RegisterPage.tsx'
import ProjectsView from './ProjectsView'
import { useEffect, useState } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'

export default function App() {
    const [isLoggedIn, setIsLoggedIn] = useState(Boolean(localStorage.getItem('token')))
    const location = useLocation()
    const navigate = useNavigate()

    useEffect(() => {
        async function checkAuth() {
            const token = localStorage.getItem('token')
            if (!token) {
                setIsLoggedIn(false)
                return
            }
            try {
                const res = await fetch('/api/user', {
                    headers: { 'Authorization': `Bearer ${token}` }
                })
                if (res.status === 401) {
                    localStorage.removeItem('token')
                    setIsLoggedIn(false)
                    navigate('/login', { replace: true })
                } else if (res.ok) {
                    setIsLoggedIn(true)
                }
            } catch {
                localStorage.removeItem('token')
                setIsLoggedIn(false)
                navigate('/login', { replace: true })
            }
        }
        checkAuth()
        // eslint-disable-next-line
    }, [location.pathname])

    if (!isLoggedIn) {
        return (
            <Routes>
                <Route path="/login" element={<LoginPage />} />
                <Route path="/register" element={<RegisterPage />} />
                <Route path="*" element={<Navigate to="/login" replace />} />
            </Routes>
        )
    }

    return (
        <Routes>
            <Route path="/projects" element={<ProjectsView />} />
            <Route path="*" element={<Navigate to="/projects" replace />} />
        </Routes>
    )
}