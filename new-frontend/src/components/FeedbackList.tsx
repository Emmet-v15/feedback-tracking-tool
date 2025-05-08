import { useState, useEffect } from 'react'
import { Card, CardContent, Typography, CardActionArea, Box, Button, Dialog, DialogTitle, DialogContent, DialogActions, TextField, MenuItem, Chip } from '@mui/material'
import Grid from '@mui/material/Grid'
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
        if (!['high', 'medium', 'low'].includes(newFeedback.priority)) {
            alert('Priority must be one of: high, medium, low');
            return;
        }
        if (!['open', 'in_progress', 'resolved', 'closed'].includes(newFeedback.status)) {
            alert('Status must be one of: open, in_progress, resolved, closed');
            return;
        }
        setLoading(true);
        try {
            const created = await api.post(`/project/${projectId}/feedback/`, newFeedback);
            setFeedbacks(f => [...f, created]);
            setAddFeedbackOpen(false);
            setNewFeedback({ title: '', description: '', status: 'open', priority: 'medium' });
        } finally {
            setLoading(false);
        }
    };

    const handleBackFromDetail = () => {
        setSelectedFeedback(null);
        fetchFeedbacks();
    };

    if (selectedFeedback !== null) {
        return <FeedbackItem projectId={projectId} feedbackId={selectedFeedback} onBack={handleBackFromDetail} />
    }

    return (
        <Box sx={{ p: 3 }}>
            <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
                <Button onClick={onBack} variant="outlined">Back to Projects</Button>
                <Button variant="contained" onClick={() => setAddFeedbackOpen(true)} startIcon={<AddIcon />}>Add Feedback</Button>
            </Box>
            <Typography variant="h5" sx={{ mb: 2 }}>Feedback List</Typography>
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
                        select
                        label="Status"
                        value={newFeedback.status}
                        onChange={e => setNewFeedback(f => ({ ...f, status: e.target.value }))}
                        fullWidth
                        sx={{ mb: 2 }}
                    >
                        {['open', 'in_progress', 'resolved', 'closed'].map(option => (
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
                        {['high', 'medium', 'low'].map(option => (
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
            <Grid container spacing={3} sx={{ pb: 6 }}>
                {feedbacks.map(f => (
                    <Grid item xs={12} sm={6} md={4} lg={3} key={f.id}>
                        <Card
                            onClick={() => setSelectedFeedback(f.id)}
                            sx={{
                                bgcolor: '#232323',
                                color: '#fff',
                                width: 250,
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
                                    <Box sx={{ display: 'flex', gap: 1, mb: 1 }}>
                                        <Chip label={f.status?.replace('_', ' ').toUpperCase() || 'UNKNOWN'} size="small" sx={{ bgcolor: getStatusColor(f.status), color: '#fff' }} />
                                        <Chip label={f.priority?.toUpperCase() || 'UNKNOWN'} size="small" sx={{ bgcolor: getPriorityColor(f.priority), color: '#fff' }} />
                                    </Box>
                                </CardContent>
                            </CardActionArea>
                        </Card>
                    </Grid>
                ))}
            </Grid>
        </Box>
    )
}

function getStatusColor(status: string): string {
    switch (status) {
        case 'open': return '#1976d2';
        case 'in_progress': return '#ffa000';
        case 'resolved': return '#388e3c';
        case 'closed': return '#616161';
        default: return '#757575';
    }
}

function getPriorityColor(priority: string): string {
    switch (priority) {
        case 'high': return '#d32f2f';
        case 'medium': return '#fbc02d';
        case 'low': return '#388e3c';
        default: return '#757575';
    }
}