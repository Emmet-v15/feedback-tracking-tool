import { useContext, useEffect, useState } from 'react'
import { Container, Box, Fab, Typography, Card, CardContent, CardActions, IconButton, Dialog, DialogTitle, DialogContent, DialogActions, TextField, Button, List, ListItem, ListItemText } from '@mui/material'
import AddIcon from '@mui/icons-material/Add'
import EditIcon from '@mui/icons-material/Edit'
import DeleteIcon from '@mui/icons-material/Delete'
import PersonAddIcon from '@mui/icons-material/PersonAdd'
import PersonRemoveIcon from '@mui/icons-material/PersonRemove'
import ProjectDialog from './components/ProjectDialog'
import FeedbackList from './components/FeedbackList'
import { api } from './api'
import { AuthContext } from './AuthContext'

export interface Project {
  id?: number
  name: string
  description?: string
  created_at?: string
  updated_at?: string
}

export default function ProjectsView() {
  const { user } = useContext(AuthContext)
  const isPrivileged = user?.role === 'teacher' || user?.role === 'admin'
  const [selectedProjectId, setSelectedProjectId] = useState<number | null>(null)
  const [projects, setProjects] = useState<Project[]>([])
  const [dialogOpen, setDialogOpen] = useState(false)
  const [active, setActive] = useState<Project | null>(null)
  const [enrollOpen, setEnrollOpen] = useState<number | null>(null)
  const [enrolledUsers, setEnrolledUsers] = useState<number[]>([])
  const [enrollUserId, setEnrollUserId] = useState<string>('')

  useEffect(() => { fetchProjects() }, [])

  const fetchProjects = async () => {
    const data = await api.get('/project/')
    setProjects(data)
  }

  const handleAdd = () => { setActive(null); setDialogOpen(true) }
  const handleEdit = (proj: Project) => { setActive(proj); setDialogOpen(true) }
  const handleDelete = async (id?: number) => {
    if (!id) return
    await api.del(`/project/${id}`)
    setProjects(p => p.filter(x => x.id !== id))
  }
  const handleClose = () => setDialogOpen(false)
  const handleSubmit = async (data: Omit<Project,'id'> & Partial<Pick<Project,'id'>>) => {
    if (data.id) {
      const res = await api.put(`/project/${data.id}`, data)
      if (res.ok) {
        setProjects(p => p.map(x => x.id === data.id ? { ...x, ...data } : x))
      }
    } else {
      const created = await api.post('/project/', data)
      setProjects(p => [...p, created])
    }
    setDialogOpen(false)
  }

  const openEnroll = async (id?: number) => {
    if (!id) return
    setEnrollOpen(id)
    const users: number[] = await api.get(`/project/${id}/enrollment/`)
    setEnrolledUsers(users)
  }
  const handleEnroll = async () => {
    const pid = enrollOpen!
    await api.post(`/project/${pid}/enrollment/`, { user_id: Number(enrollUserId) })
    setEnrolledUsers(u => [...u, Number(enrollUserId)])
    setEnrollUserId('')
  }
  const handleUnenroll = async (uid: number) => {
    const pid = enrollOpen!
    await api.del(`/project/${pid}/enrollment/?user_id=${uid}`)
    setEnrolledUsers(u => u.filter(x => x !== uid))
  }

  if (selectedProjectId !== null) {
    return <FeedbackList projectId={selectedProjectId} onBack={() => setSelectedProjectId(null)} />
  }

  return (
    <Container maxWidth="md" sx={{ mt: 4, mb: 6 }}>
      <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
        <Typography variant="h4">Projects</Typography>
        <Fab color="primary" size="small" onClick={handleAdd} aria-label="add project">
          <AddIcon />
        </Fab>
      </Box>
      <Box sx={{ 
        display: 'grid', 
        gridTemplateColumns: {
          xs: '1fr',
          sm: 'repeat(2, 1fr)',
          md: 'repeat(3, 1fr)'
        },
        gap: 3,
        pb: 6 
      }}>
        {projects.map(proj => (
          <Card
            key={proj.id}
            onClick={() => setSelectedProjectId(proj.id!)}
            sx={{
              bgcolor: '#232323',
              color: '#fff',
              minHeight: 200,
              display: 'flex',
              flexDirection: 'column',
              justifyContent: 'space-between',
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
            <CardContent>
              <Typography variant="h6" gutterBottom>{proj.name}</Typography>
              <Typography variant="body2" color="text.secondary" sx={{ color: '#bbb' }}>
                {proj.description || <span style={{ color: '#555' }}>No description</span>}
              </Typography>
              {proj.created_at && (
                <Typography variant="caption" sx={{ display: 'block', mt: 2, color: '#666' }}>
                  Created: {new Date(proj.created_at).toLocaleString()}
                </Typography>
              )}
            </CardContent>
            <CardActions sx={{ justifyContent: 'flex-end' }}>
              {isPrivileged && (
                <IconButton onClick={e => { e.stopPropagation(); handleEdit(proj) }} aria-label="edit" color="primary">
                  <EditIcon />
                </IconButton>
              )}
              {isPrivileged && (
                <IconButton onClick={e => { e.stopPropagation(); handleDelete(proj.id) }} aria-label="delete" color="error">
                  <DeleteIcon />
                </IconButton>
              )}
              {isPrivileged && (
                <IconButton onClick={e => { e.stopPropagation(); openEnroll(proj.id) }} aria-label="enroll" color="secondary">
                  <PersonAddIcon />
                </IconButton>
              )}
            </CardActions>
          </Card>
        ))}
      </Box>

      <Dialog open={enrollOpen !== null} onClose={() => setEnrollOpen(null)}>
        <DialogTitle>Manage Enrollments</DialogTitle>
        <DialogContent>
          <List>
            {enrolledUsers.map(uid => (
              <ListItem key={uid} secondaryAction={
                <IconButton edge="end" onClick={() => handleUnenroll(uid)}><PersonRemoveIcon /></IconButton>
              }>
                <ListItemText primary={`User ${uid}`} />
              </ListItem>
            ))}
          </List>
          <Box display="flex" mt={2}>
            <TextField label="User ID" value={enrollUserId} onChange={e => setEnrollUserId(e.target.value)} fullWidth />
            <Button onClick={handleEnroll} disabled={!enrollUserId} variant="contained" sx={{ ml: 1 }}>Enroll</Button>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setEnrollOpen(null)}>Close</Button>
        </DialogActions>
      </Dialog>

      <ProjectDialog open={dialogOpen} project={active} onClose={handleClose} onSubmit={handleSubmit} />
    </Container>
  )
}
