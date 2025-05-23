import {defineStore} from 'pinia';
import authService from '@/services/auth';
import {User} from 'oidc-client-ts';

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null as User | null,
    isAuthenticated: false,
    isLoading: true,
  }),

  getters: {
    // Get the user's name from the profile
    userName: (state) => state.user?.profile?.name || '',

    // Get the user's email from the profile
    userEmail: (state) => state.user?.profile?.email || '',
  },

  actions: {
    async initialize() {
      this.isLoading = true;
      try {
        await this.checkAuth();
      } catch (error) {
        console.error('Failed to initialize auth store:', error);
      } finally {
        this.isLoading = false;
      }
    },

    async checkAuth() {
      try {
        this.user = await authService.getUser();
        this.isAuthenticated = await authService.isAuthenticated();
      } catch (error) {
        console.error('Failed to check authentication:', error);
        this.user = null;
        this.isAuthenticated = false;
      }
    },

    async login() {
      try {
        await authService.login();
      } catch (error) {
        console.error('Login failed:', error);
        throw error;
      }
    },

    async handleLoginCallback() {
      this.isLoading = true;
      try {
        console.log('Processing login callback in auth store');
        const user = await authService.handleLoginCallback();
        console.log('Login callback successful, user:', user);
        this.user = user;
        this.isAuthenticated = true;
        return user;
      } catch (error) {
        console.error('Login callback failed:', error);
        this.user = null;
        this.isAuthenticated = false;
        throw error;
      } finally {
        this.isLoading = false;
      }
    },

    async logout() {
      try {
        await authService.logout();
        this.user = null;
        this.isAuthenticated = false;
      } catch (error) {
        console.error('Logout failed:', error);
        throw error;
      }
    },
  },
});
