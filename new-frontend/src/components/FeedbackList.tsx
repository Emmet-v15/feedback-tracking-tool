import { useState, useEffect } from 'react'
import { Grid, Card, CardContent, Typography, CardActionArea, Box, Button, Dialog, DialogTitle, DialogContent, DialogActions, TextField } from '@mui/material'
import AddIcon from '@mui/icons-material/Add'
import { api } from '../api'
import FeedbackItem from './FeedbackItem'

interface FeedbackListProps {
    projectId: number
    onBack: () => void
}

export default function FeedbackList({ projectId, onBack }: FeedbackListProps) {
    const [feedbacks, setFeedbacks] = useState<any[]>([])
    const [selectedFeedback, setSelectedFeedback] = useState<number | null>(null)
    const [addFeedbackOpen, setAddFeedbackOpen] = useState(false)
    const [newFeedback, setNewFeedback] = useState({ title: '', description: '', status: 'open', priority: 'medium' })
    const [loading, setLoading] = useState(false)

    useEffect(() => {
        fetchFeedbacks()
    }, [projectId])

    const fetchFeedbacks = async () => {
        const data = await api.get(`/project/${projectId}/feedback/`)
        setFeedbacks(data)
    }

    const handleAddFeedback = async () => {
        setLoading(true)
        try {
            const created = await api.post(`/project/${projectId}/feedback/`, newFeedback)
            setFeedbacks(f => [...f, created])
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
            <Dialog open={addFeedbackOpen} onClose={() => setAddFeedbackOpen(false)}>
                <DialogTitle>Add Feedback</DialogTitle>
                <DialogContent>
                    <TextField
                        label="Title"
                        value={newFeedback.title}
                        onChange={e => setNewFeedback(f => ({ ...f, title: e.target.value }))}
                        fullWidth
                        sx={{ mb: 2 }}
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
                        label="Status"
                        value={newFeedback.status}
                        onChange={e => setNewFeedback(f => ({ ...f, status: e.target.value }))}
                        fullWidth
                        sx={{ mb: 2 }}
                    />
                    <TextField
                        label="Priority"
                        value={newFeedback.priority}
                        onChange={e => setNewFeedback(f => ({ ...f, priority: e.target.value }))}
                        fullWidth
                    />
                </DialogContent>
                <DialogActions>
                    <Button onClick={() => setAddFeedbackOpen(false)} disabled={loading}>Cancel</Button>
                    <Button onClick={handleAddFeedback} disabled={loading || !newFeedback.title} variant="contained">Add</Button>
                </DialogActions>
            </Dialog>
            <Grid container spacing={3} sx={{ pb: 6 }}>
                {feedbacks.map(f => (
                    <Grid component="div" key={f.id} xs={12} sm={6} md={4} lg={3}>
                        <Card
                            onClick={() => setSelectedFeedback(f.id)}
                            sx={{
                                bgcolor: '#232323',
                                color: '#fff',
                                height: 140,
                                display: 'flex',
                                flexDirection: 'column',
                                justifyContent: 'center',
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
                                    <Typography variant="subtitle1" gutterBottom>{f.title}</Typography>
                                </CardContent>
                            </CardActionArea>
                        </Card>
                    </Grid>
                ))}
            </Grid>
        </Box>
    )
}