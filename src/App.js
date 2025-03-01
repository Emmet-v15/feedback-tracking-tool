import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Header from './components/Header';
import Footer from './components/Footer';
import Home from './components/Home';
import Feedback from './components/Feedback';
import './index.css';

function App() {
    const [theme, setTheme] = useState(localStorage.getItem('theme') || 'light');

    // Apply theme to the body when it changes
    useEffect(() => {
        document.body.setAttribute('data-theme', theme);
        localStorage.setItem('theme', theme); // Save user preference
    }, [theme]);

    // Toggle between dark and light mode
    const toggleTheme = () => {
        setTheme((prevTheme) => (prevTheme === 'light' ? 'dark' : 'light'));
    };

    return (
        <Router>
            <Header toggleTheme={toggleTheme} theme={theme} />
            <main>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/feedback" element={<Feedback />} />
                </Routes>
            </main>
            <Footer />
        </Router>
    );
}

export default App;
