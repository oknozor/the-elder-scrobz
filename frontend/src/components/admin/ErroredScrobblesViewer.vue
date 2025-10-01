<template>
    <div class="errored-scrobbles-viewer">
        <div class="viewer-header">
            <h2>Errored Scrobbles</h2>
            <p class="viewer-description">
                View and inspect scrobbles that failed to process with detailed
                error information.
            </p>
        </div>

        <div class="viewer-content">
            <div class="pagination-controls">
                <div class="pagination-info">
                    <span class="page-info">
                        Page {{ currentPage }} • {{ itemsPerPage }} items per
                        page
                    </span>
                </div>
                <div class="pagination-actions">
                    <label for="items-per-page" class="per-page-label"
                        >Items per page:</label
                    >
                    <select
                        id="items-per-page"
                        v-model="itemsPerPage"
                        @change="fetchScrobbles"
                        class="per-page-select"
                    >
                        <option value="10">10</option>
                        <option value="20">20</option>
                        <option value="50">50</option>
                        <option value="100">100</option>
                    </select>
                    <button
                        @click="refreshData"
                        :disabled="isLoading"
                        class="refresh-button"
                    >
                        <svg
                            class="refresh-icon"
                            :class="{ spinning: isLoading }"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z"
                                clip-rule="evenodd"
                            />
                        </svg>
                        Refresh
                    </button>
                </div>
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

            <div class="scrobbles-container">
                <div v-if="isLoading" class="loading-state">
                    <div class="spinner-large"></div>
                    <p>Loading errored scrobbles...</p>
                </div>

                <div
                    v-else-if="erroredScrobbles.length === 0 && !error"
                    class="empty-state"
                >
                    <div class="empty-icon">
                        <svg viewBox="0 0 20 20" fill="currentColor">
                            <path
                                fill-rule="evenodd"
                                d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z"
                                clip-rule="evenodd"
                            />
                        </svg>
                    </div>
                    <h3>No errored scrobbles found</h3>
                    <p>All scrobbles have been processed successfully.</p>
                </div>

                <div v-else class="scrobbles-list">
                    <div
                        v-for="scrobble in erroredScrobbles"
                        :key="scrobble.id"
                        class="scrobble-item"
                    >
                        <div class="scrobble-header">
                            <div class="scrobble-meta">
                                <h4 class="scrobble-id">
                                    ID: {{ scrobble.id }}
                                </h4>
                                <div class="scrobble-info">
                                    <span class="user-id"
                                        >User: {{ scrobble.user_id }}</span
                                    >
                                    <span class="created-at">{{
                                        formatDate(scrobble.created_at)
                                    }}</span>
                                </div>
                            </div>
                            <div class="scrobble-actions">
                                <button
                                    @click="toggleScrobble(scrobble.id)"
                                    class="toggle-json-button"
                                    :class="{
                                        expanded: expandedScrobbles.has(
                                            scrobble.id,
                                        ),
                                    }"
                                >
                                    <svg
                                        class="toggle-arrow"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    {{
                                        expandedScrobbles.has(scrobble.id)
                                            ? "Hide"
                                            : "Show"
                                    }}
                                    JSON
                                </button>
                                <button
                                    @click="copyScrobbleJson(scrobble)"
                                    class="copy-button"
                                >
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
                                    {{
                                        copiedScrobbles.has(scrobble.id)
                                            ? "Copied!"
                                            : "Copy"
                                    }}
                                </button>
                            </div>
                        </div>

                        <div
                            v-if="expandedScrobbles.has(scrobble.id)"
                            class="json-viewer"
                        >
                            <JsonNode
                                :value="scrobble.data"
                                :path="[]"
                                :expanded="true"
                                @toggle="onJsonToggle"
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div v-if="erroredScrobbles.length > 0" class="pagination-footer">
                <button
                    @click="previousPage"
                    :disabled="currentPage === 1 || isLoading"
                    class="pagination-button"
                >
                    <svg
                        class="pagination-icon"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M12.79 5.23a.75.75 0 01-.02 1.06L8.832 10l3.938 3.71a.75.75 0 11-1.04 1.08l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 011.06.02z"
                            clip-rule="evenodd"
                        />
                    </svg>
                    Previous
                </button>

                <div class="pagination-pages">
                    <button
                        v-for="page in visiblePages"
                        :key="page"
                        @click="goToPage(page)"
                        :disabled="isLoading"
                        class="page-button"
                        :class="{
                            active: page === currentPage,
                            ellipsis: page === '...',
                        }"
                    >
                        {{ page }}
                    </button>
                </div>

                <button
                    @click="nextPage"
                    :disabled="
                        erroredScrobbles.length < itemsPerPage || isLoading
                    "
                    class="pagination-button"
                >
                    Next
                    <svg
                        class="pagination-icon"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                            clip-rule="evenodd"
                        />
                    </svg>
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { isAxiosError } from "axios";
import { computed, onMounted, reactive, ref } from "vue";
import { type ErroredScrobble, useAdminStore } from "@/stores/adminStore";
import JsonNode from "./JsonNode.vue";

const adminStore = useAdminStore();

const currentPage = ref(1);
const itemsPerPage = ref(20);
const isLoading = ref(false);
const error = ref("");
const expandedScrobbles = reactive(new Set<number>());
const copiedScrobbles = reactive(new Set<number>());

const erroredScrobbles = computed(() => adminStore.erroredScrobbles);

const visiblePages = computed(() => {
    const pages = [];
    const totalPages = Math.ceil(100 / itemsPerPage.value); // Rough estimate since we don't know total
    const current = currentPage.value;

    // Always show first page
    if (current > 3) {
        pages.push(1);
        if (current > 4) {
            pages.push("...");
        }
    }

    // Show pages around current
    for (
        let i = Math.max(1, current - 1);
        i <= Math.min(totalPages, current + 1);
        i++
    ) {
        pages.push(i);
    }

    // Show last pages
    if (current < totalPages - 2) {
        if (current < totalPages - 3) {
            pages.push("...");
        }
        pages.push(totalPages);
    }

    return pages.filter((page, index, array) => array.indexOf(page) === index);
});

const fetchScrobbles = async () => {
    isLoading.value = true;
    error.value = "";

    try {
        await adminStore.fetchErroredScrobbles(
            currentPage.value,
            itemsPerPage.value,
        );
    } catch (e: unknown) {
        if (isAxiosError(error)) {
            console.error(
                "Failed to fetch scrobbles:",
                error.response?.data || error.message,
            );
        } else {
            console.error("Unexpected error:", error);
        }
    } finally {
        isLoading.value = false;
    }
};

const refreshData = () => {
    fetchScrobbles();
};

const clearError = () => {
    error.value = "";
};

const previousPage = () => {
    if (currentPage.value > 1) {
        currentPage.value--;
        fetchScrobbles();
    }
};

const nextPage = () => {
    currentPage.value++;
    fetchScrobbles();
};

const goToPage = (page: number | string) => {
    if (typeof page === "number" && page !== currentPage.value) {
        currentPage.value = page;
        fetchScrobbles();
    }
};

const toggleScrobble = (scrobbleId: number) => {
    if (expandedScrobbles.has(scrobbleId)) {
        expandedScrobbles.delete(scrobbleId);
    } else {
        expandedScrobbles.add(scrobbleId);
    }
};

const copyScrobbleJson = async (scrobble: ErroredScrobble) => {
    try {
        await navigator.clipboard.writeText(
            JSON.stringify(scrobble.data, null, 2),
        );
        copiedScrobbles.add(scrobble.id);
        setTimeout(() => {
            copiedScrobbles.delete(scrobble.id);
        }, 2000);
    } catch (err) {
        console.error("Failed to copy JSON:", err);
    }
};

const onJsonToggle = (path: string[]) => {
    // Handle JSON node expansion - could be implemented for per-node state
    console.log("JSON node toggled:", path);
};

const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleString();
};

onMounted(() => {
    fetchScrobbles();
});
</script>

<style scoped>
.errored-scrobbles-viewer {
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

.pagination-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    background: var(--background-secondary, rgba(0, 0, 0, 0.05));
    border-radius: 8px;
    border: 1px solid var(--border-color);
}

.pagination-info .page-info {
    color: var(--text-secondary);
    font-size: 0.9em;
}

.pagination-actions {
    display: flex;
    align-items: center;
    gap: 12px;
}

.per-page-label {
    color: var(--text-color);
    font-size: 0.9em;
    font-weight: 500;
}

.per-page-select {
    padding: 6px 12px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--input-background, var(--card-background));
    color: var(--text-color);
    font-size: 0.9em;
}

.refresh-button {
    display: flex;
    align-items: center;
    gap: 6px;
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

.refresh-button:hover:not(:disabled) {
    background: var(--primary-color, #3b82f6);
    color: white;
}

.refresh-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.refresh-icon {
    width: 16px;
    height: 16px;
    transition: transform 0.6s ease;
}

.refresh-icon.spinning {
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

.scrobbles-container {
    min-height: 400px;
}

.loading-state,
.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    color: var(--text-secondary);
}

.spinner-large {
    width: 40px;
    height: 40px;
    border: 3px solid var(--border-color);
    border-top: 3px solid var(--primary-color, #3b82f6);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 16px;
}

.empty-state .empty-icon {
    width: 48px;
    height: 48px;
    margin-bottom: 16px;
    opacity: 0.5;
}

.empty-state h3 {
    margin: 0 0 8px 0;
    color: var(--text-color);
    font-size: 1.2em;
    font-weight: 600;
}

.empty-state p {
    margin: 0;
    font-size: 0.95em;
}

.scrobbles-list {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.scrobble-item {
    border: 1px solid var(--border-color);
    border-radius: 8px;
    overflow: hidden;
    background: var(--background-secondary, rgba(0, 0, 0, 0.02));
}

.scrobble-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    background: var(--card-background);
    border-bottom: 1px solid var(--border-color);
}

.scrobble-meta h4 {
    margin: 0 0 8px 0;
    color: var(--text-color);
    font-size: 1em;
    font-weight: 600;
}

.scrobble-info {
    display: flex;
    gap: 16px;
    font-size: 0.85em;
    color: var(--text-secondary);
}

.scrobble-actions {
    display: flex;
    gap: 8px;
}

.toggle-json-button,
.copy-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--card-background);
    color: var(--text-color);
    font-size: 0.8em;
    cursor: pointer;
    transition: all 0.2s ease;
}

.toggle-json-button:hover,
.copy-button:hover {
    background: var(--background-secondary, rgba(0, 0, 0, 0.05));
}

.toggle-json-button.expanded {
    background: var(--primary-color, #3b82f6);
    color: white;
    border-color: var(--primary-color, #3b82f6);
}

.toggle-arrow {
    width: 14px;
    height: 14px;
    transition: transform 0.2s ease;
}

.toggle-json-button.expanded .toggle-arrow {
    transform: rotate(90deg);
}

.copy-icon {
    width: 14px;
    height: 14px;
}

.json-viewer {
    padding: 20px;
    background: var(--code-background, #1a1a1a);
    color: var(--code-text, #e5e5e5);
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
    font-size: 0.85em;
    line-height: 1.5;
    overflow-x: auto;
}

.pagination-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 0;
}

.pagination-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px;
    background: var(--card-background);
    color: var(--text-color);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    font-size: 0.9em;
    cursor: pointer;
    transition: all 0.2s ease;
}

.pagination-button:hover:not(:disabled) {
    background: var(--background-secondary, rgba(0, 0, 0, 0.05));
}

.pagination-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.pagination-icon {
    width: 16px;
    height: 16px;
}

.pagination-pages {
    display: flex;
    gap: 4px;
}

.page-button {
    padding: 8px 12px;
    background: var(--card-background);
    color: var(--text-color);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    font-size: 0.9em;
    cursor: pointer;
    transition: all 0.2s ease;
    min-width: 40px;
}

.page-button:hover:not(:disabled):not(.ellipsis) {
    background: var(--background-secondary, rgba(0, 0, 0, 0.05));
}

.page-button.active {
    background: var(--primary-color, #3b82f6);
    color: white;
    border-color: var(--primary-color, #3b82f6);
}

.page-button.ellipsis {
    cursor: default;
    background: transparent;
    border: none;
}

.page-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

@media (max-width: 768px) {
    .pagination-controls {
        flex-direction: column;
        gap: 12px;
        align-items: stretch;
    }

    .pagination-actions {
        justify-content: space-between;
    }

    .scrobble-header {
        flex-direction: column;
        align-items: stretch;
        gap: 12px;
    }

    .scrobble-actions {
        justify-content: stretch;
    }

    .toggle-json-button,
    .copy-button {
        flex: 1;
        justify-content: center;
    }

    .pagination-footer {
        flex-direction: column;
        gap: 16px;
    }

    .pagination-pages {
        order: -1;
        justify-content: center;
    }
}
</style>
