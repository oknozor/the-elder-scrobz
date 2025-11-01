import axios from "axios";

const apiClient = axios.create({
    baseURL: "/api/v1",
    timeout: 5000,
    headers: {
        "Content-Type": "application/json",
    },
    withCredentials: true,
});

export default apiClient;
