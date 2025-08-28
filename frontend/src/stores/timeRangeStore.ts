import { defineStore } from "pinia";
import type { TimeRange } from "@/types";

export const useTimeRangeStore = defineStore("timeRange", {
    state: () => ({
        sharedTimeRange: "week" as TimeRange,
    }),
    actions: {
        setTimeRange(timeRange: TimeRange) {
            this.sharedTimeRange = timeRange;
        },
    },
});
