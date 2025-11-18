<template>
    <div class="config-form">
        <div class="form-header">
            <h2>Playlist Configuration</h2>
            <p class="form-description">
                Manage personal settings for playlist generation and
                preferences.
            </p>
        </div>

        <form @submit.prevent="handleSave" class="config-form-content">
            <div class="form-section">
                <h3>Playlist Preferences</h3>
                <div class="checkbox-group">
                    <label class="checkbox-item">
                        <input type="checkbox" v-model="config.enable_weekly_playlist" :disabled="isSaving"
                            class="checkbox-input" />
                        <span class="checkbox-label">
                            <strong>Enable Weekly Playlists</strong>
                            <span class="checkbox-description">
                                Automatically generate weekly playlists based on
                                your listening habits
                            </span>
                        </span>
                    </label>

                    <label class="checkbox-item">
                        <input type="checkbox" v-model="config.enable_monthly_playlist" :disabled="isSaving"
                            class="checkbox-input" />
                        <span class="checkbox-label">
                            <strong>Enable Monthly Playlists</strong>
                            <span class="checkbox-description">
                                Automatically generate monthly playlists based
                                on your listening habits
                            </span>
                        </span>
                    </label>

                    <label class="checkbox-item">
                        <input type="checkbox" v-model="config.enable_yearly_playlist" :disabled="isSaving"
                            class="checkbox-input" />
                        <span class="checkbox-label">
                            <strong>Enable Yearly Playlists</strong>
                            <span class="checkbox-description">
                                Automatically generate yearly playlists based on
                                your listening habits
                            </span>
                        </span>
                    </label>
                </div>
            </div>

            <div class="form-actions">
                <button type="submit" class="save-button" :disabled="isSaving" :class="{ saving: isSaving }">
                    <div v-if="isSaving" class="saving-content">
                        <div class="spinner"></div>
                        <span>Saving...</span>
                    </div>
                    <div v-else class="button-content">
                        <svg class="save-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
                            <polyline points="17,21 17,13 7,13 7,21" />
                            <polyline points="7,3 7,8 15,8" />
                        </svg>
                        <span>Save Configuration</span>
                    </div>
                </button>
            </div>

            <div v-if="result" class="result" :class="result.type">
                <div class="result-icon">
                    <svg v-if="result.type === 'success'" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2">
                        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
                        <polyline points="22,4 12,14.01 9,11.01" />
                    </svg>
                    <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="12" cy="12" r="10" />
                        <line x1="15" y1="9" x2="9" y2="15" />
                        <line x1="9" y1="9" x2="15" y2="15" />
                    </svg>
                </div>
                <div class="result-content">
                    <p>{{ result.message }}</p>
                </div>
                <button class="result-close" @click="clearResult">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="6" x2="6" y2="18" />
                        <line x1="6" y1="6" x2="18" y2="18" />
                    </svg>
                </button>
            </div>
        </form>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useAuthStore } from "@/stores/authStore";
import { useUsersStore } from "@/stores/usersStore";
import { UserConfig } from "@/types/user/config";

const usersStore = useUsersStore();
const authStore = useAuthStore();

const config = ref<UserConfig>({
    username: authStore.user?.username || "",
    enable_weekly_playlist: false,
    enable_monthly_playlist: false,
    enable_yearly_playlist: false,
});

const isSaving = ref(false);

interface Result {
    type: "success" | "error";
    message: string;
}

const result = ref<Result | null>(null);

onMounted(async () => {
    try {
        const userConfig = await usersStore.fetchUserConfig();
        config.value = { ...config.value, ...userConfig };
    } catch (error) {
        console.error("Failed to load user config:", error);
        result.value = {
            type: "error",
            message:
                "Failed to load configuration. Please try refreshing the page.",
        };
    }
});

const handleSave = async () => {
    try {
        isSaving.value = true;
        result.value = null;

        await usersStore.updateUserConfig(config.value);

        result.value = {
            type: "success",
            message: "Configuration updated successfully.",
        };
    } catch (error) {
        console.error("Failed to update user config:", error);
        result.value = {
            type: "error",
            message: "Failed to update configuration. Please try again.",
        };
    } finally {
        isSaving.value = false;
    }
};

const clearResult = () => {
    result.value = null;
};
</script>

<style scoped>
.config-form {
    max-width: 800px;
    margin: 0 auto;
}

.form-header {
    margin-bottom: 32px;
    text-align: center;
}

.form-header h2 {
    color: var(--text-color);
    margin: 0 0 8px 0;
    font-size: 1.8em;
}

.form-description {
    color: var(--text-secondary);
    font-size: 1.1em;
    margin: 0;
}

.config-form-content {
    background: var(--card-background);
    border-radius: 8px;
    padding: 24px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
    margin-bottom: 24px;
}

.form-section {
    margin-bottom: 32px;
}

.form-section:last-child {
    margin-bottom: 0;
}

.form-section h3 {
    color: var(--text-color);
    margin: 0 0 16px 0;
    font-size: 1.2em;
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 8px;
}

.checkbox-group {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.checkbox-item {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    cursor: pointer;
    padding: 12px;
    border-radius: 6px;
    transition: background-color 0.2s;
}

.checkbox-item:hover:not(:has(.checkbox-input:disabled)) {
    background: rgba(255, 255, 255, 0.05);
}

.checkbox-input {
    width: 18px;
    height: 18px;
    margin: 0;
    cursor: pointer;
    accent-color: var(--primary-color);
}

.checkbox-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.checkbox-label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
}

.checkbox-label strong {
    color: var(--text-color);
    font-size: 0.95em;
}

.checkbox-description {
    color: var(--text-secondary);
    font-size: 0.85em;
    line-height: 1.4;
}

.form-actions {
    display: flex;
    gap: 12px;
    justify-content: center;
    flex-wrap: wrap;
    margin-top: 32px;
}

.save-button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 12px 24px;
    background: var(--primary-color);
    color: var(--background-color);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 1em;
    font-weight: 500;
    transition: all 0.2s;
    min-width: 180px;
    min-height: 48px;
}

.save-button:hover:not(:disabled) {
    opacity: 0.9;
    transform: translateY(-1px);
}

.save-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
}

.save-button.saving {
    background: var(--text-secondary);
}

.button-content,
.saving-content {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
}

.save-icon {
    width: 20px;
    height: 20px;
}

.spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% {
        transform: rotate(0deg);
    }

    100% {
        transform: rotate(360deg);
    }
}

.result {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 16px;
    border-radius: 6px;
    margin-top: 16px;
    position: relative;
}

.result.success {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.3);
}

.result.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
}

.result-icon {
    flex-shrink: 0;
    width: 20px;
    height: 20px;
    margin-top: 2px;
}

.result.success .result-icon {
    color: #22c55e;
}

.result.error .result-icon {
    color: #ef4444;
}

.result-content {
    flex: 1;
}

.result-content p {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.9em;
    line-height: 1.4;
}

.result-close {
    background: none;
    border: none;
    padding: 4px;
    cursor: pointer;
    color: var(--text-secondary);
    transition: color 0.2s;
    flex-shrink: 0;
}

.result-close:hover {
    color: var(--text-color);
}

.result-close svg {
    width: 16px;
    height: 16px;
}

@media (max-width: 600px) {
    .config-form {
        padding: 0 16px;
    }

    .form-actions {
        flex-direction: column;
        align-items: center;
    }

    .save-button {
        width: 100%;
        max-width: 300px;
    }
}
</style>
