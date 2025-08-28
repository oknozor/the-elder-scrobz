import { defineStore } from "pinia";
import apiClient from "@/services/api";
import type { PaginatedResponse } from "@/types/music.ts";

export interface AppUser {
    username: string;
    email: string;
}

export const useUsersStore = defineStore("users", {
    state: () => ({
        users: [] as AppUser[],
        selectedUser: null as AppUser | null,
    }),
    actions: {
        async fetchUsers() {
            const { data } = await apiClient
                .get<PaginatedResponse<AppUser>>("/users")
                .then((res) => res.data);
            this.users = data;
        },
        updateSelectedUser(user: AppUser | null) {
            this.selectedUser = user;
        },
    },
});
