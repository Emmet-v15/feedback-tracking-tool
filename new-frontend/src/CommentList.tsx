interface CommentListProps {
    feedbackId: number
}

export default function CommentList({ _feedbackId }: CommentListProps) {
    // Example static data for demonstration
    const comments = [
        { id: 1, author: 'Student', text: 'Can you clarify the requirements?' },
        { id: 2, author: 'Teacher', text: 'Sure! Here are more details...' },
    ]
    return (
        <div style={{ margin: '8px 0' }}>
            <strong>Comments:</strong>
            <ul>
                {comments.map(c => (
                    <li key={c.id}><b>{c.author}:</b> {c.text}</li>
                ))}
            </ul>
        </div>
    )
}