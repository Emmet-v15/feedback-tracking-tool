import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Route, Routes, useLocation } from 'react-router-dom';
import Header from './components/Header';
import Footer from './components/Footer';
import Home from './components/Home';
import Feedback from './components/Feedback';
import LoginPage from './components/Login';
import SignupPage from './components/Signup';
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
            <AppContent toggleTheme={toggleTheme} theme={theme} />
        </Router>
    );
}

// AppContent renders Header, Footer, and Routes
function AppContent({ toggleTheme, theme }) {
    const location = useLocation();  // Get the current path

    // Check if the current path is login or signup
    const isAuthPage = location.pathname === '/login' || location.pathname === '/signup';

    return (
        <>
            {/* Conditionally render Header and Footer */}
            {!isAuthPage && <Header toggleTheme={toggleTheme} theme={theme} />}
            <main>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/feedback" element={<Feedback />} />
                    <Route path="/login" element={<LoginPage />} />
                    <Route path="/signup" element={<SignupPage />} />
                </Routes>
            </main>
            {!isAuthPage && <Footer />}
        </>
    );
}

export default App;
