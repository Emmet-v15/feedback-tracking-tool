import { useState } from 'react'
import FeedbackItem from './FeedbackItem'

interface FeedbackListProps {
    projectId: number
    onBack: () => void
}

export default function FeedbackList({ projectId, onBack }: FeedbackListProps) {
    // Example static data for demonstration
    const [feedbacks] = useState([
        { id: 1, title: 'Need help with assignment', projectId },
        { id: 2, title: 'Clarification on grading', projectId },
    ])
    const [selectedFeedback, setSelectedFeedback] = useState<number | null>(null)

    if (selectedFeedback) {
        return <FeedbackItem feedbackId={selectedFeedback} onBack={() => setSelectedFeedback(null)} />
    }

    return (
        <div style={{ padding: 24 }}>
            <button onClick={onBack}>Back to Projects</button>
            <h3>Feedback List</h3>
            <ul>
                {feedbacks.map(f => (
                    <li key={f.id}>
                        <button onClick={() => setSelectedFeedback(f.id)}>{f.title}</button>
                    </li>
                ))}
            </ul>
        </div>
    )
}