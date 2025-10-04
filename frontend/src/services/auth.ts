import {
    type User,
    UserManager,
    type UserManagerSettings,
} from "oidc-client-ts";

interface Window {
    oidcUserManager?: UserManager;
    env?: ImportMetaEnv;
}

const env = (window as Window).env || import.meta.env;

const oidcSettings: UserManagerSettings = {
    authority: env.VITE_OIDC_AUTHORITY,
    metadataUrl: env.VITE_OIDC_AUTHORITY,
    client_id: env.VITE_OIDC_CLIENT_ID,
    client_secret: env.VITE_OIDC_CLIENT_SECRET,
    redirect_uri: `${window.location.origin}/callback`,
    post_logout_redirect_uri: window.location.origin,
    response_type: "code",
    scope: "openid profile email offline_access scrobz_role",
    automaticSilentRenew: true,
    silent_redirect_uri: `${window.location.origin}/silent-renew.html`,
};

const userManager = new UserManager(oidcSettings);

userManager.events.addAccessTokenExpired(async () => {
    console.warn("Access token expired, trying silent renew...");
    try {
        await userManager.signinSilent();
        console.log("Silent renew succeeded");
    } catch (err) {
        console.error("Silent renew failed", err);
    }
});

(window as Window).oidcUserManager = userManager;

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
            console.log("Handling login callback in auth service");
            const user = await userManager.signinRedirectCallback();
            console.log("Login callback processed successfully");
            return user;
        } catch (error) {
            console.error("Error in signinRedirectCallback:", error);
            throw error;
        }
    },

    async logout(): Promise<void> {
        await userManager.signoutRedirect();
    },

    async getAccessToken(): Promise<string | null> {
        let user = await userManager.getUser();

        if (!user || user.expired) {
            console.log("Access token expired, trying silent renew...");
            try {
                user = await userManager.signinSilent();
            } catch (err) {
                console.error("Silent renew failed:", err);
                return null;
            }
        }

        return user?.access_token || null;
    },

    async getUserRole(): Promise<string | null> {
        const user = await userManager.getUser();
        if (!user || user.expired) {
            return null;
        }

        return (user.profile?.scrobz_role as string) || null;
    },

    async isAdmin(): Promise<boolean> {
        const role = await this.getUserRole();
        return role === "admin";
    },
};
