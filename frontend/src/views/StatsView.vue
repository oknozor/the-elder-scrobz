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
 				:value="statsStore.overview.track_listened"
 				:percentageChange="statsStore.overview.track_listened_percentage_increase ?? 0"
 				:comparisonText="comparisonText"
 			/>
 			<OverviewCard
 				title="Time listened"
 				:value="formatDuration(statsStore.overview.time_listened)"
 				:percentageChange="statsStore.overview.time_listened_percentage_increase ?? 0 "
 				:comparisonText="comparisonText"
 			/>
 			<OverviewCard
 				title="Artists listened"
 				:value="statsStore.overview.artist_listened"
 				:percentageChange="statsStore.overview.artist_listened_percentage_increase ?? 0"
 				:comparisonText="comparisonText"
 			/>
 		</div>
 		<Suspense>
 			<template #default>
 				<div class="pulse-charts">
 					<PulseMixedChart :pulseData="statsStore.pulses" />
 				</div>
 			</template>
 			<template #fallback>
 				<div class="loading-placeholder">
 					<span class="loader-animation"></span>
 				</div>
 			</template>
 		</Suspense>
		</div>

		<div class="stats-section">
			<SectionHeader :link="{ name: 'topArtists' }">
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
			<Suspense>
				<template #default>
					<StatGrid
						:items="statsStore.topArtistsForStatsView"
						:limit="currentWidth > 500 ? 15 : 10"
						:link="{ name: 'artist' }"
					/>
				</template>
				<template #fallback>
					<div class="loading-placeholder">
						<span class="loader-animation"></span>
					</div>
				</template>
			</Suspense>
		</div>

		<div class="stats-section">
			<SectionHeader :link="{ name: 'topTracks' }">
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
			<Suspense>
				<template #default>
					<StatGrid
						:items="statsStore.topTracksForStatsView"
						:limit="currentWidth > 500 ? 15 : 10"
						:link="{ name: 'track' }"
					/>
				</template>
				<template #fallback>
					<div class="loading-placeholder">
						<span class="loader-animation"></span>
					</div>
				</template>
			</Suspense>
		</div>

		<div class="stats-section">
			<SectionHeader :link="{ name: 'topAlbums' }">
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
			</SectionHeader>
			<Suspense>
				<template #default>
					<StatGrid
						:items="statsStore.topAlbumsForStatsView"
						:limit="currentWidth > 500 ? 15 : 10"
						:link="{ name: 'album' }"
					/>
				</template>
				<template #fallback>
					<div class="loading-placeholder">
						<span class="loader-animation"></span>
					</div>
				</template>
			</Suspense>
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
			<Suspense>
				<template #default>
					<RecentTracks
						:tracks="statsStore.recentTracks.data"
						:total-pages="
							Math.ceil(statsStore.recentTracks.total / 15)
						"
						@load-more="handleLoadMore"
					/>
				</template>
				<template #fallback>
					<div class="loading-placeholder">
						<span class="loader-animation"></span>
					</div>
				</template>
			</Suspense>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import OverviewCard from "@/components/stats/OverviewCard.vue";
import PulseMixedChart from "@/components/stats/PulseMixedChart.vue";
import RecentTracks from "@/components/stats/RecentTracks.vue";
import SectionHeader from "@/components/stats/SectionHeader.vue";
import StatGrid from "@/components/stats/StatGrid.vue";
import TimeRangeSelector from "@/components/TimeRangeSelector.vue";
import { useStatsStore } from "@/stores/statsStore";
import { type AppUser, useUsersStore } from "@/stores/usersStore";
import type { TimeRange } from "@/types/music";
import { formatDuration } from "@/utils/formatter";

const sharedTimeRange = ref<TimeRange>("week");

const currentWidth = ref(window.innerWidth);

const statsStore = useStatsStore();
const usersStore = useUsersStore();

const selectedUser = ref<AppUser | null>(usersStore.selectedUser || null);

const handleLoadMore = async (page: number) => {
    await statsStore.fetchRecentTracks(
        selectedUser.value?.username || null,
        page,
        20,
    );
};

const getComparisonText = (currentRange: TimeRange): string => {
    switch (currentRange) {
        case "today":
            return "than yesterday";
        case "week":
            return "than last week";
        case "month":
            return "than last month";
        case "year":
            return "than last year";
        default:
            return "";
    }
};

const currentTimeRange = computed(() => sharedTimeRange.value);

const comparisonText = computed(() => {
    return getComparisonText(currentTimeRange.value);
});

const updateAllCharts = () => {
    fetchAllStats(selectedUser.value?.username || null);
};

watch(
    () => usersStore.selectedUser,
    (newValue) => {
        fetchAllStats(newValue?.username || null);
    },
);

const fetchAllStats = async (username: string | null) => {
    await Promise.all([
        statsStore.fetchOverview(username, sharedTimeRange.value),
        statsStore.fetchTopArtistsForStatsView(username, sharedTimeRange.value),
        statsStore.fetchTopTracksForStatsView(username, sharedTimeRange.value),
        statsStore.fetchTopAlbumsForStatsView(username, sharedTimeRange.value),
        statsStore.fetchPulses(username, sharedTimeRange.value),
        statsStore.fetchRecentTracks(username, 1, 20),
    ]);
};

// Watch for changes in the shared time range
watch(sharedTimeRange, () => {
    updateAllCharts();
});

onMounted(async () => {
    try {
        await fetchAllStats(selectedUser.value?.username || null);
    } catch (error) {
        console.error("Error fetching data:", error);
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

.loading-placeholder {
	display: flex;
	justify-content: center;
	align-items: center;
	border-radius: 8px;
	padding: 20px;
	height: 150px;
}
.loader-animation {
	border: 4px solid var(--border-color);
	border-top: 4px solid var(--primary-color);
	border-radius: 50%;
	width: 24px;
	height: 24px;
	animation: spin 1s linear infinite;
}

@keyframes spin {
	0% {
		transform: rotate(0deg);
	}
	100% {
		transform: rotate(360deg);
	}
}

@media screen and (max-width: 500px) {
	.overview-cards {
		gap: 10px;
		flex-wrap: wrap;
	}
}
</style>
