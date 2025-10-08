<template>
    <div class="time-range-selector">
        <Button
            v-for="range in timeRanges"
            :key="range"
            :active="sharedTimeRange === range"
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
        </Button>
    </div>
</template>

<script setup lang="ts">
import { storeToRefs } from "pinia";
import Button from "@/base/Button.vue";
import { useWindowWidth } from "@/composables/useWindowWidth";
import { useTimeRangeStore } from "@/stores";
import type { TimeRange } from "@/types";
import TimeRangeIcon from "./TimeRangeIcon.vue";

const timeRanges: TimeRange[] = ["today", "week", "month", "year", "all"];

const windowWidth = useWindowWidth();
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

@media screen and (max-width: 650px) {
    .time-range-selector {
        flex-direction: row;
    }
}
</style>
