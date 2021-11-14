import axios from "axios";

export function login(token: string) {

    const address = "http://localhost:8082/dashboard"

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