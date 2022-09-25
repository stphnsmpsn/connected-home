import axios from "axios";

export function register(username: string | undefined, password: string | undefined) {

    const address = `${process.env.REACT_APP_BACKEND_URL}/api/register`;
    console.log(address);
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
            if (status !== 201) {
                throw new Error("Server exception");
            }
            return data;
        },
        err => {
            throw err;
        }
    );
}