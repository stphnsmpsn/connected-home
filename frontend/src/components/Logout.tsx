import React from 'react';
import useToken from "../UseToken";

const Logout: React.FC = () => {
    const {clearToken} = useToken();


    const logout = async () => {
        clearToken();
        window.location.href = "/";
    }

    return (
        <>
            {logout()}
        </>
    )
}

export default Logout;