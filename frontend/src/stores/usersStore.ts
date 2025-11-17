import { defineStore } from "pinia";
import apiClient from "@/services/api";
import type { PaginatedResponse } from "@/types";

export interface AppUser {
    username: string;
    email: string;
}

export interface UserConfig {
    username: string;
    enable_weekly_playlist: boolean;
    enable_monthly_playlist: boolean;
    enable_yearly_playlist: boolean;
    email_notifications: boolean;
    public_profile: boolean;
    share_listening_data: boolean;
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
            const { data } = await apiClient.get<UserConfig>("/config");
            this.userConfig = data;
            return data;
        },
        async updateUserConfig(config: UserConfig) {
            const { data } = await apiClient.post<UserConfig>("/config", config);
            this.userConfig = data;
            return data;
        },
    },
});
