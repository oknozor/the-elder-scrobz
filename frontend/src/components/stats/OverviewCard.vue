<template>
  <div class="overview-card">
    <h3 class="card-title">{{ title }}</h3>
    <div class="card-value">{{ value }}</div>
    <div v-if="percentageChange !== null" class="percentage-change" :class="{ 'increase': percentageChange > 0, 'decrease': percentageChange < 0 }">
      {{ percentageChange > 0 ? '+' : '' }}{{ percentageChange }}% {{ comparisonText }}
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  title: {
    type: String,
    required: true,
  },
  value: {
    type: [String, Number],
    required: true,
  },
  percentageChange: {
    type: Number,
    default: null,
  },
  comparisonText: {
    type: String,
    default: '',
  },
});
</script>

<style scoped>
.overview-card {
  background: var(--card-background);
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  flex: 1;
}

.card-title {
  color: var(--text-secondary);
  font-size: 1em;
  margin: 0 0 8px 0;
  font-weight: 500;
}

.card-value {
  color: var(--text-color);
  font-size: 1.8em;
  font-weight: bold;
  margin-bottom: 8px;
}

.percentage-change {
  font-size: 0.9em;
  font-weight: 500;
}

.increase {
  color: #4caf50; /* Green for increase */
}

.decrease {
  color: #f44336; /* Red for decrease */
}
</style>