import axios from "axios";
import { defineStore } from "pinia";
import authService from "@/services/auth";

interface CurrentUser {
    username: string;
    admin: boolean;
}

export const useAuthStore = defineStore("auth", {
    state: () => ({
        user: null as CurrentUser | null,
        isAuthenticated: false,
        isLoading: true,
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
    },
});
