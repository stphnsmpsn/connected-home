import axios from "axios";

export function login(token: string) {

    const address = `${process.env.REACT_APP_BACKEND_URL}/dashboard`;

    return axios.create({
        timeout: 45000,
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
        responseType: "json"
    }).post(address, {token}).then(
        ({ data, status }) => {
            if (status !== 200) {
                throw new Error("Server exception");
            }
            console.log(data);
            return data;
        },
        err => {
            throw err;
        }
    );
}