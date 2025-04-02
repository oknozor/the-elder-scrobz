import type { ApiKey } from '@/types/music';
import apiClient from './api';

export const apiKeyService = {
	async createApiKey(label: string): Promise<ApiKey> {
		console.log('Creating API key with label:', label);
		const response = await apiClient.post(`/users/api-keys`, { label });
		return response.data;
	},

	async listApiKeys(): Promise<ApiKey[]> {
		const response = await apiClient.get(`/users/api-keys`);
		return response.data;
	},

	async deleteApiKey(apiKeyId: string): Promise<void> {
		await apiClient.delete(`/users/api-keys/${apiKeyId}`);
	},
};
