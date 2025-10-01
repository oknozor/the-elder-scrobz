<template>
    <div class="scrobble-json-viewer">
        <div class="viewer-header">
            <h2>Scrobble JSON Viewer</h2>
            <p class="viewer-description">
                View raw scrobble data in JSON format with syntax highlighting
                and folding capabilities.
            </p>
        </div>

        <div class="viewer-content">
            <div class="search-form">
                <div class="search-field">
                    <label for="scrobble-id" class="search-label"
                        >Scrobble ID</label
                    >
                    <input
                        id="scrobble-id"
                        v-model="scrobbleId"
                        type="text"
                        class="search-input"
                        placeholder="Enter scrobble ID..."
                        @keyup.enter="fetchScrobble"
                    />
                </div>
                <button
                    type="button"
                    @click="fetchScrobble"
                    :disabled="!scrobbleId.trim() || isLoading"
                    class="search-button"
                >
                    <span class="button-content">
                        <svg
                            v-if="!isLoading"
                            class="search-icon"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z"
                                clip-rule="evenodd"
                            />
                        </svg>
                        <div v-if="isLoading" class="spinner"></div>
                        {{ isLoading ? "Loading..." : "Get Scrobble" }}
                    </span>
                </button>
            </div>

            <div v-if="error" class="error-message">
                <div class="error-icon">
                    <svg viewBox="0 0 20 20" fill="currentColor">
                        <path
                            fill-rule="evenodd"
                            d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-5a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5A.75.75 0 0110 5zM9.25 13a.75.75 0 011.5 0v.01a.75.75 0 01-1.5 0V13z"
                            clip-rule="evenodd"
                        />
                    </svg>
                </div>
                <div class="error-content">
                    <h4>Error</h4>
                    <p>{{ error }}</p>
                </div>
                <button @click="clearError" class="error-close">
                    <svg viewBox="0 0 20 20" fill="currentColor">
                        <path
                            d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z"
                        />
                    </svg>
                </button>
            </div>

            <div v-if="scrobbleData" class="json-container">
                <div class="json-header">
                    <h3>Scrobble ID: {{ scrobbleData.id }}</h3>
                    <div class="json-actions">
                        <button @click="toggleAll" class="toggle-button">
                            {{ allExpanded ? "Collapse All" : "Expand All" }}
                        </button>
                        <button @click="copyJson" class="copy-button">
                            <svg
                                class="copy-icon"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                            >
                                <path
                                    d="M7 3.5A1.5 1.5 0 0 1 8.5 2h3.879a1.5 1.5 0 0 1 1.06.44l3.122 3.12A1.5 1.5 0 0 1 17 6.622V12.5a1.5 1.5 0 0 1-1.5 1.5h-1v-3.379a3 3 0 0 0-.879-2.121L10.5 5.379A3 3 0 0 0 8.379 4.5H7v-1Z"
                                />
                                <path
                                    d="M4.5 6A1.5 1.5 0 0 0 3 7.5v9A1.5 1.5 0 0 0 4.5 18h7a1.5 1.5 0 0 0 1.5-1.5v-5.879a1.5 1.5 0 0 0-.44-1.06L9.44 6.439A1.5 1.5 0 0 0 8.378 6H4.5Z"
                                />
                            </svg>
                            {{ copySuccess ? "Copied!" : "Copy JSON" }}
                        </button>
                    </div>
                </div>
                <div class="json-viewer">
                    <JsonNode
                        :value="scrobbleData"
                        :path="[]"
                        :expanded="allExpanded"
                        @toggle="onToggle"
                    />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { reactive, ref } from "vue";
import { type RawScrobble, useAdminStore } from "@/stores/adminStore";
import JsonNode from "./JsonNode.vue";

const adminStore = useAdminStore();

const scrobbleId = ref("");
const scrobbleData = ref<RawScrobble | null>(null);
const isLoading = ref(false);
const error = ref("");
const allExpanded = ref(true);
const expandedPaths = reactive(new Set<string>());
const copySuccess = ref(false);

const fetchScrobble = async () => {
    if (!scrobbleId.value.trim()) return;

    isLoading.value = true;
    error.value = "";

    try {
        scrobbleData.value = await adminStore.fetchScrobbleById(
            scrobbleId.value.trim(),
        );
        allExpanded.value = true;
        expandedPaths.clear();
    } catch (err: unknown) {
        error.value = "Failed to fetch scrobble";
        scrobbleData.value = null;
    } finally {
        isLoading.value = false;
    }
};

const clearError = () => {
    error.value = "";
};

const toggleAll = () => {
    allExpanded.value = !allExpanded.value;
    expandedPaths.clear();
};

const onToggle = (path: string[]) => {
    const pathStr = path.join(".");
    if (expandedPaths.has(pathStr)) {
        expandedPaths.delete(pathStr);
    } else {
        expandedPaths.add(pathStr);
    }
};

const copyJson = async () => {
    if (!scrobbleData.value) return;

    try {
        await navigator.clipboard.writeText(
            JSON.stringify(scrobbleData.value, null, 2),
        );
        copySuccess.value = true;
        setTimeout(() => {
            copySuccess.value = false;
        }, 2000);
    } catch (err) {
        console.error("Failed to copy JSON:", err);
    }
};
</script>

<style scoped>
.scrobble-json-viewer {
    background: var(--card-background);
    border-radius: 12px;
    padding: 32px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
}

.viewer-header {
    text-align: center;
    margin-bottom: 24px;
}

.viewer-header h2 {
    color: var(--text-color);
    margin: 0 0 8px 0;
    font-size: 1.6em;
    font-weight: 600;
}

.viewer-description {
    color: var(--text-secondary);
    font-size: 1em;
    margin: 0;
    line-height: 1.5;
}

.viewer-content {
    display: flex;
    flex-direction: column;
    gap: 24px;
}

.search-form {
    display: flex;
    gap: 12px;
    align-items: end;
}

.search-field {
    flex: 1;
}

.search-label {
    display: block;
    color: var(--text-color);
    font-weight: 500;
    margin-bottom: 8px;
    font-size: 0.9em;
}

.search-input {
    width: 100%;
    padding: 12px 16px;
    border: 2px solid var(--border-color);
    border-radius: 8px;
    background: var(--input-background, var(--card-background));
    color: var(--text-color);
    font-size: 1em;
    transition: border-color 0.2s ease;
}

.search-input:focus {
    outline: none;
    border-color: var(--primary-color, #3b82f6);
}

.search-button {
    padding: 12px 24px;
    background: var(--primary-color, #3b82f6);
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 48px;
}

.search-button:hover:not(:disabled) {
    background: var(--primary-color-hover, #2563eb);
    transform: translateY(-1px);
}

.search-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
}

.button-content {
    display: flex;
    align-items: center;
    gap: 8px;
}

.search-icon {
    width: 18px;
    height: 18px;
}

.spinner {
    width: 18px;
    height: 18px;
    border: 2px solid transparent;
    border-top: 2px solid currentColor;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    to {
        transform: rotate(360deg);
    }
}

.error-message {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 16px;
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 8px;
    color: #dc2626;
}

.error-icon {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
    margin-top: 2px;
}

.error-content {
    flex: 1;
}

.error-content h4 {
    margin: 0 0 4px 0;
    font-weight: 600;
    font-size: 1em;
}

.error-content p {
    margin: 0;
    font-size: 0.9em;
    opacity: 0.9;
}

.error-close {
    background: none;
    border: none;
    padding: 4px;
    cursor: pointer;
    color: inherit;
    border-radius: 4px;
    transition: background-color 0.2s ease;
}

.error-close:hover {
    background: rgba(0, 0, 0, 0.1);
}

.error-close svg {
    width: 16px;
    height: 16px;
}

.json-container {
    border: 1px solid var(--border-color);
    border-radius: 8px;
    overflow: hidden;
}

.json-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    background: var(--background-secondary, rgba(0, 0, 0, 0.1));
    border-bottom: 1px solid var(--border-color);
}

.json-header h3 {
    margin: 0;
    color: var(--text-color);
    font-size: 1.1em;
    font-weight: 600;
}

.json-actions {
    display: flex;
    gap: 12px;
}

.toggle-button,
.copy-button {
    padding: 8px 16px;
    background: var(--button-background, rgba(59, 130, 246, 0.1));
    color: var(--primary-color, #3b82f6);
    border: 1px solid var(--primary-color, #3b82f6);
    border-radius: 6px;
    font-size: 0.85em;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.copy-button {
    display: flex;
    align-items: center;
    gap: 6px;
}

.copy-icon {
    width: 14px;
    height: 14px;
}

.toggle-button:hover,
.copy-button:hover {
    background: var(--primary-color, #3b82f6);
    color: white;
}

.json-viewer {
    padding: 16px;
    background: var(--code-background, #1a1a1a);
    color: var(--code-text, #e5e5e5);
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
    font-size: 0.85em;
    line-height: 1.5;
    overflow-x: auto;
}

@media (max-width: 768px) {
    .search-form {
        flex-direction: column;
        align-items: stretch;
    }

    .json-header {
        flex-direction: column;
        align-items: stretch;
        gap: 12px;
    }

    .json-actions {
        justify-content: stretch;
    }

    .toggle-button,
    .copy-button {
        flex: 1;
        text-align: center;
    }
}
</style>
