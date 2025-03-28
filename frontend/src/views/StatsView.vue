<template>
	<div class="stats">
		<div class="time-range-container">
			<h3 class="time-range-title">Time Range</h3>
			<TimeRangeSelector
				v-model="sharedTimeRange"
				@update:modelValue="updateAllCharts"
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
						<path d="M3 3v18h18" />
						<path d="M18.4 9l-6-6-7 7" />
					</svg>
				</template>
				Overview
			</SectionHeader>
			<div class="overview-cards">
				<OverviewCard
					title="Songs listened"
					:value="songsListened"
					:percentageChange="songsPercentageChange ?? undefined"
					:comparisonText="comparisonText"
				/>
				<OverviewCard
					title="Time listened"
					:value="formatDuration(timeListened)"
					:percentageChange="timePercentageChange ?? undefined"
					:comparisonText="comparisonText"
				/>
				<OverviewCard
					title="Artists listened"
					:value="artistsListened"
					:percentageChange="artistsPercentageChange ?? undefined"
					:comparisonText="comparisonText"
				/>
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
						<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
						<circle cx="12" cy="7" r="4" />
					</svg>
				</template>
				Artists Chart
				<!-- Time range selector moved to top of page -->
			</SectionHeader>
			<StatGrid
				:items="statsStore.topArtists"
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
				<!-- Time range selector moved to top of page -->
			</SectionHeader>
			<StatGrid
				:items="statsStore.topTracks"
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
				<!-- Time range selector moved to top of page -->
			</SectionHeader>
			<StatGrid
				:items="statsStore.topAlbums"
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
				<!-- Time range selector moved to top of page -->
			</SectionHeader>
			<div class="pulse-charts">
				<PulseMixedChart :pulseData="statsStore.pulses" />
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
			</SectionHeader>
			<RecentTracks
				:tracks="statsStore.recentTracks.content"
				:total-pages="statsStore.recentTracks.totalPages"
				@load-more="handleLoadMore"
			/>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { TimeRange, MusicStats, RecentTrack } from '@/types/music';
import TimeRangeSelector from '@/components/TimeRangeSelector.vue';
import SectionHeader from '@/components/stats/SectionHeader.vue';
import StatGrid from '@/components/stats/StatGrid.vue';
import { formatDuration } from '@/utils/formatter';
import OverviewCard from '@/components/stats/OverviewCard.vue';
import PulseMixedChart from '@/components/stats/PulseMixedChart.vue';
import { useStatsStore } from '@/stores/statsStore';
import { AppUser, useUsersStore } from '@/stores/usersStore';
import RecentTracks from '@/components/stats/RecentTracks.vue';

interface TimeRanges {
	artists: TimeRange;
	tracks: TimeRange;
	albums: TimeRange;
	recent: TimeRange;
	pulses: TimeRange;
}

const sharedTimeRange = ref<TimeRange>('week');

// Keep this for backward compatibility but use sharedTimeRange for all
const timeRanges = ref<TimeRanges>({
	artists: 'week',
	tracks: 'week',
	albums: 'week',
	recent: 'week',
	pulses: 'week',
});

const stats = ref<MusicStats>({
	recentTracks: [] as RecentTrack[],
	timeStats: {
		today: { playCount: 0, duration: 0 },
		week: { playCount: 0, duration: 0 },
		month: { playCount: 0, duration: 0 },
		year: { playCount: 0, duration: 0 },
		all: { playCount: 0, duration: 0 },
	},
});
const statsStore = useStatsStore();
const usersStore = useUsersStore();

const selectedUser = ref<AppUser | null>(usersStore.selectedUser || null);

const handleLoadMore = async (page: number) => {
	await statsStore.fetchRecentTracks(
		selectedUser.value?.username || null,
		page,
		20
	);
};

// Get comparison text based on the current time range
const getComparisonText = (currentRange: TimeRange): string => {
	switch (currentRange) {
		case 'today':
			return 'than yesterday';
		case 'week':
			return 'than last week';
		case 'month':
			return 'than last month';
		case 'year':
			return 'than last year';
		default:
			return '';
	}
};

// Calculate percentage change between current and previous values
const calculatePercentageChange = (
	current: number,
	previous: number
): number => {
	if (previous === 0) return 0;
	return Math.round(((current - previous) / previous) * 100);
};

// For demonstration purposes, we'll simulate previous period data
// In a real app, this would come from the API
const getPreviousPeriodData = (currentRange: TimeRange) => {
	// Simulate previous period data with a random decrease/increase
	const currentPlayCount = stats.value.timeStats[currentRange].playCount;
	const currentDuration = stats.value.timeStats[currentRange].duration;

	// Generate a random factor between 0.7 and 1.3 for variation
	const randomFactor = 0.7 + Math.random() * 0.6;

	return {
		playCount: Math.round(currentPlayCount * randomFactor),
		duration: Math.round(currentDuration * randomFactor),
		// For artists count, we'll use a different random factor
		artistsCount: Math.round(
			(statsStore.topArtists.length * randomFactor) / 2
		),
	};
};

// Computed properties for the overview cards
const currentTimeRange = computed(() => sharedTimeRange.value);

const songsListened = computed(() => {
	// For this demo, we'll use data from the pulses array
	return statsStore.pulses.reduce((acc, pulse) => acc + pulse.listens, 0);
});

const timeListened = computed(() => {
	return Math.floor(Math.random() * 100) + 10;
});

const artistsListened = computed(() => {
	// In a real app, this would be the count of unique artists in the current time range
	// For this demo, we'll use a portion of the topArtists array length
	return Math.round(statsStore.topArtists.length / 2);
});

const previousPeriodData = computed(() => {
	return getPreviousPeriodData(currentTimeRange.value);
});

const songsPercentageChange = computed(() => {
	return currentTimeRange.value === 'all'
		? null
		: calculatePercentageChange(
				songsListened.value,
				previousPeriodData.value.playCount
		  );
});

const timePercentageChange = computed(() => {
	return currentTimeRange.value === 'all'
		? null
		: calculatePercentageChange(
				timeListened.value,
				previousPeriodData.value.duration
		  );
});

const artistsPercentageChange = computed(() => {
	return currentTimeRange.value === 'all'
		? null
		: calculatePercentageChange(
				artistsListened.value,
				previousPeriodData.value.artistsCount
		  );
});

const comparisonText = computed(() => {
	return getComparisonText(currentTimeRange.value);
});

// Function to update all charts when the time range changes
const updateAllCharts = () => {
	// Update all properties in the timeRanges object to match the shared time range
	timeRanges.value = {
		artists: sharedTimeRange.value,
		tracks: sharedTimeRange.value,
		albums: sharedTimeRange.value,
		recent: sharedTimeRange.value,
		pulses: sharedTimeRange.value,
	};

	// Fetch all stats with the new time range
	fetchAllStats(selectedUser.value?.username || null, timeRanges.value);
};

// Watch for changes in the selected user
watch(
	() => usersStore.selectedUser,
	(newValue) => {
		fetchAllStats(newValue?.username || null, timeRanges.value);
	}
);

const fetchAllStats = async (
	username: string | null,
	timeRanges: TimeRanges
) => {
	await Promise.all([
		statsStore.fetchTopArtists(username, timeRanges.artists),
		statsStore.fetchTopTracks(username, timeRanges.tracks),
		statsStore.fetchTopAlbums(username, timeRanges.albums),
		statsStore.fetchPulses(username, timeRanges.pulses),
		statsStore.fetchRecentTracks(username, 1, 20),
	]);
};

// Watch for changes in the shared time range
watch(sharedTimeRange, () => {
	updateAllCharts();
});

onMounted(async () => {
	try {
		// Initialize timeRanges with sharedTimeRange
		timeRanges.value = {
			artists: sharedTimeRange.value,
			tracks: sharedTimeRange.value,
			albums: sharedTimeRange.value,
			recent: sharedTimeRange.value,
			pulses: sharedTimeRange.value,
		};

		await fetchAllStats(
			selectedUser.value?.username || null,
			timeRanges.value
		);
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
	position: relative;
}

.time-range-container {
	position: fixed;
	left: calc(
		50% - 600px - 140px
	); /* 50% - half of max-width - container width - margin */
	top: 100px;
	z-index: 10;
	background: var(--card-background);
	padding: 15px;
	border-radius: 8px;
	box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	border: 1px solid var(--border-color);
	width: 120px;
}

.time-range-title {
	margin-top: 0;
	margin-bottom: 10px;
	font-size: 1em;
	color: var(--text-color);
	text-align: center;
}

.stats-section {
	margin-bottom: 40px;
}

.overview-cards {
	display: flex;
	gap: 20px;
	margin-bottom: 20px;
}

.title-icon {
	width: 20px;
	height: 20px;
	color: var(--primary-color);
}

.pulse-charts {
	display: flex;
	justify-content: space-between;
	align-items: center;
	gap: 20px;
}
</style>
