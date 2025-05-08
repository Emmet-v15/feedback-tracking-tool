import React from 'react';
import { useEffect, useState, useContext } from 'react'
import { Box, Button, Typography, TextField, Chip, List, ListItem, ListItemText, IconButton, Dialog, DialogTitle, DialogContent, DialogActions, MenuItem, Paper } from '@mui/material'
import CloseIcon from '@mui/icons-material/Close'
import AddIcon from '@mui/icons-material/Add'
import { api } from '../api'
import { AuthContext } from '../AuthContext'

interface FeedbackItemProps {
    projectId: number
    feedbackId: number
    onBack: () => void
}

export default function FeedbackItem({ projectId, feedbackId, onBack }: FeedbackItemProps) {
  const { user } = useContext(AuthContext)
  const [feedback, setFeedback] = useState<any>(null)
  const [labels, setLabels] = useState<{ id: number; name: string; color: string }[]>([])
  const [comments, setComments] = useState<any[]>([])
  const [newComment, setNewComment] = useState('')
  const [addLabelOpen, setAddLabelOpen] = useState(false)
  const [newLabel, setNewLabel] = useState({ name: '', color: '#1976d2' })
  const [loading, setLoading] = useState(false)
  const [usernames, setUsernames] = useState<{ [id: number]: string }>({})

  const fetchDetails = async () => {
    const fb = await api.get(`/project/${projectId}/feedback/${feedbackId}/`);
    setFeedback(fb);
    const lbs = await api.get(`/project/${projectId}/feedback/${feedbackId}/labels/`);
    setLabels(lbs);
    const cms = await api.get(`/project/${projectId}/feedback/${feedbackId}/comments/`);
    setComments(cms);
  };

  useEffect(() => {
    fetchDetails();
  }, [projectId, feedbackId]);

  useEffect(() => {
    console.log('Comments with user data:', comments);
  }, [comments]);

  const canDeleteComment = (c: any) => user?.id === c.user_id || user?.role === 'admin'

  const handleAddLabel = async () => {
    setLoading(true);
    try {
      await api.post(`/project/${projectId}/feedback/${feedbackId}/labels/`, newLabel);
      await fetchDetails();
      setAddLabelOpen(false);
      setNewLabel({ name: '', color: '#1976d2' });
    } catch (error) {
      console.error('Error adding label:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleRemoveLabel = async (id: number) => {
    await api.del(`/project/${projectId}/feedback/${feedbackId}/labels/${id}`)
    setLabels(l => l.filter(x => x.id !== id))
  }
  const handleAddComment = async () => {
    if (!newComment) return
    setLoading(true)
    try {
      const cm = await api.post(`/project/${projectId}/feedback/${feedbackId}/comments/`, { content: newComment })
      setComments(c => [...c, { ...cm, username: user?.username }])
      setNewComment('')
    } finally {
      setLoading(false)
    }
  }
  const handleDeleteComment = async (id: number) => {
    await api.del(`/project/${projectId}/feedback/${feedbackId}/comments/${id}`)
    setComments(c => c.filter(x => x.id !== id))
  }

  const handleDeleteFeedback = async () => {
    if (!window.confirm('Are you sure you want to delete this feedback?')) return;
    await api.del(`/project/${projectId}/feedback/${feedbackId}/`);
    onBack();
  };

  const handleUpdateField = async (field: 'priority' | 'status', value: string) => {
    if (!feedback) return;
    const payload = {
      title: feedback.title,
      description: feedback.description,
      status: field === 'status' ? value : feedback.status,
      priority: field === 'priority' ? value : feedback.priority,
    };
    await api.put(`/project/${projectId}/feedback/${feedbackId}/`, payload);
    await fetchDetails();
  };

  if (!feedback) return null
  return (
    <Paper elevation={3} sx={{ p: 4, borderRadius: 3, maxWidth: 700, mx: 'auto', mt: 4, display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
      <Box sx={{ display: 'flex', gap: 2, mb: 2 }}>
        <Button onClick={onBack} variant="outlined">Back to Feedback List</Button>
        <Button onClick={handleDeleteFeedback} variant="outlined" color="error">Delete Feedback</Button>
      </Box>
      <Typography variant="h5" gutterBottom>{feedback.title}</Typography>
      <Typography variant="body1" sx={{ mb: 2, textAlign: 'center' }}>{feedback.description}</Typography>
      <Box sx={{ display: 'flex', gap: 2, mb: 2 }}>
        <TextField
          select
          label="Status"
          value={feedback.status}
          onChange={e => handleUpdateField('status', e.target.value)}
          size="small"
          sx={{ minWidth: 140 }}
        >
          {['open', 'in_progress', 'resolved', 'closed'].map(option => (
            <MenuItem key={option} value={option}>{option.replace('_', ' ').toUpperCase()}</MenuItem>
          ))}
        </TextField>
        <TextField
          select
          label="Priority"
          value={feedback.priority}
          onChange={e => handleUpdateField('priority', e.target.value)}
          size="small"
          sx={{ minWidth: 120 }}
        >
          {['high', 'medium', 'low'].map(option => (
            <MenuItem key={option} value={option}>{option.toUpperCase()}</MenuItem>
          ))}
        </TextField>
      </Box>

      <Box sx={{ mb: 3, width: '100%', maxWidth: 600 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 1 }}>
          <Typography variant="subtitle1">Labels</Typography>
          <IconButton size="small" onClick={() => setAddLabelOpen(true)} color="primary">
            <AddIcon />
          </IconButton>
        </Box>
        <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 1 }}>
          {labels.map(l => (
            <Chip 
              key={l.id} 
              label={l.name || 'Unnamed Label'} 
              sx={{ 
                bgcolor: l.color || '#1976d2', 
                color: '#fff', 
                mr: 1, 
                mb: 1,
                '& .MuiChip-label': {
                  color: '#fff'
                }
              }} 
              onDelete={() => handleRemoveLabel(l.id)} 
            />
          ))}
        </Box>
      </Box>

      <Dialog open={addLabelOpen} onClose={() => setAddLabelOpen(false)}>
        <DialogTitle>Add Label</DialogTitle>
        <DialogContent>
          <TextField label="Name" value={newLabel.name} onChange={e => setNewLabel(n => ({ ...n, name: e.target.value }))} fullWidth sx={{ mb: 2 }} />
          <TextField label="Color" type="color" value={newLabel.color} onChange={e => setNewLabel(n => ({ ...n, color: e.target.value }))} fullWidth />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setAddLabelOpen(false)}>Cancel</Button>
          <Button onClick={handleAddLabel} disabled={loading || !newLabel.name}>Add</Button>
        </DialogActions>
      </Dialog>

      <Box sx={{ mb: 3, width: '100%', maxWidth: 600 }}>
        <Typography variant="subtitle1" align="center">Comments</Typography>
        <List sx={{ width: '100%' }}>
          {comments.map(c => (
            <Paper key={c.id} elevation={1} sx={{ mb: 1, px: 3, py: 1.25, borderRadius: 2 }}>
              <ListItem disableGutters
                secondaryAction={
                  canDeleteComment(c) && (
                    <IconButton edge="end" onClick={() => handleDeleteComment(c.id)} sx={{ ml: -1, px: 1.5 }}>
                      <CloseIcon />
                    </IconButton>
                  )
                }
              >
                <ListItemText primary={c.content} secondary={`User ${c.username || c.userId || c.user_id || 'Unknown User'}`} />
              </ListItem>
            </Paper>
          ))}
        </List>
        <Box display="flex" alignItems="center" justifyContent="center" sx={{ mt: 2 }}>
          <TextField label="New Comment" value={newComment} onChange={e => setNewComment(e.target.value)} fullWidth sx={{ maxWidth: 500 }} />
          <Button onClick={handleAddComment} disabled={loading || !newComment} sx={{ ml: 1 }}>Add</Button>
        </Box>
      </Box>
    </Paper>
  )
}