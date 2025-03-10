export const validateLogin = (email, password) => {
    let errors = [];

    if (!email.trim()) {
        errors.push('Email is required');
    }
    if (!password.trim()) {
        errors.push('Password is required');
    }

    return errors;
};

export const validateSignup = (firstname, email, password, repeatPassword) => {
    let errors = [];

    if (!firstname.trim()) {
        errors.push('Firstname is required');
    }
    if (!email.trim()) {
        errors.push('Email is required');
    }
    if (!password.trim()) {
        errors.push('Password is required');
    } else if (password.length < 8) {
        errors.push('Password must have at least 8 characters');
    }
    if (password !== repeatPassword) {
        errors.push('Password does not match repeated password');
    }

    return errors;
};
