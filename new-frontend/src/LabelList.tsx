interface LabelListProps {
    feedbackId: number
}

export default function LabelList({ _feedbackId }: LabelListProps) {
    // Example static data for demonstration
    const labels = ['question', 'urgent']
    return (
        <div style={{ margin: '8px 0' }}>
            <strong>Labels:</strong> {labels.map(l => (
                <span key={l} style={{ marginRight: 8, padding: '2px 6px', background: '#333', borderRadius: 4 }}>{l}</span>
            ))}
        </div>
    )
}