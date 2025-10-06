<template>
    <div class="admin-stats">
        <div class="stats-header">
            <h2>Database Statistics</h2>
            <button
                class="refresh-button"
                @click="refreshStats"
                :disabled="isLoading"
                :class="{ spinning: isLoading }"
            >
                <div class="refresh-icon" v-html="RefreshIcon"></div>
                Refresh
            </button>
        </div>

        <div v-if="isLoading" class="loading-container">
            <div class="spinner"></div>
            <p>Loading statistics...</p>
        </div>

        <div v-else-if="error" class="error-container">
            <div class="error-icon" v-html="ErrorIcon" />
            <h3>Error Loading Stats</h3>
            <p>{{ error }}</p>
            <button class="retry-button" @click="refreshStats">
                Try Again
            </button>
        </div>

        <div v-else-if="stats" class="stats-content">
            <div class="stats-overview">
                <StatsCard
                    title="Total Tracks"
                    :icon="MusicIcon"
                    :value="stats.total_track_count"
                    :color="'var(--primary-color)'"
                    :background="'rgba(66, 185, 131, 0.2)'"
                />
                <StatsCard
                    title="Total Releases"
                    :icon="AlbumIcon"
                    :value="stats.total_releases_count"
                    :color="'#3b82f6'"
                    :background="'rgba(59, 130, 246, 0.2)'"
                />
                <StatsCard
                    title="Total Artists"
                    :icon="ArtistIcon"
                    :value="stats.total_releases_count"
                    :color="'#f59e0b'"
                    :background="'rgba(245, 158, 11, 0.2)'"
                />
                <StatsCard
                    title="Total Scrobbles"
                    :icon="ScrobbleIcon"
                    :value="stats.total_scrobble_count"
                    :color="'#8b45c1'"
                    :background="'rgba(139, 69, 193, 0.2)'"
                />
            </div>

            <div class="processing-stats">
                <h3>Scrobble Processing</h3>
                <div class="processing-grid">
                    <div class="processing-item">
                        <div class="processing-label">Raw Scrobbles</div>
                        <div class="processing-value">
                            {{ formatNumber(stats.total_raw_scrobble_count) }}
                        </div>
                    </div>
                    <div class="processing-item">
                        <div class="processing-label">Processed Scrobbles</div>
                        <div class="processing-value">
                            {{ formatNumber(stats.total_scrobble_count) }}
                        </div>
                    </div>
                    <div class="processing-item">
                        <div class="processing-label">Processing Rate</div>
                        <div class="processing-value">
                            {{ processingRate }}%
                        </div>
                    </div>
                </div>

                <div
                    v-if="stats.unparsable_scrobbles.count > 0"
                    class="processing-issues"
                >
                    <div class="issue-item error">
                        <span class="issue-count">{{
                            stats.unparsable_scrobbles.count
                        }}</span>
                        <span class="issue-label">Unparseable Scrobbles</span>
                    </div>
                </div>
            </div>

            <div class="issues-section">
                <h3>Data Issues</h3>
                <div class="issues-grid">
                    <div
                        v-if="stats.releases_without_coverart.count > 0"
                        class="issue-card"
                        @click="toggleExpansion('releases_coverart')"
                    >
                        <div class="issue-header">
                            <div class="issue-info">
                                <div class="issue-icon warning">
                                    <svg
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                    >
                                        <path
                                            d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"
                                        />
                                        <line x1="12" y1="9" x2="12" y2="13" />
                                        <line
                                            x1="12"
                                            y1="17"
                                            x2="12.01"
                                            y2="17"
                                        />
                                    </svg>
                                </div>
                                <div>
                                    <h4>Releases Missing Cover Art</h4>
                                    <p>
                                        {{
                                            stats.releases_without_coverart
                                                .count
                                        }}
                                        releases
                                    </p>
                                </div>
                            </div>
                            <div
                                class="expand-icon"
                                :class="{
                                    expanded:
                                        expandedSections.releases_coverart,
                                }"
                            >
                                <svg
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path d="M6 9l6 6 6-6" />
                                </svg>
                            </div>
                        </div>
                        <div
                            v-if="expandedSections.releases_coverart"
                            class="issue-details"
                        >
                            <div class="ids-list">
                                <div
                                    v-for="id in stats.releases_without_coverart.ids.slice(
                                        0,
                                        10,
                                    )"
                                    :key="id"
                                    class="id-item"
                                >
                                    {{ id }}
                                </div>
                                <div
                                    v-if="
                                        stats.releases_without_coverart.ids
                                            .length > 10
                                    "
                                    class="more-items"
                                >
                                    +{{
                                        stats.releases_without_coverart.ids
                                            .length - 10
                                    }}
                                    more...
                                </div>
                            </div>
                        </div>
                    </div>

                    <div
                        v-if="stats.artists_without_thumbnail.count > 0"
                        class="issue-card"
                        @click="toggleExpansion('artists_thumbnails')"
                    >
                        <div class="issue-header">
                            <div class="issue-info">
                                <div class="issue-icon warning">
                                    <svg
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                    >
                                        <path
                                            d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"
                                        />
                                        <line x1="12" y1="9" x2="12" y2="13" />
                                        <line
                                            x1="12"
                                            y1="17"
                                            x2="12.01"
                                            y2="17"
                                        />
                                    </svg>
                                </div>
                                <div>
                                    <h4>Artists Missing Thumbnails</h4>
                                    <p>
                                        {{
                                            stats.artists_without_thumbnail
                                                .count
                                        }}
                                        artists
                                    </p>
                                </div>
                            </div>
                            <div
                                class="expand-icon"
                                :class="{
                                    expanded:
                                        expandedSections.artists_thumbnails,
                                }"
                            >
                                <svg
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path d="M6 9l6 6 6-6" />
                                </svg>
                            </div>
                        </div>
                        <div
                            v-if="expandedSections.artists_thumbnails"
                            class="issue-details"
                        >
                            <div class="ids-list">
                                <div
                                    v-for="id in stats.artists_without_thumbnail.ids.slice(
                                        0,
                                        10,
                                    )"
                                    :key="id"
                                    class="id-item"
                                >
                                    {{ id }}
                                </div>
                                <div
                                    v-if="
                                        stats.artists_without_thumbnail.ids
                                            .length > 10
                                    "
                                    class="more-items"
                                >
                                    +{{
                                        stats.artists_without_thumbnail.ids
                                            .length - 10
                                    }}
                                    more...
                                </div>
                            </div>
                        </div>
                    </div>

                    <div
                        v-if="stats.releases_without_subsonic_id.count > 0"
                        class="issue-card"
                        @click="toggleExpansion('releases_subsonic')"
                    >
                        <div class="issue-header">
                            <div class="issue-info">
                                <div class="issue-icon info">
                                    <svg
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                    >
                                        <circle cx="12" cy="12" r="10" />
                                        <line x1="12" y1="16" x2="12" y2="12" />
                                        <line
                                            x1="12"
                                            y1="8"
                                            x2="12.01"
                                            y2="8"
                                        />
                                    </svg>
                                </div>
                                <div>
                                    <h4>Releases Missing Subsonic ID</h4>
                                    <p>
                                        {{
                                            stats.releases_without_subsonic_id
                                                .count
                                        }}
                                        releases
                                    </p>
                                </div>
                            </div>
                            <div
                                class="expand-icon"
                                :class="{
                                    expanded:
                                        expandedSections.releases_subsonic,
                                }"
                            >
                                <svg
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path d="M6 9l6 6 6-6" />
                                </svg>
                            </div>
                        </div>
                        <div
                            v-if="expandedSections.releases_subsonic"
                            class="issue-details"
                        >
                            <div class="ids-list">
                                <div
                                    v-for="id in stats.releases_without_subsonic_id.ids.slice(
                                        0,
                                        10,
                                    )"
                                    :key="id"
                                    class="id-item"
                                >
                                    {{ id }}
                                </div>
                                <div
                                    v-if="
                                        stats.releases_without_subsonic_id.ids
                                            .length > 10
                                    "
                                    class="more-items"
                                >
                                    +{{
                                        stats.releases_without_subsonic_id.ids
                                            .length - 10
                                    }}
                                    more...
                                </div>
                            </div>
                        </div>
                    </div>

                    <div
                        v-if="stats.artists_without_subsonic_id.count > 0"
                        class="issue-card"
                        @click="toggleExpansion('artists_subsonic')"
                    >
                        <div class="issue-header">
                            <div class="issue-info">
                                <div class="issue-icon info">
                                    <svg
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                    >
                                        <circle cx="12" cy="12" r="10" />
                                        <line x1="12" y1="16" x2="12" y2="12" />
                                        <line
                                            x1="12"
                                            y1="8"
                                            x2="12.01"
                                            y2="8"
                                        />
                                    </svg>
                                </div>
                                <div>
                                    <h4>Artists Missing Subsonic ID</h4>
                                    <p>
                                        {{
                                            stats.artists_without_subsonic_id
                                                .count
                                        }}
                                        artists
                                    </p>
                                </div>
                            </div>
                            <div
                                class="expand-icon"
                                :class="{
                                    expanded: expandedSections.artists_subsonic,
                                }"
                            >
                                <svg
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path d="M6 9l6 6 6-6" />
                                </svg>
                            </div>
                        </div>
                        <div
                            v-if="expandedSections.artists_subsonic"
                            class="issue-details"
                        >
                            <div class="ids-list">
                                <div
                                    v-for="id in stats.artists_without_subsonic_id.ids.slice(
                                        0,
                                        10,
                                    )"
                                    :key="id"
                                    class="id-item"
                                >
                                    {{ id }}
                                </div>
                                <div
                                    v-if="
                                        stats.artists_without_subsonic_id.ids
                                            .length > 10
                                    "
                                    class="more-items"
                                >
                                    +{{
                                        stats.artists_without_subsonic_id.ids
                                            .length - 10
                                    }}
                                    more...
                                </div>
                            </div>
                        </div>
                    </div>

                    <div
                        v-if="stats.tracks_without_subsonic_id.count > 0"
                        class="issue-card"
                        @click="toggleExpansion('tracks_subsonic')"
                    >
                        <div class="issue-header">
                            <div class="issue-info">
                                <div class="issue-icon info">
                                    <svg
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                    >
                                        <circle cx="12" cy="12" r="10" />
                                        <line x1="12" y1="16" x2="12" y2="12" />
                                        <line
                                            x1="12"
                                            y1="8"
                                            x2="12.01"
                                            y2="8"
                                        />
                                    </svg>
                                </div>
                                <div>
                                    <h4>Tracks Missing Subsonic ID</h4>
                                    <p>
                                        {{
                                            stats.tracks_without_subsonic_id
                                                .count
                                        }}
                                        tracks
                                    </p>
                                </div>
                            </div>
                            <div
                                class="expand-icon"
                                :class="{
                                    expanded: expandedSections.tracks_subsonic,
                                }"
                            >
                                <svg
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path d="M6 9l6 6 6-6" />
                                </svg>
                            </div>
                        </div>
                        <div
                            v-if="expandedSections.tracks_subsonic"
                            class="issue-details"
                        >
                            <div class="ids-list">
                                <div
                                    v-for="id in stats.tracks_without_subsonic_id.ids.slice(
                                        0,
                                        10,
                                    )"
                                    :key="id"
                                    class="id-item"
                                >
                                    {{ id }}
                                </div>
                                <div
                                    v-if="
                                        stats.tracks_without_subsonic_id.ids
                                            .length > 10
                                    "
                                    class="more-items"
                                >
                                    +{{
                                        stats.tracks_without_subsonic_id.ids
                                            .length - 10
                                    }}
                                    more...
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useAdminStore } from "@/stores/adminStore";
import type { Stats } from "@/types/admin/stats";
import StatsCard from "./StatCard.vue";
import MusicIcon from "@/assets/icons/music.svg?raw";
import ArtistIcon from "@/assets/icons/artist.svg?raw";
import AlbumIcon from "@/assets/icons/album.svg?raw";
import ScrobbleIcon from "@/assets/icons/scrobble.svg?raw";
import RefreshIcon from "@/assets/icons/refresh.svg?raw";
import ErrorIcon from "@/assets/icons/error.svg?raw";

const adminStore = useAdminStore();

const isLoading = ref(true);
const error = ref<string | null>(null);
const expandedSections = ref<Record<string, boolean>>({});

const stats = computed<Stats | null>(() => adminStore.stats);

const processingRate = computed(() => {
    if (
        !stats.value ||
        !stats.value.total_raw_scrobble_count ||
        stats.value.total_raw_scrobble_count === 0
    )
        return 0;
    return Math.round(
        (stats.value.total_scrobble_count /
            stats.value.total_raw_scrobble_count) *
            100,
    );
});

const refreshStats = async () => {
    isLoading.value = true;
    error.value = null;

    try {
        await adminStore.fetchStats();
    } catch (err) {
        error.value =
            err instanceof Error ? err.message : "Failed to load statistics";
    } finally {
        isLoading.value = false;
    }
};

const formatNumber = (num: number | undefined | null): string => {
    if (num === undefined || num === null || Number.isNaN(num)) return "0";
    return num.toLocaleString();
};

const toggleExpansion = (section: string) => {
    expandedSections.value[section] = !expandedSections.value[section];
};

onMounted(() => {
    refreshStats();
});
</script>

<style scoped>
.admin-stats {
    max-width: 1200px;
    margin: 0 auto;
}

.stats-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 32px;
}

.stats-header h2 {
    color: var(--text-color);
    margin: 0;
    font-size: 1.8em;
}

.refresh-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: var(--card-background);
    color: var(--text-color);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9em;
    transition: all 0.2s;
}

.refresh-button:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.05);
    transform: translateY(-1px);
}

.refresh-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.refresh-icon {
    width: 16px;
    height: 16px;
    transition: transform 0.2s;
}

.refresh-button.spinning .refresh-icon {
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

.loading-container,
.error-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    text-align: center;
    background: var(--card-background);
    border-radius: 8px;
    border: 1px solid var(--border-color);
}

.spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border-color);
    border-top: 3px solid var(--primary-color);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 16px;
}

.error-container {
    color: var(--text-color);
}

.error-icon {
    width: 48px;
    height: 48px;
    color: #ef4444;
    margin-bottom: 16px;
}

.error-container h3 {
    margin: 0 0 8px 0;
    color: var(--text-color);
}

.error-container p {
    margin: 0 0 16px 0;
    color: var(--text-secondary);
}

.retry-button {
    padding: 8px 16px;
    background: var(--primary-color);
    color: var(--background-color);
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
}

.retry-button:hover {
    opacity: 0.9;
}

.stats-content {
    display: flex;
    flex-direction: column;
    gap: 32px;
}

.stats-overview {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 20px;
}

.processing-stats {
    background: var(--card-background);
    border-radius: 8px;
    padding: 24px;
    border: 1px solid var(--border-color);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.processing-stats h3 {
    margin: 0 0 16px 0;
    color: var(--text-color);
    font-size: 1.2em;
}

.processing-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 16px;
    margin-bottom: 16px;
}

.processing-item {
    text-align: center;
    padding: 16px;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.1);
}

.processing-label {
    color: var(--text-secondary);
    font-size: 0.85em;
    margin-bottom: 4px;
}

.processing-value {
    color: var(--text-color);
    font-size: 1.4em;
    font-weight: bold;
}

.processing-issues {
    padding-top: 16px;
    border-top: 1px solid var(--border-color);
}

.issue-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 4px;
}

.issue-item.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
}

.issue-count {
    font-weight: bold;
    color: #ef4444;
}

.issue-label {
    color: var(--text-color);
    font-size: 0.9em;
}

.issues-section {
    background: var(--card-background);
    border-radius: 8px;
    padding: 24px;
    border: 1px solid var(--border-color);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.issues-section h3 {
    margin: 0 0 20px 0;
    color: var(--text-color);
    font-size: 1.2em;
}

.issues-grid {
    display: flex;
    flex-direction: column;
    gap: 12px;
}

.issue-card {
    background: rgba(255, 255, 255, 0.03);
    border-radius: 6px;
    border: 1px solid var(--border-color);
    cursor: pointer;
    transition: all 0.2s;
}

.issue-card:hover {
    background: rgba(255, 255, 255, 0.05);
}

.issue-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px;
}

.issue-info {
    display: flex;
    align-items: center;
    gap: 12px;
}

.issue-icon {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    flex-shrink: 0;
}

.issue-icon.warning {
    color: #f59e0b;
}

.issue-icon.info {
    color: #3b82f6;
}

.issue-icon svg {
    width: 16px;
    height: 16px;
}

.issue-info h4 {
    margin: 0 0 2px 0;
    color: var(--text-color);
    font-size: 0.95em;
}

.issue-info p {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.85em;
}

.expand-icon {
    width: 20px;
    height: 20px;
    color: var(--text-secondary);
    transition: transform 0.2s;
}

.expand-icon.expanded {
    transform: rotate(180deg);
}

.expand-icon svg {
    width: 100%;
    height: 100%;
}

.issue-details {
    border-top: 1px solid var(--border-color);
    padding: 16px;
}

.ids-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.id-item {
    padding: 4px 8px;
    background: rgba(0, 0, 0, 0.2);
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.8em;
    color: var(--text-secondary);
}

.more-items {
    padding: 4px 8px;
    color: var(--text-secondary);
    font-style: italic;
    font-size: 0.85em;
}

@media (max-width: 768px) {
    .stats-header {
        flex-direction: column;
        gap: 16px;
        align-items: stretch;
    }

    .refresh-button {
        align-self: center;
    }

    .stats-overview {
        grid-template-columns: 1fr;
    }

    .stat-card {
        padding: 16px;
    }

    .processing-grid {
        grid-template-columns: 1fr;
    }
}
</style>
