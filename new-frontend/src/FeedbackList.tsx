import { useState, useEffect } from 'react'
import { Card, CardContent, Typography, CardActionArea, Box, Button, Dialog, DialogTitle, DialogContent, DialogActions, TextField, Chip, MenuItem } from '@mui/material'
import { Grid } from '@mui/material'
import AddIcon from '@mui/icons-material/Add'
import { api } from './api'
import FeedbackItem from './FeedbackItem'

interface FeedbackListProps {
    projectId: number
    onBack: () => void
}

interface Feedback {
    id: number
    title: string
    description: string
    status: 'open' | 'in_progress' | 'resolved'
    priority: 'low' | 'medium' | 'high'
    labels?: { id: number; name: string; color: string }[]
}

const STATUS_OPTIONS = ['open', 'in_progress', 'resolved']
const PRIORITY_OPTIONS = ['low', 'medium', 'high']

const STATUS_COLORS = {
    open: '#ff9800',
    in_progress: '#2196f3',
    resolved: '#4caf50'
}

const PRIORITY_COLORS = {
    low: '#4caf50',
    medium: '#ff9800',
    high: '#f44336'
}

export default function FeedbackList({ projectId, onBack }: FeedbackListProps) {
    const [feedbacks, setFeedbacks] = useState<Feedback[]>([])
    const [selectedFeedback, setSelectedFeedback] = useState<number | null>(null)
    const [addFeedbackOpen, setAddFeedbackOpen] = useState(false)
    const [newFeedback, setNewFeedback] = useState({ title: '', description: '', status: 'open', priority: 'medium' })
    const [loading, setLoading] = useState(false)

    useEffect(() => {
        fetchFeedbacks()
    }, [projectId])

    const fetchFeedbacks = async () => {
        const data = await api.get(`/project/${projectId}/feedback/`)
        // Fetch labels for each feedback
        const feedbacksWithLabels = await Promise.all(data.map(async (f: Feedback) => {
            const labels = await api.get(`/project/${projectId}/feedback/${f.id}/labels/`)
            return { ...f, labels }
        }))
        setFeedbacks(feedbacksWithLabels)
    }

    const handleAddFeedback = async () => {
        setLoading(true)
        try {
            const created = await api.post(`/project/${projectId}/feedback/`, newFeedback)
            setFeedbacks(f => [...f, { ...created, labels: [] }])
            setAddFeedbackOpen(false)
            setNewFeedback({ title: '', description: '', status: 'open', priority: 'medium' })
        } finally {
            setLoading(false)
        }
    }

    if (selectedFeedback !== null) {
        return <FeedbackItem projectId={projectId} feedbackId={selectedFeedback} onBack={() => setSelectedFeedback(null)} />
    }

    return (
        <Box sx={{ p: 3 }}>
            <Button onClick={onBack} variant="outlined" sx={{ mb: 2 }}>Back to Projects</Button>
            <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
                <Typography variant="h5">Feedback List</Typography>
                <Button variant="contained" onClick={() => setAddFeedbackOpen(true)} startIcon={<AddIcon />}>Add Feedback</Button>
            </Box>
            <Dialog open={addFeedbackOpen} onClose={() => setAddFeedbackOpen(false)} maxWidth="sm" fullWidth>
                <DialogTitle>Add Feedback</DialogTitle>
                <DialogContent>
                    <TextField
                        label="Title"
                        value={newFeedback.title}
                        onChange={e => setNewFeedback(f => ({ ...f, title: e.target.value }))}
                        fullWidth
                        sx={{ mb: 2, mt: 1 }}
                    />
                    <TextField
                        label="Description"
                        value={newFeedback.description}
                        onChange={e => setNewFeedback(f => ({ ...f, description: e.target.value }))}
                        fullWidth
                        multiline
                        rows={3}
                        sx={{ mb: 2 }}
                    />
                    <TextField
                        select
                        label="Status"
                        value={newFeedback.status}
                        onChange={e => setNewFeedback(f => ({ ...f, status: e.target.value }))}
                        fullWidth
                        sx={{ mb: 2 }}
                    >
                        {STATUS_OPTIONS.map(option => (
                            <MenuItem key={option} value={option}>
                                {option.replace('_', ' ').toUpperCase()}
                            </MenuItem>
                        ))}
                    </TextField>
                    <TextField
                        select
                        label="Priority"
                        value={newFeedback.priority}
                        onChange={e => setNewFeedback(f => ({ ...f, priority: e.target.value }))}
                        fullWidth
                    >
                        {PRIORITY_OPTIONS.map(option => (
                            <MenuItem key={option} value={option}>
                                {option.toUpperCase()}
                            </MenuItem>
                        ))}
                    </TextField>
                </DialogContent>
                <DialogActions>
                    <Button onClick={() => setAddFeedbackOpen(false)} disabled={loading}>Cancel</Button>
                    <Button onClick={handleAddFeedback} disabled={loading || !newFeedback.title} variant="contained">Add</Button>
                </DialogActions>
            </Dialog>
            <Box sx={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))', gap: 3, pb: 6 }}>
                {feedbacks.map(f => (
                    <Card
                        key={f.id}
                        onClick={() => setSelectedFeedback(f.id)}
                        sx={{
                            bgcolor: '#232323',
                            color: '#fff',
                            minHeight: 180,
                            display: 'flex',
                            flexDirection: 'column',
                            cursor: 'pointer',
                            boxShadow: 3,
                            borderRadius: 2,
                            transition: 'transform 0.15s, box-shadow 0.15s',
                            '&:hover': {
                                transform: 'translateY(-6px) scale(1.03)',
                                boxShadow: 8,
                                bgcolor: '#282828',
                            },
                        }}
                    >
                        <CardActionArea sx={{ height: '100%' }}>
                            <CardContent>
                                <Typography variant="h6" gutterBottom>{f.title}</Typography>
                                <Typography variant="body2" color="text.secondary" sx={{ color: '#bbb', mb: 2 }}>
                                    {f.description.length > 100 ? f.description.slice(0, 100) + '...' : f.description}
                                </Typography>
                                <Box display="flex" gap={1} mb={1}>
                                    <Chip
                                        label={f.status.replace('_', ' ').toUpperCase()}
                                        size="small"
                                        sx={{ bgcolor: STATUS_COLORS[f.status], color: '#fff' }}
                                    />
                                    <Chip
                                        label={f.priority.toUpperCase()}
                                        size="small"
                                        sx={{ bgcolor: PRIORITY_COLORS[f.priority], color: '#fff' }}
                                    />
                                </Box>
                                <Box display="flex" gap={1} flexWrap="wrap">
                                    {f.labels?.map(label => (
                                        <Chip
                                            key={label.id}
                                            label={label.name}
                                            size="small"
                                            sx={{ bgcolor: label.color, color: '#fff' }}
                                        />
                                    ))}
                                </Box>
                            </CardContent>
                        </CardActionArea>
                    </Card>
                ))}
            </Box>
        </Box>
    )
}