<template>
    <div class="stat-card">
        <div
            v-html="icon"
            class="stat-icon"
            :style="{ color: color, background: background }"
        ></div>
        <div class="stat-content">
            <h3>{{ props.title }}</h3>
            <p class="stat-number">
                {{ formatNumber(props.value) }}
            </p>
        </div>
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{
    title: string;
    icon: string;
    value: number;
    color?: string;
    background?: string;
}>();

const formatNumber = (num: number | undefined | null): string => {
    if (num === undefined || num === null || Number.isNaN(num)) return "0";
    return num.toLocaleString();
};
</script>

<style>
.stat-icon svg {
    width: 24px;
    height: 24px;
}
</style>

<style scoped>
.stat-card {
    background: var(--card-background);
    border-radius: 8px;
    padding: 24px;
    border: 1px solid var(--border-color);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    display: flex;
    align-items: center;
    gap: 16px;
    transition: all 0.2s;
}

.stat-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
}

.stat-icon {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
}

.stats-content {
    display: flex;
    flex-direction: column;
    gap: 32px;
}

.stat-content h3 {
    margin: 0 0 4px 0;
    color: var(--text-secondary);
    font-size: 0.9em;
    font-weight: 500;
}

.stat-number {
    margin: 0;
    color: var(--text-color);
    font-size: 1.8em;
    font-weight: bold;
}
</style>
