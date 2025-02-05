import React, { useState } from 'react';
import '../styles/Feedback.css'; // Import the Feedback-specific CSS

function Feedback() {
    // State to manage modal visibility
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');

    // Function to open the modal
    const openModal = () => setIsModalOpen(true);

    // Function to close the modal
    const closeModal = () => setIsModalOpen(false);

    // Function to handle form submission (for feedback)
    const handleSubmit = (e) => {
        e.preventDefault();
        alert(`Feedback Submitted:\nTitle: ${title}\nDescription: ${description}`);
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

                <p>This section shows you your current feedback or feedback to be responded to.</p>
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
