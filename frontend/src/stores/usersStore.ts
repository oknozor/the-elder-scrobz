import apiClient from '@/services/api';
import { defineStore } from 'pinia';

export interface AppUser {
	username: string;
	email: string;
}

export const useUsersStore = defineStore('users', {
	state: () => ({
		users: [] as AppUser[],
		selectedUser: null as AppUser | null,
	}),
	actions: {
		async fetchUsers() {
			const { data } = await apiClient.get<AppUser[]>('/users');
			this.users = data;
		},
		updateSelectedUser(user: AppUser | null) {
			this.selectedUser = user;
		},
	},
});
