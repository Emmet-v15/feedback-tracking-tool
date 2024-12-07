import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Header from './components/Header';
import Footer from './components/Footer';
import Home from './components/Home';
import Feedback from './components/Feedback';
import './index.css'; 

function App() {
    return (
        <Router>
            <Header />
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
