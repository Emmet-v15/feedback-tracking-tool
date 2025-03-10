import React, { useState } from 'react';
import "../styles/Login.css"; // Make sure the Login.css is imported
import { validateLogin } from '../utils/Validation';
import { useNavigate } from 'react-router-dom';
import LockIcon from '../assets/Lock.svg';

const Login = () => {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [errorMessage, setErrorMessage] = useState('');
    const navigate = useNavigate();

    const handleSubmit = (e) => {
        e.preventDefault();
        const errors = validateLogin(email, password);
        if (errors.length > 0) {
            setErrorMessage(errors.join('. '));
        } else {
            setErrorMessage('');
            navigate('/dashboard');
        }
    };

    return (
        <div className="login-page">
            {/* Changed from "form-container" to "wrapper" to align with CSS */}
            <div className="wrapper">
                <h1>Login</h1>
                <p className="error-message">{errorMessage}</p>
                <form onSubmit={handleSubmit}>
                    <div>
                        <label htmlFor="email-input">@</label>
                        <input
                            type="email"
                            id="email-input"
                            placeholder="Email"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                        />
                    </div>
                    <div>
                        <label htmlFor="password-input">
                            <img src={LockIcon} alt="Lock Icon" className="icon" />
                        </label>
                        <input
                            type="password"
                            id="password-input"
                            placeholder="Password"
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                        />
                    </div>
                    <button type="submit">Login</button>
                </form>
                <p>New here? <a href="/signup">Create an Account</a></p>
            </div>
        </div>
    );
};

export default Login;
