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
                    <DataIssueCard
                        title="Releases Missing Cover Art"
                        description="releases"
                        :issue-data="stats.releases_without_coverart"
                        icon-type="warning"
                    />

                    <DataIssueCard
                        title="Releases Missing Subsonic ID"
                        description="releases"
                        :issue-data="stats.releases_without_subsonic_id"
                        icon-type="info"
                    />

                    <DataIssueCard
                        title="Artists Missing Subsonic ID"
                        description="artists"
                        :issue-data="stats.artists_without_subsonic_id"
                        icon-type="info"
                    />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import AlbumIcon from "@/assets/icons/album.svg?raw";
import ArtistIcon from "@/assets/icons/artist.svg?raw";
import ErrorIcon from "@/assets/icons/error.svg?raw";
import MusicIcon from "@/assets/icons/music.svg?raw";
import RefreshIcon from "@/assets/icons/refresh.svg?raw";
import ScrobbleIcon from "@/assets/icons/scrobble.svg?raw";
import { useAdminStore } from "@/stores/adminStore";
import type { Stats } from "@/types/admin/stats";
import DataIssueCard from "./DataIssueCard.vue";
import StatsCard from "./StatCard.vue";

const adminStore = useAdminStore();

const isLoading = ref(true);
const error = ref<string | null>(null);

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
