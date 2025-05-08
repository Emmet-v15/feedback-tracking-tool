import { useState } from 'react'
import type { FormEvent } from 'react'
import { Container, Paper, Typography, TextField, Button, Alert, Box } from '@mui/material'
import { useNavigate } from 'react-router-dom'

export default function LoginPage() {
    const [username, setUsername] = useState('')
    const [password, setPassword] = useState('')
    const [error, setError] = useState('')
    const navigate = useNavigate()

    const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        setError('')
        try {
            const response = await fetch('/api/login', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ username: username.trim(), password })
            })
            const data = await response.text()
            if (!response.ok) {
                throw new Error(data || 'Login failed')
            }
            localStorage.setItem('token', data.replace(/^"|"$/g, ''))
            setUsername('')
            setPassword('')
            navigate('/projects', { replace: true })
        } catch (err: unknown) {
            const message = err instanceof Error ? err.message : 'An unexpected error occurred.'
            setError(message)
        }
    }

    return (
        <Container
            component="main"
            maxWidth="xs"
            sx={{
                height: '100vh',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
            }}
        >
            <Paper sx={{ p: 4, width: '100%' }}>
                <Typography variant="h4" align="center" gutterBottom>
                    FeedTrack Login
                </Typography>
                {error && (
                    <Alert severity="error" sx={{ mb: 2 }}>
                        {error}
                    </Alert>
                )}
                <Box component="form" onSubmit={handleSubmit} sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                    <TextField
                        label="Username"
                        variant="outlined"
                        value={username}
                        onChange={e => setUsername(e.target.value)}
                        fullWidth
                    />
                    <TextField
                        label="Password"
                        variant="outlined"
                        type="password"
                        value={password}
                        onChange={e => setPassword(e.target.value)}
                        fullWidth
                    />
                    <Button type="submit" variant="contained" color="primary" fullWidth disabled={!username || !password}>
                        Login
                    </Button>
                    <Button variant="outlined" color="secondary" fullWidth type="button" onClick={() => navigate('/register')}>
                        Register
                    </Button>
                </Box>
            </Paper>
        </Container>
    )
}