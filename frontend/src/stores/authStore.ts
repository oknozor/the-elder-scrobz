import axios from "axios";
import { defineStore } from "pinia";
import authService from "@/services/auth";
import type { UserConfig } from "@/types/user/config";

interface CurrentUser {
    username: string;
    admin: boolean;
}

export const useAuthStore = defineStore("auth", {
    state: () => ({
        user: null as CurrentUser | null,
        isAuthenticated: false,
        isLoading: true,
        config: {} as UserConfig,
    }),

    getters: {
        userName: (state) => state.user?.username || "",
        isAdmin: (state) => state.user?.admin || false,
    },

    actions: {
        async initialize() {
            this.isLoading = true;
            try {
                await this.checkAuth();
            } catch (error) {
                this.user = null;
                this.isAuthenticated = false;
            } finally {
                this.isLoading = false;
            }
        },

        async checkAuth() {
            try {
                const response =
                    await axios.get<CurrentUser>("/api/v1/users/me");
                this.user = response.data;
                this.isAuthenticated = true;
            } catch (error) {
                this.user = null;
                this.isAuthenticated = false;
                throw error;
            }
        },

        login() {
            authService.login();
        },

        logout() {
            authService.logout();
        },

        async fetchUserConfig() {
            try {
                const response = await axios.get("/api/v1/users/config");
                this.config = response.data;
            } catch (error) {
                console.error("Error fetching user config:", error);
                throw error;
            }
        },

        async updateUserConfig(config: UserConfig) {
            try {
                const response = await axios.post(
                    "/api/v1/users/config",
                    config,
                );
                this.config = response.data;
                return response.data;
            } catch (error) {
                console.error("Error updating user config:", error);
                throw error;
            }
        },
    },
});
