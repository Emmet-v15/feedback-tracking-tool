import { useState, useEffect } from 'react'
import FeedbackList from './FeedbackList'

interface Project {
    id: number;
    name: string;
    // Add other fields if needed
}

export default function ProjectsView() {
    const [projects, setProjects] = useState<Project[]>([])
    const [selectedProject, setSelectedProject] = useState<number | null>(null)
    const [loading, setLoading] = useState(true)
    const [error, setError] = useState<string | null>(null)

    useEffect(() => {
        async function fetchProjects() {
            setLoading(true)
            setError(null)
            try {
                const token = localStorage.getItem('token')
                const res = await fetch('/api/projects/', {
                    headers: token ? { 'Authorization': `Bearer ${token}` } : {}
                })
                if (!res.ok) throw new Error('Failed to fetch projects')
                const data = await res.json()
                setProjects(data)
            } catch (err) {
                if (err instanceof Error) setError(err.message)
                else setError('Unknown error')
            } finally {
                setLoading(false)
            }
        }
        fetchProjects()
    }, [])

    if (selectedProject) {
        return <FeedbackList projectId={selectedProject} onBack={() => setSelectedProject(null)} />
    }

    return (
        <div style={{ padding: 24 }}>
            <h2>Projects</h2>
            {loading && <div>Loading...</div>}
            {error && <div style={{color: 'red'}}>{error}</div>}
            <ul>
                {projects.map(p => (
                    <li key={p.id}>
                        <button onClick={() => setSelectedProject(p.id)}>{p.name}</button>
                    </li>
                ))}
            </ul>
        </div>
    )
}