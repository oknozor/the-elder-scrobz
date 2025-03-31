<template>
	<div class="grid-container top-items">
		<StatCard
			v-if="items.length > 0"
			v-for="(item, index) in items.slice(0, step)"
			:key="generateKey(item, index)"
			:item="item"
			:rank="index + 1"
			:link="link"
			:step="step"
		/>
	</div>
	<div
		v-if="items.length > step && currentWidth > 500"
		class="grid-container bottom-items"
		:style="{ height: bottomHeight }"
		ref="bottomItems"
	>
		<StatCard
			v-for="(item, index) in items.slice(step, limit)"
			:key="generateKey(item, index)"
			:item="item"
			:rank="index + step + 1"
			:link="link"
			:step="step"
			@mouseenter="handleMouseEnter"
			@mouseleave="handleMouseLeave"
			class="small-card"
		/>
	</div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import StatCard from './StatCard.vue';
import type { Artist, Track, Album } from '@/types/music';
interface Props {
	items: Artist[] | Track[] | Album[];
	limit?: number;
	step?: number;
	link?: {
		name: string;
	};
}

withDefaults(defineProps<Props>(), {
	step: 5,
});

const currentWidth = ref(window.innerWidth);

const bottomItems = ref<HTMLDivElement | null>(null);
const bottomHeight = ref('7rem');

const handleMouseEnter = () => {
	if (window.innerWidth < 650) {
		return;
	} else {
		bottomHeight.value = '7rem';
	}
};

const handleMouseLeave = () => {
	if (window.innerWidth < 650) {
		return;
	} else {
		bottomHeight.value = '7rem';
	}
};

const generateKey = (item: Artist | Track | Album, index: number) => {
	if ('release_id' in item) {
		return item.release_id;
	} else if ('artist_id' in item) {
		return item.artist_id;
	} else if ('track_id' in item) {
		return item.track_id;
	}
	return index;
};
</script>

<style scoped>
.grid-container {
	display: flex;
	padding: 0;
	--gap: 12px;
	gap: var(--gap);
	align-items: flex-start;
	flex-wrap: nowrap;
	margin-bottom: var(--gap);
}

.grid-container.top-items {
	justify-content: flex-start;
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

.card {
	max-height: 14rem;
	max-width: calc(20% - var(--gap));
}

.small-card {
	height: auto;
	flex: 0 0 calc(10% - var(--gap));
	max-width: calc(10% - var(--gap));
	min-width: calc(10% - var(--gap));
}

/* Override hover behavior for small cards to maintain consistent sizing */
.small-card:hover {
	flex: 0 0 calc(20% - var(--gap));
	max-width: calc(20% - var(--gap));
	min-width: calc(20% - var(--gap));
}

@media screen and (max-width: 650px) {
	.small-card {
		height: auto;
		flex: 1;
		max-width: calc(20% - var(--gap));
		min-width: calc(20% - var(--gap));
	}
	.small-card:hover {
		flex: 1;
		max-width: calc(20% - var(--gap));
		min-width: calc(20% - var(--gap));
	}
}

@media screen and (max-width: 500px) {
	.grid-container.top-items {
		overflow-x: auto;
		scrollbar-width: none;
		-ms-overflow-style: none;
		--gap: 10px;
		gap: var(--gap);
	}
	.card {
		height: auto;
		flex: 0 0 calc(37.5% - var(--gap));
		max-width: calc(37.5% - var(--gap));
		min-width: calc(37.5% - var(--gap));
	}
	.small-card:hover {
		flex: 1;
		max-width: calc(20% - var(--gap));
		min-width: calc(20% - var(--gap));
	}
}
</style>
