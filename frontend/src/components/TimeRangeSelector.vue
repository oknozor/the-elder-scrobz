<template>
  <div class="time-range-selector">
    <button
      v-for="range in timeRanges"
      :key="range"
      class="time-range-btn"
      :class="{ active: modelValue === range }"
      @click="$emit('update:modelValue', range)"
    >
      {{ range === 'all' ? 'All time' : range === 'today' ? 'Today' : `This ${range}` }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { TimeRange } from '@/types/music';

const timeRanges: TimeRange[] = ['today', 'week', 'month', 'year', 'all'];

defineProps<{
  modelValue: TimeRange;
}>();

defineEmits<{
  (e: 'update:modelValue', value: TimeRange): void;
}>();
</script>

<style scoped>
.time-range-selector {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}

.time-range-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--card-background);
  color: var(--text-color);
  cursor: pointer;
  font-size: 0.9em;
  transition: all 0.2s;
}

.time-range-btn:hover {
  background: var(--background-color);
}

.time-range-btn.active {
  background: var(--primary-color);
  color: var(--background-color);
  border-color: var(--primary-color);
}
</style> 