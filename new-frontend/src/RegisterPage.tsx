import { useState } from 'react'
import type { FormEvent } from 'react'
import { Container, Paper, Typography, TextField, Button, Alert, Box, MenuItem, Select, InputLabel, FormControl } from '@mui/material'
import { useNavigate } from 'react-router-dom'

export default function RegisterPage() {
    const [form, setForm] = useState({ username: '', email: '', password: '', role: '' })
    const [error, setError] = useState('')
    const navigate = useNavigate()

    const handleChange = (e: React.ChangeEvent<HTMLInputElement> | { target: { name?: string; value: unknown } }) => {
        const { name, value } = e.target
        if (name) {
            setForm({ ...form, [name]: value })
        }
    }

    const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        setError('')
        if (!form.username || !form.email || !form.password || !form.role) {
            setError('All fields are required.')
            return
        }
        try {
            const response = await fetch('/api/register', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    username: form.username.trim(),
                    email: form.email.trim(),
                    password: form.password,
                    role: form.role
                })
            })
            let data = null
            switch (response.status) {
                case 200:
                case 201:
                    break;
                case 400:
                    throw new Error('Invalid input. Please check your data.')
                case 409:
                    throw new Error('Username or email already exists.')
                case 500:
                    throw new Error('Server error. Please try again later.')
                default:
                    throw new Error('An unexpected error occurred.')
                    
            }
            if (!response.ok) {
                throw new Error('Registration failed')
            }
            setForm({ username: '', email: '', password: '', role: '' })
            navigate('/login')
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
                    FeedTrack Register
                </Typography>
                {error && (
                    <Alert severity="error" sx={{ mb: 2 }}>
                        {error}
                    </Alert>
                )}
                <Box component="form" onSubmit={handleSubmit} sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                    <TextField
                        label="Username"
                        name="username"
                        variant="outlined"
                        value={form.username}
                        onChange={handleChange}
                        fullWidth
                    />
                    <TextField
                        label="Email"
                        name="email"
                        type="email"
                        variant="outlined"
                        value={form.email}
                        onChange={handleChange}
                        fullWidth
                    />
                    <TextField
                        label="Password"
                        name="password"
                        type="password"
                        variant="outlined"
                        value={form.password}
                        onChange={handleChange}
                        fullWidth
                    />
                    <FormControl fullWidth required>
                        <InputLabel id="role-label">Role</InputLabel>
                        <Select
                            labelId="role-label"
                            id="role"
                            name="role"
                            value={form.role}
                            label="Role"
                            onChange={handleChange}
                        >
                            <MenuItem value="teacher">Teacher</MenuItem>
                            <MenuItem value="student">Student</MenuItem>
                            <MenuItem value="admin">Admin</MenuItem>
                        </Select>
                    </FormControl>
                    <Button type="submit" variant="contained" color="primary" fullWidth disabled={!form.username || !form.email || !form.password || !form.role}>
                        Register
                    </Button>
                    <Button variant="outlined" color="secondary" fullWidth type="button" onClick={() => navigate('/login')}>
                        Login
                    </Button>
                </Box>
            </Paper>
        </Container>
    )
}
