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
        <div v-if="isExpanded" class="issue-details">
            <div class="ids-list">
                <div
                    v-for="id in issueData.ids.slice(0, displayLimit)"
                    :key="id"
                    class="id-item"
                >
                    {{ id }}
                </div>
                <div v-if="issueData.ids.length > displayLimit" class="more-items">
                    +{{ issueData.ids.length - displayLimit }} more...
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

interface IssueData {
    count: number;
    ids: string[];
}

interface Props {
    title: string;
    description: string;
    issueData: IssueData | null;
    iconType?: "warning" | "info";
    displayLimit?: number;
}

withDefaults(defineProps<Props>(), {
    iconType: "warning",
    displayLimit: 10,
});

const isExpanded = ref(false);

const toggleExpansion = () => {
    isExpanded.value = !isExpanded.value;
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
}

.ids-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
}

.id-item {
    background: var(--background-secondary);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-secondary);
}

.more-items {
    background: var(--primary-color);
    color: white;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 500;
}
</style>
