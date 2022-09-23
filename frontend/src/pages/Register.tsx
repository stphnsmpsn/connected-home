import React, {useState} from 'react';
import './Register.css';
import PropTypes from 'prop-types';
import {register} from "../api/register";

export default function Register({setToken}: { setToken: any }) {

    const [username, setUserName] = useState<string>();
    const [password, setPassword] = useState<string>();

    const handleSubmit = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        const token = await register(username, password);
        setToken(token);
        window.location.href = "/dashboard";
    }

    return (
        <div className="register-wrapper">
            <h1>Please Register</h1>
            <form onSubmit={handleSubmit}>
                <label>
                    <p>Username</p>
                    <input type="text" onChange={e => setUserName(e.target.value)}/>
                </label>
                <label>
                    <p>Password</p>
                    <input type="password" onChange={e => setPassword(e.target.value)}/>
                </label>
                <div>
                    <br/>
                    <button type="submit">Submit</button>
                </div>
            </form>
        </div>
    )
}

Register.propTypes = {
    setToken: PropTypes.func.isRequired
};