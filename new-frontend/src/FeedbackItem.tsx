import CommentList from './CommentList.tsx'
import LabelList from './LabelList.tsx'

interface FeedbackItemProps {
    feedbackId: number
    onBack: () => void
}

export default function FeedbackItem({ feedbackId, onBack }: FeedbackItemProps) {
    // Example static data for demonstration
    const feedback = { id: feedbackId, title: 'Sample Feedback', description: 'Feedback details here.' }
    return (
        <div style={{ padding: 24 }}>
            <button onClick={onBack}>Back to Feedback List</button>
            <h4>{feedback.title}</h4>
            <p>{feedback.description}</p>
            <LabelList feedbackId={feedbackId} />
            <CommentList feedbackId={feedbackId} />
        </div>
    )
}