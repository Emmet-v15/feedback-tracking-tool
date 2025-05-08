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
  const [comments, setComments] = useState<{ id: number; content: string; user_id: number }[]>([])
  const [newComment, setNewComment] = useState('')
  const [addLabelOpen, setAddLabelOpen] = useState(false)
  const [newLabel, setNewLabel] = useState({ name: '', color: '#1976d2' })
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    async function fetchDetails() {
      const fb = await api.get(`/project/${projectId}/feedback/${feedbackId}/`)
      setFeedback(fb)
      const lbs = await api.get(`/project/${projectId}/feedback/${feedbackId}/labels/`)
      setLabels(lbs)
      const cms = await api.get(`/project/${projectId}/feedback/${feedbackId}/comments/`)
      setComments(cms)
    }
    fetchDetails()
  }, [projectId, feedbackId])

  const canDeleteComment = (c: any) => user?.id === c.user_id || user?.role === 'admin'

  const handleAddLabel = async () => {
    setLoading(true)
    try {
      const lbl = await api.post(`/project/${projectId}/feedback/${feedbackId}/labels/`, newLabel)
      setLabels(l => [...l, lbl])
      setNewLabel({ name: '', color: '#1976d2' })
      setAddLabelOpen(false)
    } finally {
      setLoading(false)
    }
  }
  const handleRemoveLabel = async (id: number) => {
    await api.del(`/project/${projectId}/feedback/${feedbackId}/labels/${id}`)
    setLabels(l => l.filter(x => x.id !== id))
  }
  const handleAddComment = async () => {
    if (!newComment) return
    setLoading(true)
    try {
      const cm = await api.post(`/project/${projectId}/feedback/${feedbackId}/comments/`, { content: newComment })
      setComments(c => [...c, cm])
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
    <Box sx={{ p: 3 }}>
      <Button onClick={onBack} variant="outlined" sx={{ mb: 2 }}>Back to Feedback List</Button>
      <Typography variant="h5" gutterBottom>{feedback.title}</Typography>
      <Typography variant="body1" sx={{ mb: 2 }}>{feedback.description}</Typography>

      <Box sx={{ mb: 3 }}>
        <Typography variant="subtitle1">Labels</Typography>
        {labels.map(l => (
          <Chip key={l.id} label={l.name} sx={{ bgcolor: l.color, color: '#fff', mr: 1, mb: 1 }} onDelete={() => handleRemoveLabel(l.id)} />
        ))}
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

      <Box sx={{ mb: 3 }}>
        <Typography variant="subtitle1">Comments</Typography>
        <List>
          {comments.map(c => (
            <ListItem key={c.id} secondaryAction={
              canDeleteComment(c) && <IconButton edge="end" onClick={() => handleDeleteComment(c.id)}><CloseIcon /></IconButton>
            }>
              <ListItemText primary={c.content} secondary={`User ${c.user_id}`} />
            </ListItem>
          ))}
        </List>
        <Box display="flex" alignItems="center">
          <TextField label="New Comment" value={newComment} onChange={e => setNewComment(e.target.value)} fullWidth />
          <Button onClick={handleAddComment} disabled={loading || !newComment} sx={{ ml: 1 }}>Add</Button>
        </Box>
      </Box>
    </Box>
  )
}