import { defineStore } from "pinia";

export type TimeRange = "today" | "week" | "month" | "year" | "all";

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
