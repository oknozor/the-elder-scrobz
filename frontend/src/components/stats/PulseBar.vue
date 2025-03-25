<template>
	<div class="pulse-bar-container">
		<div class="pulse-bar" :style="{ width: barWidth + '%' }"></div>
	</div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps({
	playCount: {
		type: Number,
		required: true,
	},
	maxPlayCount: {
		type: Number,
		required: true,
	},
});

const barWidth = ref(0);

const updateBarWidth = () => {
	barWidth.value = (props.playCount / props.maxPlayCount) * 100 || 0;
};

watch(() => props.playCount, updateBarWidth);
watch(() => props.maxPlayCount, updateBarWidth);

// Initial update
updateBarWidth();
</script>

<style scoped>
.pulse-bar-container {
	height: 6px;
	background: rgba(255, 255, 255, 0.1);
	border-radius: 3px;
	overflow: hidden;
}

.pulse-bar {
	height: 100%;
	background: var(--primary-color);
	border-radius: 3px;
	width: 0;
	transition: width 0.3s ease;
}
</style>
