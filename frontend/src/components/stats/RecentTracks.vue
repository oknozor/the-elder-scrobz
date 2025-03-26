<template>
	<table class="recent-tracks-table">
		<tbody>
			<tr
				v-for="(track, index) in paginatedTracks"
				:key="index"
				class="recent-track-row"
			>
				<td class="time-column">
					<transition name="fade" mode="out-in">
						<span :key="track.lastPlayed">{{
							formatTimeAgo(track.lastPlayed)
						}}</span>
					</transition>
				</td>
				<td class="user-column">
					<transition name="fade" mode="out-in">
						<span :key="track.user" class="user-name">
							{{ track.user }}
						</span>
					</transition>
				</td>
				<td class="track-column">
					<img
						:src="track.imageUrl"
						:alt="track.title"
						class="track-thumbnail"
						:class="{
							'image-loaded': imageLoaded(track.id),
						}"
						@load="onImageLoad(track.id)"
					/>
					<div class="track-info-container">
						<transition name="fade" mode="out-in">
							<div>
								<router-link
									:key="track.title"
									:to="{
										name: 'artist',
										params: { id: track.artist },
									}"
									class="link"
								>
									{{ track.artist }}
								</router-link>
								-
								<router-link
									:key="track.title"
									:to="{
										name: 'track',
										params: { id: track.title },
									}"
									class="link"
								>
									{{ track.title }}
								</router-link>
							</div>
						</transition>
					</div>
				</td>
			</tr>
		</tbody>
	</table>

	<div class="pagination">
		<button
			class="page-btn"
			:disabled="currentPage === 1"
			@click="$emit('change-page', currentPage - 1)"
		>
			Previous
		</button>
		<span class="page-info">
			Page {{ currentPage }} of {{ totalPages }}
		</span>
		<button
			class="page-btn"
			:disabled="currentPage === totalPages"
			@click="$emit('change-page', currentPage + 1)"
		>
			Next
		</button>
	</div>
</template>

<script setup lang="ts">
import { ref, PropType } from 'vue';
import { formatTimeAgo } from '@/utils/formatter';
import router from '@/router';
import { RecentTrack } from '@/types/music';

const loadedImages = ref<Set<string>>(new Set());

defineProps({
	paginatedTracks: {
		type: Array as PropType<RecentTrack[]>,
		required: true,
	},
	currentPage: {
		type: Number,
		required: true,
	},
	totalPages: {
		type: Number,
		required: true,
	},
});

defineEmits(['change-page']);

const goToTrackPage = (trackId: number) => {
	return () => {
		router.push({ name: 'track', params: { id: trackId } });
	};
};

const onImageLoad = (trackId: string) => {
	loadedImages.value.add(trackId);
};

const imageLoaded = (trackId: string) => {
	return loadedImages.value.has(trackId);
};
</script>

<style scoped>
.recent-tracks-table {
	height: 51rem;
	width: 100%;
	background: var(--card-background);
	border-radius: 8px;
	overflow: hidden;
	box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	border: 1px solid var(--border-color);
	margin-bottom: 16px;
}

.recent-track-row {
	display: grid;
	grid-template-columns: 120px 150px 1fr;
	padding: 8px 12px;
	border-bottom: 1px solid var(--border-color);
	align-items: center;
}

.recent-track-row:last-child {
	border-bottom: none;
}

.recent-track-row:hover {
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

.track {
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

.pagination {
	display: flex;
	justify-content: center;
	align-items: center;
	gap: 16px;
	margin-top: 16px;
}

.page-btn {
	padding: 6px 12px;
	border: 1px solid var(--border-color);
	border-radius: 4px;
	background: var(--card-background);
	color: var(--text-color);
	cursor: pointer;
	font-size: 0.9em;
	transition: all 0.2s;
}

.page-btn:hover:not(:disabled) {
	background: var(--background-color);
}

.page-btn:disabled {
	opacity: 0.5;
	cursor: not-allowed;
}

.page-info {
	color: var(--text-secondary);
	font-size: 0.9em;
}

/* Fade transition for the table content */
.fade-enter-active,
.fade-leave-active {
	transition: opacity 0.25s;
}
.fade-enter,
.fade-leave-to {
	opacity: 0;
}
</style>
