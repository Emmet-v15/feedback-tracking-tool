import React, { useState } from 'react';
import '../styles/Feedback.css'; // Import the Feedback CSS

function Feedback() {
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [isViewingFeedback, setIsViewingFeedback] = useState(false);
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [feedbacks, setFeedbacks] = useState([]); // Store all feedbacks
    const [selectedFeedback, setSelectedFeedback] = useState(null); // Track which feedback is selected for viewing

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
        setFeedbacks([...feedbacks, { title, description }]); // Store title & description
        setTitle('');
        setDescription('');
        closeModal();
    };

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
                            <h3>{feedback.title}</h3>
                            <p>{feedback.description.length > 50 ? `${feedback.description.substring(0, 250)}...` : feedback.description}</p>
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
