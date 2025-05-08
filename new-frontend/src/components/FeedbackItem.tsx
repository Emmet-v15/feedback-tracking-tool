import React from 'react';
import { useEffect, useState, useContext } from 'react'
import { Box, Button, Typography, TextField, Chip, List, ListItem, ListItemText, IconButton, Dialog, DialogTitle, DialogContent, DialogActions } from '@mui/material'
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

  if (!feedback) return null
  return (
    <Box sx={{ p: 3, display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
      <Button onClick={onBack} variant="outlined" sx={{ mb: 2 }}>Back to Feedback List</Button>
      <Typography variant="h5" gutterBottom>{feedback.title}</Typography>
      <Typography variant="body1" sx={{ mb: 2, textAlign: 'center' }}>{feedback.description}</Typography>

      <Box sx={{ mb: 3, width: '100%', maxWidth: 600 }}>
        <Typography variant="subtitle1" align="center">Labels</Typography>
        <Box sx={{ display: 'flex', justifyContent: 'center', flexWrap: 'wrap' }}>
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
        <Button size="small" onClick={() => setAddLabelOpen(true)} startIcon={<AddIcon />}>Add Label</Button>
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
            <ListItem key={c.id} secondaryAction={
              canDeleteComment(c) && <IconButton edge="end" onClick={() => handleDeleteComment(c.id)}><CloseIcon /></IconButton>
            }>
              <ListItemText primary={c.content} secondary={`User ${c.username || c.userId || c.user_id || 'Unknown User'}`} />
            </ListItem>
          ))}
        </List>
        <Box display="flex" alignItems="center" justifyContent="center" sx={{ mt: 2 }}>
          <TextField label="New Comment" value={newComment} onChange={e => setNewComment(e.target.value)} fullWidth sx={{ maxWidth: 500 }} />
          <Button onClick={handleAddComment} disabled={loading || !newComment} sx={{ ml: 1 }}>Add</Button>
        </Box>
      </Box>
    </Box>
  )
}