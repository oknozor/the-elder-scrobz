<template>
	<div class="grid-container top-items">
		<StatCard
			v-for="(item, index) in items.slice(0, step)"
			:key="item.id"
			:item="item"
			:rank="index + 1"
			:link="link"
			:step="step"
		/>
	</div>
	<div
		class="grid-container bottom-items"
		:style="{ height: bottomHeight }"
		@mouseenter="handleMouseEnter"
		@mouseleave="handleMouseLeave"
		ref="bottomItems"
	>
		<StatCard
			v-for="(item, index) in items.slice(step, limit)"
			:key="item.id"
			:item="item"
			:rank="index + step + 1"
			:link="link"
			:step="step"
		/>
	</div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import StatCard from './StatCard.vue';

interface Props {
	items: Array<{
		id: string;
		name?: string;
		title?: string;
		artist?: string;
		imageUrl: string;
		playCount: number;
		duration: number;
	}>;
	limit?: number;
	step?: number;
	link?: {
		name: string;
	};
}

withDefaults(defineProps<Props>(), {
	step: 5,
});

const bottomItems = ref<HTMLDivElement | null>(null);
const bottomHeight = ref('7.5rem');

const handleMouseEnter = () => {
	if (bottomItems.value) {
		bottomHeight.value = '14rem';
	}
};

const handleMouseLeave = () => {
	if (bottomItems.value) {
		bottomHeight.value = '7.5rem';
	}
};
</script>

<style scoped>
.grid-container {
	display: flex;
	padding: 5px 0;
	--gap: 12px;
	gap: var(--gap);
	align-items: flex-start;
	flex-wrap: nowrap;
}

.grid-container.top-items {
	margin-bottom: 5px;
	justify-content: center;
}

.grid-container.top-items .card {
	flex: 1;
}

.grid-container.bottom-items {
	justify-content: flex-start;
	gap: var(--gap);
	width: 100%;
	overflow-x: auto;
	scrollbar-width: none;
	-ms-overflow-style: none;
}

.bottom-items {
	transition: all 0.3s ease;
}

.grid-container .card {
	transition: all 0.3s ease;
}

.top-items .card:hover {
	transform: translateY(-4px);
}

.card:hover {
	flex: 0 0 calc(20% - (var(--gap) * 9 / 10));
	z-index: 1;
}
</style>
