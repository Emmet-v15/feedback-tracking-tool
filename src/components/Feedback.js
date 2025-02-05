import React, { useState } from 'react';
import '../styles/Feedback.css'; // Import the Feedback CSS

// MODAL = NEW FEEDBACK BOX WHERE THE USER ENTERS THERE FEEDBACK

function Feedback() {
    // State to manage modal visibility
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [feedbacks, setFeedbacks] = useState([]); // State to store all submitted feedback

    // Function to open the modal
    const openModal = () => setIsModalOpen(true);

    // Function to close the modal
    const closeModal = () => setIsModalOpen(false);

    // Function to handle form submission (for feedback)
    const handleSubmit = (e) => {
        e.preventDefault();
        // Add new feedback to the feedbacks array
        setFeedbacks([...feedbacks, title]);
        // Reset form fields after submission
        setTitle('');
        setDescription('');
        closeModal();  // Close modal after submission
    };

    return (
        <main>
            {/* Header Section */}
            <section className="page-header">
                <h2>Feedback</h2>
            </section>

            {/* Main Content Section */}
            <section className="content-box">
                {/* New Feedback Button */}
                <button className="feedback-button" onClick={openModal}>
                    New Feedback
                </button>

                {/* Display all feedback titles */}
                <div className="feedback-list">
                    {feedbacks.map((feedback, index) => (
                        <div key={index} className="feedback-item">
                            <p>{feedback}</p>
                        </div>
                    ))}
                </div>
            </section>

            {/* Modal for New Feedback */}
            {isModalOpen && (
                <div className="modal-overlay">
                    <div className="modal-content">
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
                                <button type="button" onClick={closeModal}>Cancel</button>
                            </div>
                        </form>
                    </div>
                </div>
            )}
        </main>
    );
}

export default Feedback;