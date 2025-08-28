<template>
    <div class="top-albums-view">
        <div>
            <h1>{{ title }}</h1>
        </div>
        <div class="time-range-container">
            <h3 class="time-range-title">Time Range</h3>
            <TimeRangeSelector />
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
import { storeToRefs } from "pinia";
import { computed, onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";
import InfiniteCardGrid from "@/components/InfiniteCardGrid.vue";
import TimeRangeSelector from "@/components/TimeRangeSelector.vue";
import { useWindowWidth } from "@/composables/useWindowWidth";
import { useStatsStore, useTimeRangeStore, useUsersStore } from "@/stores";
import type { Album, Artist, PaginatedResponse, Track } from "@/types";

const windowSize = useWindowWidth();
const ELEMENT_PER_PAGE = windowSize.value > 650 ? 15 : 5;
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

const statsStore = useStatsStore();
const { selectedUser } = storeToRefs(useUsersStore());
const { sharedTimeRange } = storeToRefs(useTimeRangeStore());
const { topAlbums, topTracks, topArtists } = storeToRefs(statsStore);

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
                data: topAlbums.value.data,
                page: topAlbums.value.page,
                page_size: topAlbums.value.page_size,
                total: topAlbums.value.total,
            };
        } else if (currentRoute.value === "topTracks") {
            await statsStore.fetchTopTracksForTracksView(
                selectedUser.value?.username || null,
                sharedTimeRange.value,
                page,
                ELEMENT_PER_PAGE,
            );
            paginatedData.value = {
                data: topTracks.value.data,
                page: topTracks.value.page,
                page_size: topTracks.value.page_size,
                total: topTracks.value.total,
            };
        } else if (currentRoute.value === "topArtists") {
            await statsStore.fetchTopArtistsForArtistsView(
                selectedUser.value?.username || null,
                sharedTimeRange.value,
                page,
                ELEMENT_PER_PAGE,
            );
            paginatedData.value = {
                data: topArtists.value.data,
                page: topArtists.value.page,
                page_size: topArtists.value.page_size,
                total: topArtists.value.total,
            };
        }
    } catch (error) {
        console.error("Failed to load items", error);
    } finally {
        isLoading.value = false;
    }
};

watch(sharedTimeRange, () => {
    console.log("Time range changed");
    updateChart();
});

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
