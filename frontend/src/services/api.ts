import axios from 'axios';

const API_BASE_URL =
	import.meta.env.VITE_API_BASE_URL || 'https://the-elder-scrobz.hoohoot.org';

const apiClient = axios.create({
	baseURL: API_BASE_URL,
	timeout: 5000,
	headers: {
		'Content-Type': 'application/json',
	},
});

export default apiClient;
