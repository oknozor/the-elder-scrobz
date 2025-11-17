import { defineStore } from "pinia";
import apiClient from "@/services/api";
import type { PaginatedResponse } from "@/types";
import { UserConfig } from "@/types/user/config";

export interface AppUser {
    username: string;
    email: string;
}

export const useUsersStore = defineStore("users", {
    state: () => ({
        users: [] as AppUser[],
        selectedUser: null as AppUser | null,
        userConfig: null as UserConfig | null,
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
        async fetchUserConfig() {
            const { data } = await apiClient.get<UserConfig>("users/config");
            this.userConfig = data;
            return data;
        },
        async updateUserConfig(config: UserConfig) {
            const { data } = await apiClient.post<UserConfig>(
                "users/config",
                config,
            );
            this.userConfig = data;
            return data;
        },
    },
});
