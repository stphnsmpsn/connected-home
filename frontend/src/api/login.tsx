import axios from "axios";

export function login(username: string | undefined, password: string | undefined) {

    const address = `${process.env.REACT_APP_BACKEND_URL}/api/login`;
    let credentials = {
        username,
        password
    };

    return axios.create({
        timeout: 45000,
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        responseType: "json"
    }).post(address, credentials).then(
        ({ data, status }) => {
            if (status !== 200) {
                throw new Error("Server exception");
            }
            return data;
        },
        err => {
            throw err;
        }
    );
}