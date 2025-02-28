import React from 'react';
import '../styles/Home.css'; // Import the Home CSS


function Home() {
    return (
        <main>
            <section className="page-header">
                <h2>Home</h2>
            </section>

            <section className="title">
                <h3>How it works</h3>
            </section>
            
            <section className="description">
                <p>You create feedback about problems around campus. These problems are then seen and responded too by a member of staff as soon as possible. Hopefully getting your problem resolved.</p>
            </section>

            <section className="title">
                <h3>How to create feedback</h3>
            </section>

            <section className="description">
                <p>You can create feedback by first clicking on the "feedback" button in the navigation bar at the top of the page.</p>
                <p>Once on that page you are able too create feedback using the green button in the top right hand corner. Once you have submitted feedback, a member of staff will respond to is as soon as possible</p>
                <p>You can track how long your feedback has been open with the timer on the right hand side of your feedback</p>
            </section>

            <section className="gap">
            </section>
            
        </main>
    );
}

export default Home;
