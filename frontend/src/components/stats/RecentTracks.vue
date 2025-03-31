<template>
	<div class="tracks">
		<div
			class="tracks-container"
			ref="tableContainer"
			@scroll="handleScroll"
		>
			<table class="tracks-table" ref="table">
				<tbody>
					<transition-group
						name="track-fade"
						tag="tr"
						appear
						@before-enter="beforeEnter"
					>
						<tr
							v-for="(track, index) in tableElements"
							:key="track.id || index"
							class="track-row"
							:data-index="index"
							ref="trackRows"
						>
							<td class="time-column">
								<span>{{
									formatTimeAgo(track.listened_at)
								}}</span>
							</td>
							<td class="user-column">
								<span class="user-name">{{ track.user }}</span>
							</td>
							<td class="track-column">
								<img
									:src="track.cover_art_url"
									:alt="track.track_name"
									class="track-thumbnail"
									:class="{
										'image-loaded': imageLoaded(track.id),
									}"
									@load="onImageLoad(track.id)"
									@error="onImageError"
								/>
								<div class="track-info-container">
									<router-link
										:to="{
											name: 'artist',
											params: { id: track.artist_name },
										}"
										class="link"
									>
										{{ track.artist_name }}
									</router-link>
									-
									<router-link
										:to="{
											name: 'track',
											params: { id: track.track_name },
										}"
										class="link"
									>
										<span class="song-name">
											{{ track.track_name }}
										</span>
									</router-link>
								</div>
							</td>
						</tr>
					</transition-group>
				</tbody>
			</table>
			<div v-if="isLoading" class="loading-indicator">
				Loading more tracks...
			</div>
		</div>
		<button
			class="back-to-top"
			@click="scrollToTop"
			v-if="isScrollTopButtonVisible"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="24"
				height="24"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M12 5v6m0 3v1.5m0 3v.5" />
				<path d="M16 9l-4 -4" />
				<path d="M8 9l4 -4" />
			</svg>
		</button>
	</div>
</template>

<script setup lang="ts">
import { ref, PropType, onMounted, onUnmounted, watch, nextTick } from 'vue';
import { formatTimeAgo } from '@/utils/formatter';
import { RecentTrack } from '@/types/music';

const loadedImages = ref<Set<string>>(new Set());
const tableContainer = ref<HTMLElement | null>(null);
const table = ref<HTMLElement | null>(null);
const isLoading = ref(false);
const currentPage = ref(1);
const trackRows = ref<HTMLElement[]>([]);
const scrollTimeout = ref<number | null>(null);
const tableElements = ref<RecentTrack[]>([]);
const isScrollTopButtonVisible = ref(false);
let observer: IntersectionObserver | null = null;

const props = defineProps({
	tracks: {
		type: Array as PropType<RecentTrack[]>,
		required: true,
	},
	totalPages: {
		type: Number,
		required: true,
	},
});

const onImageLoad = (trackId: string) => {
	console.log('image loaded', trackId);
	loadedImages.value.add(trackId);
};

const imageLoaded = (trackId: string) => {
	return loadedImages.value.has(trackId);
};

const onImageError = (event: Event) => {
	const target = event.target as HTMLImageElement;
	target.src = '/img/photo-off.svg';
};

const scrollToTop = () => {
	if (tableContainer.value) {
		tableContainer.value.scrollTo({ top: 0, behavior: 'smooth' });
	}
};

const handleScroll = () => {
	if (!tableContainer.value) return;
	const { scrollTop } = tableContainer.value;
	isScrollTopButtonVisible.value = scrollTop > 200;
};

const loadMoreTracks = async () => {
	if (isLoading.value || currentPage.value >= props.totalPages) return;

	try {
		isLoading.value = true;
		currentPage.value++;
		await emit('load-more', currentPage.value);
	} finally {
		isLoading.value = false;
	}
};

const beforeEnter = (el: Element) => {
	const index = Number((el as HTMLElement).dataset.index);
	(el as HTMLElement).style.transitionDelay = `${Math.min(
		index * 10,
		300
	)}ms`;
};

// IntersectionObserver to detect when the last row enters the viewport
const observeLastRow = () => {
	if (observer) observer.disconnect();

	observer = new IntersectionObserver(
		(entries) => {
			const lastEntry = entries[0];
			if (lastEntry.isIntersecting && !isLoading.value) {
				loadMoreTracks();
			}
		},
		{
			rootMargin: '100px',
			threshold: 0.1,
			root: tableContainer.value,
		}
	);

	// Attach observer to the last row only
	if (trackRows.value.length > 0) {
		const lastRow = trackRows.value[trackRows.value.length - 1];
		if (lastRow) {
			observer.observe(lastRow);
		}
	}
};

// Watch for new tracks and append them smoothly
watch(
	() => props.tracks,
	async (newTracks) => {
		await nextTick(); // Ensures DOM updates before mutation
		if (currentPage.value === 1) {
			tableElements.value = newTracks;
		} else {
			requestAnimationFrame(() => {
				tableElements.value = [...tableElements.value, ...newTracks];
			});
		}
		// Add a small delay before observing the new last row
		setTimeout(() => {
			observeLastRow();
		}, 100);
	}
);

// Also observe the last row when the component is mounted
onMounted(() => {
	tableElements.value = props.tracks;
	// Add a small delay before initial observation
	setTimeout(() => {
		observeLastRow();
	}, 100);
	if (table.value) {
		table.value.scrollIntoView({ behavior: 'smooth', block: 'start' });
	}
	if (tableContainer.value) {
		tableContainer.value.addEventListener('scroll', handleScroll);
	}
});

onUnmounted(() => {
	if (observer) observer.disconnect();
	if (scrollTimeout.value) {
		window.clearTimeout(scrollTimeout.value);
	}
	if (tableContainer.value) {
		tableContainer.value.removeEventListener('scroll', handleScroll);
	}
});

const emit = defineEmits(['load-more']);
</script>

<style scoped>
.tracks {
	width: 100%;
	height: 100%;
}

.tracks-container {
	height: 40rem;
	width: 100%;
	background: var(--card-background);
	border-radius: 8px;
	overflow-y: auto;
	box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	border: 1px solid var(--border-color);
	margin-bottom: 16px;
	position: relative;
}

.tracks-table {
	width: 100%;
	table-layout: fixed;
}

.track-row {
	display: grid;
	grid-template-columns: 120px 150px 1fr;
	padding: 8px 12px;
	border-bottom: 1px solid var(--border-color);
	align-items: center;
	height: 48px;
	will-change: background-color;
}

.track-row:last-child {
	border-bottom: none;
}

.track-row:hover {
	background: rgba(255, 255, 255, 0.05);
}

.time-column {
	color: var(--text-secondary);
	font-size: 0.9em;
}

.user-column {
	color: var(--text-color);
	font-size: 0.9em;
}

.link {
	color: var(--text-color);
	text-decoration: none;
}

.song-name {
	font-style: italic;
}

.link:hover {
	color: var(--text-secondary);
	text-decoration: underline;
}

.user-name {
	color: var(--text-color);
}

.track-column {
	color: var(--text-color);
	font-size: 0.9em;
	white-space: nowrap;
	overflow: hidden;
	text-overflow: ellipsis;
	display: flex;
	align-items: center;
	gap: 8px;
}

.track-thumbnail {
	width: 24px;
	height: 24px;
	border-radius: 4px;
	object-fit: cover;
	flex-shrink: 0;
	transition: opacity 0.25s ease-in-out;
	opacity: 0;
}

.track-thumbnail.image-loaded {
	opacity: 1;
}

.track-info-container {
	display: flex;
	align-items: center;
	gap: 8px;
	flex: 1;
}

.tracks-container::-webkit-scrollbar {
	width: 8px;
}

.tracks-container::-webkit-scrollbar-track {
	background: var(--background-color);
}

.tracks-container::-webkit-scrollbar-thumb {
	background: var(--border-color);
	border-radius: 4px;
}

.tracks-container::-webkit-scrollbar-thumb:hover {
	background: var(--text-secondary);
}

.loading-indicator {
	text-align: center;
	padding: 1rem;
	color: var(--text-secondary);
	font-size: 0.9em;
	position: sticky;
	bottom: 0;
	background: var(--card-background);
	border-top: 1px solid var(--border-color);
}

.back-to-top {
	position: absolute;
	color: var(--text-color);
	bottom: 20px;
	right: 20px;
	background-color: var(--primary-color);
	border: none;
	border-radius: 50%;
	width: 40px;
	height: 40px;
	display: flex;
	justify-content: center;
	align-items: center;
	cursor: pointer;
	box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	opacity: 0.7;
	transition: opacity 0.3s, transform 0.3s;
}

.back-to-top:hover {
	opacity: 1;
	transform: scale(1.1);
}

.track-fade-enter-active {
	transition: all 0.2s ease-out;
}

.track-fade-leave-active {
	transition: all 0.1s ease-in;
}

.track-fade-enter-from {
	opacity: 0;
	transform: translateY(20px);
}

.track-fade-leave-to {
	opacity: 0;
	transform: translateY(-20px);
}

.track-fade-move {
	transition: transform 0.2s ease;
}

@media screen and (max-width: 500px) {
	.track-row {
		grid-template-columns: 70px 60px 1fr;
		overflow-x: scroll;
	}
	.time-column,
	.user-column {
		text-align: start;
		text-wrap: nowrap;
	}
	.track-info-container {
		overflow-x: scroll;
	}
}
</style>
