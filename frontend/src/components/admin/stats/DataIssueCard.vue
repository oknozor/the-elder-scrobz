<template>
    <div
        v-if="issueData && issueData.count > 0"
        class="issue-card"
        @click="toggleExpansion"
    >
        <div class="issue-header">
            <div class="issue-info">
                <div class="issue-icon" :class="iconType">
                    <svg
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path v-if="iconType === 'warning'"
                            d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"
                        />
                        <line v-if="iconType === 'warning'" x1="12" y1="9" x2="12" y2="13" />
                        <line v-if="iconType === 'warning'" x1="12" y1="17" x2="12.01" y2="17" />

                        <circle v-if="iconType === 'info'" cx="12" cy="12" r="10" />
                        <line v-if="iconType === 'info'" x1="12" y1="16" x2="12" y2="12" />
                        <line v-if="iconType === 'info'" x1="12" y1="8" x2="12.01" y2="8" />
                    </svg>
                </div>
                <div>
                    <h4>{{ title }}</h4>
                    <p>{{ issueData.count }} {{ description }}</p>
                </div>
            </div>
            <div class="expand-icon" :class="{ expanded: isExpanded }">
                <svg
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <path d="M6 9l6 6 6-6" />
                </svg>
            </div>
        </div>
        <div v-if="isExpanded" class="issue-details" @click.stop>
            <div class="ids-container">
                <div class="ids-list">
                    <div
                        v-for="id in paginatedIds"
                        :key="id"
                        class="id-item"
                    >
                        <span class="id-text">{{ id }}</span>
                        <button
                            class="copy-btn"
                            @click="copyToClipboard(id)"
                            :title="`Copy ${id}`"
                        >
                            <svg
                                v-if="!copiedIds.has(id)"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
                            </svg>
                            <svg
                                v-else
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                class="check-icon"
                            >
                                <polyline points="20,6 9,17 4,12" />
                            </svg>
                        </button>
                    </div>
                </div>

                <div v-if="totalPages > 1" class="pagination-controls">
                    <div class="pagination-info">
                        Page {{ currentPage }} of {{ totalPages }} ({{ issueData.count }} total)
                    </div>
                    <div class="pagination-buttons">
                        <button
                            class="pagination-btn"
                            @click="goToPage(currentPage - 1)"
                            :disabled="currentPage <= 1"
                            title="Previous page"
                        >
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M15 18l-6-6 6-6" />
                            </svg>
                        </button>
                        <button
                            class="pagination-btn"
                            @click="goToPage(currentPage + 1)"
                            :disabled="currentPage >= totalPages"
                            title="Next page"
                        >
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M9 18l6-6-6-6" />
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="bulk-actions">
                    <button
                        class="bulk-copy-btn"
                        @click="copyAllVisible"
                        title="Copy all visible IDs"
                    >
                        Copy Page ({{ paginatedIds.length }})
                    </button>
                    <button
                        class="bulk-copy-btn"
                        @click="copyAllIds"
                        title="Copy all IDs from all pages"
                    >
                        Copy All ({{ issueData.count }})
                    </button>
                </div>
            </div>
        </div>

        <div v-if="showToast" class="toast" :class="{ show: showToast }">
            {{ toastMessage }}
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";

interface IssueData {
    count: number;
    ids: string[];
}

interface Props {
    title: string;
    description: string;
    issueData: IssueData | null;
    iconType?: "warning" | "info";
    itemsPerPage?: number;
}

const props = withDefaults(defineProps<Props>(), {
    iconType: "warning",
    itemsPerPage: 10,
});

const isExpanded = ref(false);
const currentPage = ref(1);
const copiedIds = ref(new Set<string>());
const showToast = ref(false);
const toastMessage = ref("");

const totalPages = computed(() => {
    if (!props.issueData) return 0;
    return Math.ceil(props.issueData.ids.length / props.itemsPerPage);
});

const paginatedIds = computed(() => {
    if (!props.issueData) return [];
    const start = (currentPage.value - 1) * props.itemsPerPage;
    const end = start + props.itemsPerPage;
    return props.issueData.ids.slice(start, end);
});

const toggleExpansion = () => {
    isExpanded.value = !isExpanded.value;
    // Reset to first page when expanding
    if (isExpanded.value) {
        currentPage.value = 1;
    }
};

const goToPage = (page: number) => {
    if (page >= 1 && page <= totalPages.value) {
        currentPage.value = page;
    }
};

const copyToClipboard = async (text: string) => {
    try {
        await navigator.clipboard.writeText(text);
        copiedIds.value.add(text);
        showToast.value = false; // Hide any existing toast
        toastMessage.value = `Copied: ${text}`;
        showToast.value = true;

        // Remove the copied state after 2 seconds
        setTimeout(() => {
            copiedIds.value.delete(text);
        }, 2000);

        // Hide toast after 3 seconds
        setTimeout(() => {
            showToast.value = false;
        }, 3000);
    } catch (err) {
        console.error("Failed to copy text: ", err);
        toastMessage.value = "Failed to copy to clipboard";
        showToast.value = true;
        setTimeout(() => {
            showToast.value = false;
        }, 3000);
    }
};

const copyAllVisible = async () => {
    const text = paginatedIds.value.join("\n");
    try {
        await navigator.clipboard.writeText(text);
        toastMessage.value = `Copied ${paginatedIds.value.length} IDs from page ${currentPage.value}`;
        showToast.value = true;
        setTimeout(() => {
            showToast.value = false;
        }, 3000);
    } catch (err) {
        console.error("Failed to copy text: ", err);
        toastMessage.value = "Failed to copy to clipboard";
        showToast.value = true;
        setTimeout(() => {
            showToast.value = false;
        }, 3000);
    }
};

const copyAllIds = async () => {
    if (!props.issueData) return;
    const text = props.issueData.ids.join("\n");
    try {
        await navigator.clipboard.writeText(text);
        toastMessage.value = `Copied all ${props.issueData.count} IDs`;
        showToast.value = true;
        setTimeout(() => {
            showToast.value = false;
        }, 3000);
    } catch (err) {
        console.error("Failed to copy text: ", err);
        toastMessage.value = "Failed to copy to clipboard";
        showToast.value = true;
        setTimeout(() => {
            showToast.value = false;
        }, 3000);
    }
};
</script>

<style scoped>
.issue-card {
    background: var(--card-background);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1rem;
    cursor: pointer;
    transition: all 0.2s ease;
    position: relative;
}

.issue-card:hover {
    border-color: var(--primary-color);
}

.issue-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.issue-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
}

.issue-icon {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
}

.issue-icon.warning {
    color: #f59e0b;
}

.issue-icon.info {
    color: #3b82f6;
}

.issue-icon svg {
    width: 16px;
    height: 16px;
}

.issue-info h4 {
    margin: 0 0 0.25rem 0;
    font-size: 0.875rem;
    font-weight: 600;
}

.issue-info p {
    margin: 0;
    color: var(--text-muted);
    font-size: 0.75rem;
}

.expand-icon {
    width: 20px;
    height: 20px;
    color: var(--text-muted);
    transition: transform 0.2s ease;
}

.expand-icon.expanded {
    transform: rotate(180deg);
}

.expand-icon svg {
    width: 100%;
    height: 100%;
}

.issue-details {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
    cursor: default;
}

.ids-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.ids-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.id-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--background-secondary);
    padding: 0.5rem;
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
}

.id-text {
    word-break: break-all;
    flex: 1;
    margin-right: 0.5rem;
}

.copy-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 3px;
    color: var(--text-muted);
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 20px;
    height: 20px;
}

.copy-btn:hover {
    background: var(--border-color);
    color: var(--text-secondary);
}

.copy-btn svg {
    width: 14px;
    height: 14px;
}

.copy-btn .check-icon {
    color: #10b981;
}

.pagination-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: var(--background-secondary);
    border-radius: 6px;
    border: 1px solid var(--border-color);
}

.pagination-info {
    font-size: 0.75rem;
    color: var(--text-muted);
}

.pagination-buttons {
    display: flex;
    gap: 0.5rem;
}

.pagination-btn {
    background: none;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 0.375rem;
    cursor: pointer;
    color: var(--text-secondary);
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
}

.pagination-btn:hover:not(:disabled) {
    background: var(--border-color);
    border-color: var(--primary-color);
}

.pagination-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.pagination-btn svg {
    width: 16px;
    height: 16px;
}

.bulk-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: center;
}

.bulk-copy-btn {
    background: var(--primary-color);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.bulk-copy-btn:hover {
    background: var(--primary-color-dark, #1d4ed8);
    transform: translateY(-1px);
}

.toast {
    position: absolute;
    top: -3rem;
    right: 1rem;
    background: #374151;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.75rem;
    font-weight: 500;
    opacity: 0;
    transform: translateY(10px);
    transition: all 0.3s ease;
    pointer-events: none;
    z-index: 1000;
    white-space: nowrap;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.toast.show {
    opacity: 1;
    transform: translateY(0);
}

.toast::after {
    content: '';
    position: absolute;
    bottom: -5px;
    right: 1rem;
    width: 0;
    height: 0;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
    border-top: 5px solid #374151;
}
</style>
