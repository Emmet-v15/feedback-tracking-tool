import React, { useState } from 'react';
import "../styles/Login.css"; // Reuse the same Login.css for signup as well
import { validateSignup } from '../utils/Validation';
import { useNavigate } from 'react-router-dom';
import LockIcon from '../assets/Lock.svg';
import PersonIcon from '../assets/Person.svg';

const Signup = () => {
    const [firstname, setFirstname] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [repeatPassword, setRepeatPassword] = useState('');
    const [errorMessage, setErrorMessage] = useState('');
    const navigate = useNavigate();

    const handleSubmit = (e) => {
        e.preventDefault();
        const errors = validateSignup(firstname, email, password, repeatPassword);
        if (errors.length > 0) {
            setErrorMessage(errors.join('. '));
        } else {
            setErrorMessage('');
            navigate('/dashboard');
        }
    };

    return (
        <div className="login-page">  {/* Add this class here */}
            {/* Changed from "form-container" to "wrapper" to align with CSS */}
            <div className="wrapper">
                <h1>Signup</h1>
                <p className="error-message">{errorMessage}</p>
                <form onSubmit={handleSubmit}>
                    <div>
                        <label htmlFor="firstname-input">
                            <img src={PersonIcon} alt="Person Icon" className="icon" />
                        </label>
                        <input
                            type="text"
                            id="firstname-input"
                            placeholder="Firstname"
                            value={firstname}
                            onChange={(e) => setFirstname(e.target.value)}
                        />
                    </div>
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
                    <div>
                        <label htmlFor="repeat-password-input">
                            <img src={LockIcon} alt="Lock Icon" className="icon" />
                        </label>
                        <input
                            type="password"
                            id="repeat-password-input"
                            placeholder="Repeat Password"
                            value={repeatPassword}
                            onChange={(e) => setRepeatPassword(e.target.value)}
                        />
                    </div>
                    <button type="submit">Signup</button>
                </form>
                <p>Already have an Account? <a href="/login">Login</a></p>
            </div>
        </div>
    );
};

export default Signup;
