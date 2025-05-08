export interface Project {
  id?: number
  name: string
  description?: string
  created_at?: string
  updated_at?: string
}

export interface Feedback {
  id: number
  title: string
  description: string
  status: string
  priority: string
}

export interface Label {
  id: number
  name: string
  color: string
}

export interface Comment {
  id: number
  content: string
  user_id: number
}

export interface User {
  id: number
  role: string
  username: string
}
