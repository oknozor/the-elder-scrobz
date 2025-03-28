import { UserManager, UserManagerSettings, User } from 'oidc-client-ts';

// Use runtime environment variables from window.env if available, otherwise fall back to import.meta.env
const env = (window as any).env || import.meta.env;

const oidcSettings: UserManagerSettings = {
  authority: env.VITE_OIDC_AUTHORITY,
  metadataUrl: env.VITE_OIDC_AUTHORITY,
  client_id: env.VITE_OIDC_CLIENT_ID,
  client_secret: env.VITE_OIDC_CLIENT_SECRET,
  redirect_uri: `${window.location.origin}/callback`,
  post_logout_redirect_uri: window.location.origin,
  response_type: 'code',
  scope: 'openid profile email',
  automaticSilentRenew: true,
  silent_redirect_uri: `${window.location.origin}/silent-renew.html`,
};

const userManager = new UserManager(oidcSettings);

(window as any).oidcUserManager = userManager;

export default {
  async getUser(): Promise<User | null> {
    return await userManager.getUser();
  },

  async isAuthenticated(): Promise<boolean> {
    const user = await userManager.getUser();
    return !!user && !user.expired;
  },

  async login(): Promise<void> {
    await userManager.signinRedirect();
  },

  async handleLoginCallback(): Promise<User> {
    try {
      console.log('Handling login callback in auth service');
      const user = await userManager.signinRedirectCallback();
      console.log('Login callback processed successfully');
      return user;
    } catch (error) {
      console.error('Error in signinRedirectCallback:', error);
      throw error;
    }
  },

  async logout(): Promise<void> {
    await userManager.signoutRedirect();
  },

  async getAccessToken(): Promise<string | null> {
    const user = await userManager.getUser();
    return user?.access_token || null;
  },
};
