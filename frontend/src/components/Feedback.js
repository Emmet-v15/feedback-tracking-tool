import React, { useState, useEffect } from 'react';
import '../styles/Feedback.css'; // Import the Feedback CSS

function Feedback() {
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [isViewingFeedback, setIsViewingFeedback] = useState(false);
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [feedbacks, setFeedbacks] = useState([]); // Store all feedbacks
    const [selectedFeedback, setSelectedFeedback] = useState(null); // Track which feedback is selected for viewing

    // Function to fetch feedbacks from the server
    const fetchFeedbacks = () => {
        fetch('http://localhost:3001/api/feedback')
            .then(response => response.json())
            .then(data => {
                if (data.success) {
                    setFeedbacks(data.data);
                }
            })
            .catch(error => console.error('Error fetching feedbacks:', error));
    };

    // Load feedbacks when component mounts
    useEffect(() => {
        fetchFeedbacks();
    }, []);

    // Function to open the "new feedback" modal
    const openModal = () => {
        setIsModalOpen(true);
        setIsViewingFeedback(false);
    };

    // Function to open the "view feedback" modal
    const openFeedbackModal = (feedback) => {
        setSelectedFeedback(feedback);
        setIsViewingFeedback(true);
        setIsModalOpen(true);
    };

    // Function to close the modal
    const closeModal = () => {
        setIsModalOpen(false);
        setSelectedFeedback(null);
    };

    // Function to handle form submission (for new feedback)
    const handleSubmit = (e) => {
        e.preventDefault();
        
        // Send feedback to the server
        fetch('http://localhost:3001/api/feedback', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ title, description })
        })
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                // Refresh the feedbacks list
                fetchFeedbacks();
                
                // Clear form and close modal
                setTitle('');
                setDescription('');
                closeModal();
            }
        })
        .catch(error => console.error('Error submitting feedback:', error));
    };

    // Function to format the time elapsed
    const getElapsedTime = (timestamp) => {
        const now = new Date();
        const diff = now - new Date(timestamp); // Difference in milliseconds
        const seconds = Math.floor(diff / 1000);
        const minutes = Math.floor(seconds / 60);
        const hours = Math.floor(minutes / 60);
        const days = Math.floor(hours / 24);

        if (days > 0) {
            return `${days} day${days > 1 ? 's' : ''} ago`;
        } else if (hours > 0) {
            return `${hours} hour${hours > 1 ? 's' : ''} ago`;
        } else if (minutes > 0) {
            return `${minutes} minute${minutes > 1 ? 's' : ''} ago`;
        } else {
            return `${seconds} second${seconds > 1 ? 's' : ''} ago`;
        }
    };

    // Update feedback timers every second
    useEffect(() => {
        const interval = setInterval(() => {
            setFeedbacks((prevFeedbacks) => {
                return prevFeedbacks.map((feedback) => ({
                    ...feedback,
                    elapsedTime: getElapsedTime(feedback.timestamp), // Update the elapsed time
                }));
            });
        }, 1000); // Update every 1 second

        // Cleanup interval on component unmount
        return () => clearInterval(interval);
    }, []);

    return (
        <main>
            <section className="page-header">
                <h2>Feedback</h2>
            </section>

            <section className="content-box">
                <button className="feedback-button" onClick={openModal}>
                    New Feedback
                </button>

                <div className="feedback-list">
                    {feedbacks.map((feedback, index) => (
                        <div
                            key={index}
                            className="feedback-item"
                            onClick={() => openFeedbackModal(feedback)}
                        >
                            <div className="feedback-details">
                                <h3>{feedback.title}</h3>
                                <p>{feedback.description.length > 50 ? `${feedback.description.substring(0, 250)}...` : feedback.description}</p>
                            </div>
                            <div className="feedback-timer-container">
                                <span className="feedback-timer-title">Feedback Created:</span>
                                <span className="feedback-timer">{feedback.elapsedTime || getElapsedTime(feedback.timestamp)}</span>
                            </div>
                        </div>
                    ))}
                </div>
            </section>

            {/* Modal for creating new feedback OR viewing feedback */}
            {isModalOpen && (
                <div className="modal-overlay">
                    <div className="modal-content">
                        {isViewingFeedback ? (
                            <>
                                <h3>{selectedFeedback.title}</h3>
                                <p>{selectedFeedback.description}</p>
                                <div>
                                    <button type="red" onClick={closeModal}>Close</button>
                                </div>
                            </>
                        ) : (
                            <>
                                <h3>Submit New Feedback</h3>
                                <form onSubmit={handleSubmit}>
                                    <div>
                                        <label htmlFor="title">Title</label>
                                        <input
                                            type="text"
                                            id="title"
                                            value={title}
                                            onChange={(e) => setTitle(e.target.value)}
                                            required
                                        />
                                    </div>
                                    <div>
                                        <label htmlFor="description">Description</label>
                                        <textarea
                                            id="description"
                                            value={description}
                                            onChange={(e) => setDescription(e.target.value)}
                                            required
                                        />
                                    </div>
                                    <div>
                                        <button type="submit">Submit</button>
                                        <button type="red" onClick={closeModal}>Cancel</button>
                                    </div>
                                </form>
                            </>
                        )}
                    </div>
                </div>
            )}
        </main>
    );
}

export default Feedback;