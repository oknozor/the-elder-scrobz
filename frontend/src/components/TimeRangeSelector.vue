<template>
    <div class="time-range-selector">
        <button
            v-for="range in timeRanges"
            :key="range"
            class="time-range-btn"
            :class="{ active: sharedTimeRange === range }"
            @click="setTimeRange(range)"
        >
            <TimeRangeIcon
                v-if="windowWidth < 650"
                :key="range"
                :timeRange="range"
            />
            <span v-else>
                {{ formatRange(range) }}
            </span>
        </button>
    </div>
</template>

<script setup lang="ts">
import { TimeRange } from "@/types/music";
import { useWindowWidth } from "@/composables/useWindowWidth";
import TimeRangeIcon from "./TimeRangeIcon.vue";
import { useTimeRangeStore } from "@/stores";
import { storeToRefs } from "pinia";

const windowWidth = useWindowWidth();

const timeRanges: TimeRange[] = ["today", "week", "month", "year", "all"];
const timeRangeStore = useTimeRangeStore();
const { sharedTimeRange } = storeToRefs(timeRangeStore);
const { setTimeRange } = timeRangeStore;

const formatRange = (range: TimeRange) => {
    if (range === "all") return "All time";
    if (range === "today") return "Today";
    return `This ${range}`;
};
</script>

<style scoped>
.time-range-selector {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 0;
}

.time-range-btn {
    padding: 6px 8px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    background: var(--card-background);
    color: var(--text-color);
    cursor: pointer;
    font-size: 0.9em;
    transition: all 0.2s;
    text-align: center;
    width: 100%;
}

.time-range-btn:hover {
    background: var(--background-color);
}

.time-range-btn.active {
    background: var(--primary-color);
    color: var(--background-color);
    border-color: var(--primary-color);
}

@media screen and (max-width: 650px) {
    .time-range-selector {
        flex-direction: row;
    }
}
</style>
