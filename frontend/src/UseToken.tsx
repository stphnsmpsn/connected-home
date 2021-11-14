import { useState } from 'react';

export default function useToken() {
    const getToken = () => {
        const tokenString = localStorage.getItem('token');
        if(tokenString) {
            const userToken = JSON.parse(tokenString);
            return userToken?.token
        }
        return null
    };

    const [token, setToken] = useState(getToken());

    const saveToken = (userToken: { token: any | string; }) => {
        localStorage.setItem('token', JSON.stringify(userToken));
        setToken(userToken.token);
    };

    const clearToken = () => {
        localStorage.removeItem('token');
    };

    return {
        setToken: saveToken,
        clearToken: clearToken,
        token
    }
}