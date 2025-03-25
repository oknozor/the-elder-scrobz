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
					<svg
						class="title-icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path
							d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"
						/>
						<path d="M19 10v2a7 7 0 0 1-14 0v-2" />
						<line x1="12" y1="19" x2="12" y2="23" />
					</svg>
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
					<svg
						class="title-icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<circle cx="12" cy="12" r="10" />
						<circle cx="12" cy="12" r="3" />
						<path d="M12 2v4" />
						<path d="M12 18v4" />
						<path d="M2 12h4" />
						<path d="M18 12h4" />
					</svg>
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
					<svg
						class="title-icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M12 2v4" />
						<path d="M12 18v4" />
						<path d="M4.93 4.93l2.83 2.83" />
						<path d="M16.24 16.24l2.83 2.83" />
						<path d="M2 12h4" />
						<path d="M18 12h4" />
						<path d="M4.93 19.07l2.83-2.83" />
						<path d="M16.24 7.76l2.83-2.83" />
					</svg>
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
				<StatItem
					v-for="(label, period) in periodLabels"
					:key="period"
					:label="label"
					:value="
						showDuration
							? formatDuration(stats.timeStats[period].duration)
							: stats.timeStats[period].playCount
					"
				/>
			</div>
			<RecentTracks
				:paginatedTracks="paginatedTracks"
				:currentPage="currentPage"
				:totalPages="totalPages"
				@change-page="currentPage = $event"
			/>
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
import StatItem from '@/components/stats/StatItem.vue';
import RecentTracks from '@/components/stats/RecentTracks.vue';

interface TimeRanges {
	artists: TimeRange;
	tracks: TimeRange;
	albums: TimeRange;
	recent: TimeRange;
}

const periodLabels = {
	today: 'Today',
	week: 'This Week',
	month: 'This Month',
	year: 'This Year',
	all: 'All Time',
};

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
