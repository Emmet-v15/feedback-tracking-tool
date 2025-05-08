import React from 'react';
import { Link, useLocation } from 'react-router-dom';

function Header({ toggleTheme, theme }) {
    const location = useLocation();

    return (
        <header>
            <h1>Feedback Tracking Tool</h1>
            <nav>
                <ul>
                    <li>
                        <Link to="/" className={location.pathname === "/" ? "active" : ""}>
                            Home
                        </Link>
                    </li>
                    <li>
                        <Link to="/feedback" className={location.pathname === "/feedback" ? "active" : ""}>
                            Feedback
                        </Link>
                    </li>
                    <li>
                        <button className="theme-toggle" onClick={toggleTheme}>
                            {theme === 'light' ? '🌙 Dark Mode' : '☀️ Light Mode'}
                        </button>
                    </li>
                </ul>
            </nav>
        </header>
    );
}

export default Header;
