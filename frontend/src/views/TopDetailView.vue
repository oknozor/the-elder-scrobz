<template>
	<div class="top-albums-view">
		<div>
			<h1>{{ title }}</h1>
		</div>
		<div class="time-range-container">
			<h3 class="time-range-title">Time Range</h3>
			<TimeRangeSelector
				v-model="sharedTimeRange"
				@update:modelValue="updateChart"
			/>
		</div>
		<InfiniteCardGrid
			:items="paginatedData"
			:loading="isLoading"
			:link="{ name: linkName }"
			@load-more="loadMoreItems"
		/>
	</div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import InfiniteCardGrid from "@/components/InfiniteCardGrid.vue";
import TimeRangeSelector from "@/components/TimeRangeSelector.vue";
import { useWindowWidth } from "@/composables/useWindowWidth";
import { useStatsStore } from "@/stores/statsStore";
import { type AppUser, useUsersStore } from "@/stores/usersStore";
import type {
    Album,
    Artist,
    PaginatedResponse,
    TimeRange,
    Track,
} from "@/types/music";

const windowSize = useWindowWidth();
const ELEMENT_PER_PAGE = windowSize.value > 650 ? 15 : 5;
const statsStore = useStatsStore();
const usersStore = useUsersStore();
const route = useRoute();

const currentRoute = computed(() => {
    return route.name;
});

const linkName = computed(() => {
    return currentRoute.value === "topAlbums"
        ? "album"
        : currentRoute.value === "topTracks"
          ? "track"
          : "artist";
});

const title = computed(() => {
    return currentRoute.value === "topAlbums"
        ? "Top Albums"
        : currentRoute.value === "topTracks"
          ? "Top Tracks"
          : "Top Artists";
});
const selectedUser = ref<AppUser | null>(usersStore.selectedUser || null);
const sharedTimeRange = ref<TimeRange>("week");
const isLoading = ref(false);

const paginatedData = ref<PaginatedResponse<Album | Track | Artist>>({
    data: [],
    page: 1,
    page_size: ELEMENT_PER_PAGE,
    total: 0,
});

const loadMoreItems = async (page: number) => {
    isLoading.value = true;
    try {
        if (currentRoute.value === "topAlbums") {
            await statsStore.fetchTopAlbumsForAlbumsView(
                selectedUser.value?.username || null,
                sharedTimeRange.value,
                page,
                ELEMENT_PER_PAGE,
            );
            paginatedData.value = {
                data: statsStore.topAlbums.data,
                page: statsStore.topAlbums.page,
                page_size: statsStore.topAlbums.page_size,
                total: statsStore.topAlbums.total,
            };
        } else if (currentRoute.value === "topTracks") {
            await statsStore.fetchTopTracksForTracksView(
                selectedUser.value?.username || null,
                sharedTimeRange.value,
                page,
                ELEMENT_PER_PAGE,
            );
            paginatedData.value = {
                data: statsStore.topTracks.data,
                page: statsStore.topTracks.page,
                page_size: statsStore.topTracks.page_size,
                total: statsStore.topTracks.total,
            };
        } else if (currentRoute.value === "topArtists") {
            await statsStore.fetchTopArtistsForArtistsView(
                selectedUser.value?.username || null,
                sharedTimeRange.value,
                page,
                ELEMENT_PER_PAGE,
            );
            paginatedData.value = {
                data: statsStore.topArtists.data,
                page: statsStore.topArtists.page,
                page_size: statsStore.topArtists.page_size,
                total: statsStore.topArtists.total,
            };
        }
    } catch (error) {
        console.error("Failed to load items", error);
    } finally {
        isLoading.value = false;
    }
};

const updateChart = async () => {
    paginatedData.value = {
        data: [],
        page: 1,
        page_size: ELEMENT_PER_PAGE,
        total: 0,
    };
    loadMoreItems(1);
};

onMounted(() => {
    loadMoreItems(1);
});
</script>

<style scoped>
.top-albums-view {
	display: flex;
	flex-direction: column;
	gap: 1rem;
	height: calc(100vh - calc(2 * var(--header-height)) - var(--app-padding));
	overflow: hidden;
}
</style>
