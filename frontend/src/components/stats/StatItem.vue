<template>
	<div class="stat-item">
		<span class="stat-label">{{ label }}</span>
		<span class="stat-value">{{ value }}</span>
	</div>
</template>

<script setup lang="ts">
defineProps({
	label: {
		type: String,
		required: true,
	},
	value: {
		type: [String, Number],
		required: true,
	},
});
</script>

<style scoped>
.stat-item {
	display: flex;
	flex-direction: column;
	align-items: center;
	flex: 1;
}

.stat-label {
	color: var(--text-secondary);
	font-size: 0.9em;
	margin-bottom: 4px;
}

.stat-value {
	color: var(--text-color);
	font-size: 1.2em;
	font-weight: bold;
}
</style>
