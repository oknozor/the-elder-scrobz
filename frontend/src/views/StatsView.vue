<template>
	<div class="stats">
		<div class="stats-section">
			<SectionHeader>
				<template #icon>
					<svg
						class="title-icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
						<circle cx="12" cy="7" r="4" />
					</svg>
				</template>
				Artists Chart
				<template #controls>
					<TimeRangeSelector
						v-model="timeRanges.tracks"
						@update:modelValue="fetchStats"
					/>
				</template>
			</SectionHeader>
			<StatGrid
				:items="stats.topArtists"
				:limit="15"
				:link="{ name: 'artist' }"
			/>
		</div>

		<div class="stats-section">
			<SectionHeader>
				<template #icon>
					<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
					<circle cx="12" cy="7" r="4" />
				</template>
				Tracks Chart
				<template #controls>
					<TimeRangeSelector
						v-model="timeRanges.tracks"
						@update:modelValue="fetchStats"
					/>
				</template>
			</SectionHeader>
			<StatGrid
				:items="stats.topTracks"
				:limit="15"
				:link="{ name: 'track' }"
			/>
		</div>

		<div class="stats-section">
			<SectionHeader>
				<template #icon>
					<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
					<circle cx="12" cy="7" r="4" />
				</template>
				Album Chart
				<template #controls>
					<TimeRangeSelector
						v-model="timeRanges.albums"
						@update:modelValue="fetchStats"
					/>
				</template>
			</SectionHeader>
			<StatGrid
				:items="stats.topAlbums"
				:limit="15"
				:link="{ name: 'album' }"
			/>
		</div>

		<div class="stats-section">
			<SectionHeader>
				<template #icon>
					<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
					<circle cx="12" cy="7" r="4" />
				</template>
				Pulse
				<template #controls>
					<div class="pulse-filters">
						<button
							v-for="range in pulseTimeRanges"
							:key="range"
							class="pulse-filter-btn"
							:class="{ active: selectedPulseRange === range }"
							@click="selectedPulseRange = range"
						>
							{{ range.replace(/(\d+)(\w+)/, '$1 $2') }}
						</button>
					</div>
				</template>
			</SectionHeader>
			<div class="pulse-table">
				<div
					v-for="(data, index) in stats.pulseData"
					:key="index"
					class="pulse-row"
				>
					<transition name="fade" mode="out-in">
						<div :key="data.period" class="pulse-period">
							{{ data.period }}
						</div>
					</transition>
					<transition name="fade" mode="out-in">
						<div :key="data.playCount" class="pulse-count">
							{{ data.playCount }} plays
						</div>
					</transition>
					<PulseBar
						:key="index"
						:playCount="data.playCount"
						:maxPlayCount="maxPlayCount"
					/>
				</div>
			</div>
		</div>

		<div class="stats-section">
			<SectionHeader>
				<template #icon>
					<svg
						class="title-icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<polygon points="5 3 19 12 5 21 5 3" />
					</svg>
				</template>
				Recent Tracks
				<template #controls>
					<div class="view-toggle">
						<button
							class="toggle-btn"
							:class="{ active: showDuration }"
							@click="showDuration = !showDuration"
							:title="
								showDuration ? 'Show Plays' : 'Show Duration'
							"
						>
							<svg
								v-if="showDuration"
								class="icon"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<circle cx="12" cy="12" r="10" />
								<path d="M12 6v6l4 2" />
							</svg>
							<svg
								v-else
								class="icon"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path d="M8 5v14l11-7z" />
							</svg>
						</button>
					</div>
				</template>
			</SectionHeader>
			<div class="stats-summary">
				<div class="stat-item">
					<span class="stat-label">Today</span>
					<span class="stat-value">
						{{
							showDuration
								? formatDuration(stats.timeStats.today.duration)
								: stats.timeStats.today.playCount
						}}
					</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">This Week</span>
					<span class="stat-value">
						{{
							showDuration
								? formatDuration(stats.timeStats.week.duration)
								: stats.timeStats.week.playCount
						}}
					</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">This Month</span>
					<span class="stat-value">
						{{
							showDuration
								? formatDuration(stats.timeStats.month.duration)
								: stats.timeStats.month.playCount
						}}
					</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">This Year</span>
					<span class="stat-value">
						{{
							showDuration
								? formatDuration(stats.timeStats.year.duration)
								: stats.timeStats.year.playCount
						}}
					</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">All Time</span>
					<span class="stat-value">
						{{
							showDuration
								? formatDuration(stats.timeStats.all.duration)
								: stats.timeStats.all.playCount
						}}
					</span>
				</div>
			</div>
			<div class="recent-tracks-table">
				<div
					v-for="track in paginatedTracks"
					:key="track.id"
					class="recent-track-row"
				>
					<div class="time-column">
						{{ formatTimeAgo(track.lastPlayed) }}
					</div>
					<div class="user-column">{{ track.user }}</div>
					<div class="track-column">
						<img
							:src="track.imageUrl"
							:alt="track.title"
							class="track-thumbnail"
						/>
						<div class="track-info-container">
							{{ track.artist }} - {{ track.title }}
						</div>
					</div>
				</div>
			</div>
			<div class="pagination">
				<button
					class="page-btn"
					:disabled="currentPage === 1"
					@click="currentPage--"
				>
					Previous
				</button>
				<span class="page-info"
					>Page {{ currentPage }} of {{ totalPages }}</span
				>
				<button
					class="page-btn"
					:disabled="currentPage === totalPages"
					@click="currentPage++"
				>
					Next
				</button>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { TimeRange, MusicStats, User, PulseTimeRange } from '@/types/music';
import { fetchMusicStats } from '@/services/mockData';
import { fetchUsers } from '@/services/mockUsers';
import TimeRangeSelector from '@/components/TimeRangeSelector.vue';
import SectionHeader from '@/components/stats/SectionHeader.vue';
import StatGrid from '@/components/stats/StatGrid.vue';
import PulseBar from '@/components/stats/PulseBar.vue';
import { formatTimeAgo, formatDuration } from '@/utils/formatter';

interface TimeRanges {
	artists: TimeRange;
	tracks: TimeRange;
	albums: TimeRange;
	recent: TimeRange;
}

const timeRanges = ref<TimeRanges>({
	artists: 'week',
	tracks: 'week',
	albums: 'week',
	recent: 'week',
});

const stats = ref<MusicStats>({
	topArtists: [],
	topTracks: [],
	topAlbums: [],
	recentTracks: [],
	timeStats: {
		today: { playCount: 0, duration: 0 },
		week: { playCount: 0, duration: 0 },
		month: { playCount: 0, duration: 0 },
		year: { playCount: 0, duration: 0 },
		all: { playCount: 0, duration: 0 },
	},
	pulseData: [],
});

const users = ref<User[]>([]);
const selectedUser = ref<User | null>(null);

const ITEMS_PER_PAGE = 20;
const currentPage = ref(1);

const totalPages = computed(() => {
	return Math.ceil(stats.value.recentTracks.length / ITEMS_PER_PAGE);
});

const maxPlayCount = computed(() => {
	return Math.max(...stats.value.pulseData.map((d) => d.playCount));
});

const paginatedTracks = computed(() => {
	const start = (currentPage.value - 1) * ITEMS_PER_PAGE;
	const end = start + ITEMS_PER_PAGE;
	return stats.value.recentTracks.slice(start, end);
});

const showDuration = ref(false);

const pulseTimeRanges: PulseTimeRange[] = [
	'12days',
	'12weeks',
	'12months',
	'12years',
];
const selectedPulseRange = ref<PulseTimeRange>('12months');

const fetchStats = async () => {
	try {
		const username = selectedUser.value?.name || null;
		const [statsData, usersData] = await Promise.all([
			fetchMusicStats(
				timeRanges.value.artists,
				selectedPulseRange.value,
				username
			),
			fetchUsers(),
		]);
		stats.value = statsData;
		users.value = usersData;
		currentPage.value = 1; // Reset to first page when fetching new data
	} catch (error) {
		console.error('Error fetching data:', error);
	}
};

// Watch for changes in the pulse range
watch(selectedPulseRange, () => {
	fetchStats();
});

// Watch for changes in the selected user
watch(selectedUser, () => {
	fetchStats();
});

onMounted(async () => {
	try {
		const username = selectedUser.value?.name || null;
		const [statsData, usersData] = await Promise.all([
			fetchMusicStats(
				timeRanges.value.artists,
				selectedPulseRange.value,
				username
			),
			fetchUsers(),
		]);
		stats.value = statsData;
		users.value = usersData;
	} catch (error) {
		console.error('Error fetching data:', error);
	}
});
</script>

<style scoped>
.stats {
	padding-top: 20px;
	max-width: 1200px;
	margin: 0 auto;
}

.stats-section {
	margin-bottom: 40px;
}

.title-icon {
	width: 20px;
	height: 20px;
	color: var(--primary-color);
}

.recent-tracks-table {
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

.stats-summary {
	display: flex;
	gap: 20px;
	margin-bottom: 20px;
	padding: 16px;
	background: var(--card-background);
	border-radius: 8px;
	box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	border: 1px solid var(--border-color);
}

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

.view-toggle {
	display: flex;
	align-items: center;
}

.toggle-btn {
	padding: 8px;
	border: 1px solid var(--border-color);
	border-radius: 4px;
	background: var(--card-background);
	color: var(--text-color);
	cursor: pointer;
	transition: all 0.2s;
	display: flex;
	align-items: center;
	justify-content: center;
}

.toggle-btn:hover {
	background: var(--background-color);
}

.toggle-btn.active {
	background: var(--primary-color);
	color: var(--background-color);
	border-color: var(--primary-color);
}

.icon {
	width: 20px;
	height: 20px;
}

.pulse-filters {
	display: flex;
	gap: 8px;
}

.pulse-filter-btn {
	padding: 6px 12px;
	border: 1px solid var(--border-color);
	border-radius: 4px;
	background: var(--card-background);
	color: var(--text-color);
	cursor: pointer;
	font-size: 0.9em;
	transition: all 0.2s;
}

.pulse-filter-btn:hover {
	background: var(--background-color);
}

.pulse-filter-btn.active {
	background: var(--primary-color);
	color: var(--background-color);
	border-color: var(--primary-color);
}

.pulse-table {
	background: var(--card-background);
	border-radius: 8px;
	overflow: hidden;
	box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	border: 1px solid var(--border-color);
}

.pulse-row {
	display: grid;
	grid-template-columns: 150px 120px 1fr;
	padding: 12px;
	border-bottom: 1px solid var(--border-color);
	align-items: center;
}

.pulse-row:last-child {
	border-bottom: none;
}

.pulse-row:hover {
	background: rgba(255, 255, 255, 0.05);
}

.pulse-period {
	color: var(--text-color);
	font-size: 0.9em;
}

.pulse-count {
	color: var(--text-secondary);
	font-size: 0.9em;
}

.fade-enter-active,
.fade-leave-active {
	transition: opacity 0.25s, transform 0.25s;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
	transform: translateY(-5px);
}

.fade-enter-to,
.fade-leave-from {
	opacity: 1;
	transform: translateY(0);
}
</style>
