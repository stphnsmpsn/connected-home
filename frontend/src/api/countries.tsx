import axios from "axios";

export function getCountries(token: string) {

    const address = "http://localhost:8082/api/countries"

    return axios.create({
        timeout: 45000,
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
        responseType: "json"
    }).get(address, {headers: {
        'Authorization': token
        }}).then(
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