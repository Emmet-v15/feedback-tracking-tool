import React from 'react';
import { Link, useLocation } from 'react-router-dom';

function Header() {
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
                </ul>
            </nav>
        </header>
    );
}

export default Header;
