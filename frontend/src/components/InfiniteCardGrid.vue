<template>
	<div class="card-grid-container">
		<div class="card-grid-wrapper" ref="wrapperRef" @scroll="handleScroll">
			<transition-group
				name="fade"
				tag="div"
				class="card-grid"
				@before-enter="beforeEnter"
			>
				<Card
					v-for="(item, index) in items"
					:key="getItemKey(item, index)"
					:item="item"
					:rank="index + 1"
					:link="link"
					class="card"
				/>
				<div class="load-more-wrapper" ref="loadMoreRef">
					<div v-if="loading" class="loading-indicator">
						Loading<span>.</span><span>.</span><span>.</span>
					</div>
				</div>
			</transition-group>
		</div>
		<div
			class="blur-overlay blur-top"
			:class="{ active: showTopBlur }"
		></div>
		<div
			class="blur-overlay blur-bottom"
			:class="{ active: showBottomBlur }"
		></div>
	</div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import Card from './Card.vue';
import type { Artist, Track, Album } from '@/types/music';
import type { PaginatedResponse } from '@/types/music';

interface Props {
	items: PaginatedResponse<Artist | Track | Album>;
	loading?: boolean;
	link?: {
		name: string;
	};
}

const props = withDefaults(defineProps<Props>(), {
	loading: false,
});

const emit = defineEmits<{
	(e: 'load-more', page: number): void;
}>();

const wrapperRef = ref<HTMLElement | null>(null);
const loadMoreRef = ref<HTMLElement | null>(null);
const currentPage = ref(1);
const showTopBlur = ref(false);
const showBottomBlur = ref(true);
const loading = ref(false);
const items = ref<Array<Artist | Track | Album>>([]);
const isEndReached = ref(false);

const getItemKey = (item: Artist | Track | Album, index: number) => {
	if ('release_id' in item) return `release-${item.release_id}`;
	if ('artist_id' in item) return `artist-${item.artist_id}`;
	if ('track_id' in item) return `track-${item.track_id}`;
	return `item-${index}`;
};

watch(
	() => props.items,
	(newValue) => {
		if (newValue && newValue.data) {
			// If we receive an empty array, we've reached the end (Might change if API is updated, we could use the total property to check 👌)
			if (newValue.data.length === 0) {
				isEndReached.value = true;
			} else {
				// Append new items to our local state
				if (currentPage.value === 1) {
					// First page, replace items
					items.value = [...newValue.data];
				} else {
					// Subsequent pages, append items
					items.value = [...items.value, ...newValue.data];
				}
				currentPage.value++;
			}
			loading.value = false;
		}
	},
	{ deep: true }
);

watch(
	() => props.loading,
	(newValue) => {
		loading.value = newValue;
	}
);

// Intersection Observer for infinite loading
let observer: IntersectionObserver | null = null;

const setupIntersectionObserver = () => {
	if (observer) observer.disconnect();

	observer = new IntersectionObserver(
		(entries) => {
			const target = entries[0];
			if (
				target.isIntersecting &&
				!loading.value &&
				!isEndReached.value
			) {
				loadMoreContent();
			}
		},
		{
			root: null,
			rootMargin: '200px',
			threshold: 0.1,
		}
	);

	if (loadMoreRef.value) {
		observer.observe(loadMoreRef.value);
	}
};

const loadMoreContent = () => {
	loading.value = true;
	emit('load-more', currentPage.value);
};

// Handle scroll for blur effects
const handleScroll = () => {
	if (!wrapperRef.value) return;
	const { scrollTop, scrollHeight } = wrapperRef.value;
	showTopBlur.value = scrollTop > 10;
	showBottomBlur.value = scrollTop < scrollHeight - 10;
};

const beforeEnter = (el: Element) => {
	const index = Array.from(el.parentElement?.children || []).indexOf(el);
	(el as HTMLElement).style.transitionDelay = `${index * 50}ms`;
};

onMounted(() => {
	if (props.items && props.items.data) {
		items.value = [...props.items.data];
	}
	setupIntersectionObserver();
	handleScroll();

	window.addEventListener('resize', () => {
		setupIntersectionObserver();
		handleScroll();
	});
});

onUnmounted(() => {
	if (observer) observer.disconnect();
	window.removeEventListener('resize', () => {});
});
</script>

<style scoped>
.card-grid-container {
	width: 100%;
	height: 75%;
	position: relative;
}

.card-grid-wrapper {
	width: 100%;
	height: 100%;
	overflow-y: auto;
	overflow-x: hidden;
	scrollbar-width: none;
}

.card-grid-wrapper::-webkit-scrollbar {
	display: none;
}

.card-grid {
	--gap: 1rem;
	width: 100%;
	display: flex;
	flex-wrap: wrap;
	justify-content: flex-start;
	align-items: flex-start;
	gap: var(--gap);
	box-sizing: border-box;
	position: relative;
	z-index: 1;
}

.card {
	width: calc(25% - var(--gap) * 3 / 4);
}

.load-more-wrapper {
	width: 100%;
	padding: 1rem;
	display: flex;
	justify-content: center;
	align-items: center;
	min-height: 60px;
}

.loading-indicator {
	display: flex;
	justify-content: center;
	align-items: center;
	gap: 0.5rem;
	font-weight: 500;
	color: var(--text-color-secondary, #666);
	animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
	0% {
		opacity: 0.3;
	}
	50% {
		opacity: 1;
	}
	100% {
		opacity: 0.3;
	}
}

.blur-overlay {
	position: absolute;
	z-index: 2;
	left: 0;
	right: 0;
	height: 50px;
	pointer-events: none;
	opacity: 0;
	transition: opacity 0.3s ease;
	background-color: var(--background-color, #ffffff);
}

.blur-overlay.active {
	opacity: 1;
}

.blur-top {
	top: 0;
	background: linear-gradient(
		to bottom,
		var(--background-color, #ffffff) 20%,
		transparent
	);
}

.blur-bottom {
	bottom: 0;
	background: linear-gradient(
		to top,
		var(--background-color, #ffffff) 20%,
		transparent
	);
}

@media (max-width: 768px) {
	.card {
		width: calc(50% - var(--gap) / 2);
	}
}

.fade-enter-active,
.fade-leave-active {
	transition: all 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
	transform: translateY(20px);
}

.fade-move {
	transition: transform 0.5s ease;
}
</style>
