<template>
    <div class="album-view">
        <div v-if="isLoading" class="loading">Loading...</div>
        <div v-else-if="error" class="error">{{ error }}</div>
        <div v-else-if="album" class="album-content">
            <div class="album-header">
                <img
                    :src="imageUrl || '/img/photo-off.svg'"
                    :alt="album.name"
                    class="album-image"
                    @error="handleImageError"
                />
                <div class="album-info">
                    <h1>
                        {{ album.name }}
                        <span v-if="album.year" class="album-year"
                            >({{ album.year }})</span
                        >
                    </h1>
                    <p class="album-artist">by {{ album.artist_name }}</p>
                    <p v-if="album.release_date" class="release-date">
                        Released: {{ album.release_date }}
                    </p>
                    <p v-if="album.genres" class="genre">
                        Genre: {{ album.genres.join(", ") }}
                    </p>
                    <p v-if="album.description" class="album-description">
                        {{ album.description }}
                    </p>
                </div>
            </div>

            <div class="album-summary">
                <div class="album-summary-content">
                    <p><strong>Total Listens:</strong> {{ album.listens }}</p>
                    <p v-if="topListeners.length">
                        <strong>Top Listeners:</strong>
                        {{ topListeners.map((l) => l.username).join(", ") }}
                    </p>
                    <div class="album-summary-links">
                        <div v-if="album.subsonic_url">
                            <img
                                src="/img/navidrome-logo.png"
                                alt="Navidrome"
                                class="logo"
                            />
                            <a :href="album.subsonic_url" target="_blank"> </a>
                        </div>
                        <div v-if="album.musicbrainz_url">
                            <img
                                src="/img/musicbrainz-logo.png"
                                alt="MusicBrainz"
                                class="logo"
                            />
                            <a
                                :href="album.musicbrainz_url"
                                target="_blank"
                            ></a>
                        </div>
                    </div>
                </div>
            </div>

            <div class="tracks-section">
                <h2>Tracks</h2>
                <div v-if="album.tracks && album.tracks.length" class="tracks">
                    <div class="tracks-container">
                        <table class="tracks-table">
                            <thead>
                                <tr class="track-header-row">
                                    <th>#</th>
                                    <th>Track</th>
                                    <th>Duration</th>
                                    <th>Top Plays</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr
                                    v-for="(track, index) in album.tracks"
                                    :key="track.mbid"
                                    class="track-row"
                                >
                                    <td class="track-number-column">
                                        {{ index + 1 }}
                                    </td>
                                    <td class="track-name-column">
                                        <span class="track-name">{{
                                            track.name
                                        }}</span>
                                        <span
                                            v-if="track.artist_name"
                                            class="track-artist"
                                        >
                                            - {{ track.artist_name }}</span
                                        >
                                    </td>
                                    <td class="track-duration-column">
                                        <span>{{
                                            track.length
                                                ? formatTrackLength(
                                                      track.length,
                                                  )
                                                : "N/A"
                                        }}</span>
                                    </td>
                                    <td class="track-playcount-column">
                                        <div
                                            v-if="
                                                track.playcount &&
                                                track.playcount.length
                                            "
                                        >
                                            <span
                                                v-for="(
                                                    pc, idx
                                                ) in track.playcount"
                                                :key="idx"
                                                class="playcount-item"
                                            >
                                                {{ pc.username }} ({{
                                                    pc.count
                                                }})
                                                <span
                                                    v-if="
                                                        idx <
                                                        track.playcount.length -
                                                            1
                                                    "
                                                    >,
                                                </span>
                                            </span>
                                        </div>
                                        <span v-else>N/A</span>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
                <p v-else class="no-tracks">No tracks available</p>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";
import apiClient from "@/services/api";
import { useUsersStore } from "@/stores";
import type { AlbumDetails } from "@/types";
import { formatTrackLength } from "@/utils/formatter";
import { loadImage } from "@/utils/thumbail";

const route = useRoute();
const usersStore = useUsersStore();

const isLoading = ref(true);
const error = ref<string | null>(null);
const album = ref<AlbumDetails | null>(null);

const albumId = computed(() => route.params.id as string);

const handleImageError = (event: Event) => {
    const img = event.target as HTMLImageElement;
    img.src = "/img/photo-off.svg";
};

const fetchAlbum = async () => {
    if (!albumId.value) {
        error.value = "No album ID provided";
        isLoading.value = false;
        return;
    }

    isLoading.value = true;
    error.value = null;

    try {
        const selectedUsername = usersStore.selectedUser?.username;
        const usernameParam = selectedUsername
            ? `?username=${selectedUsername}`
            : "";

        const response = await apiClient.get<AlbumDetails>(
            `/albums/${albumId.value}${usernameParam}`,
        );
        album.value = response.data;
    } catch (err) {
        console.error(err);
        error.value = "Failed to load album data";
    } finally {
        isLoading.value = false;
    }
};

const imageUrl = computed(() =>
    album.value?.thumbnail_url ? loadImage(album.value.thumbnail_url) : "",
);

const topListeners = computed(() => {
    if (!album.value?.tracks) return [];
    const listenerMap = new Map<string, number>();
    album.value.tracks.forEach((track) => {
        track.playcount?.forEach((pc) => {
            listenerMap.set(
                pc.username,
                (listenerMap.get(pc.username) || 0) + pc.count,
            );
        });
    });
    return Array.from(listenerMap.entries())
        .map(([username, count]) => ({ username, count }))
        .sort((a, b) => b.count - a.count)
        .slice(0, 5);
});

watch(() => route.params.id, fetchAlbum);
watch(() => usersStore.selectedUser, fetchAlbum);
onMounted(fetchAlbum);
</script>
<style scoped>
.album-view {
    padding: 20px;
}

.loading,
.error {
    text-align: center;
    padding: 40px;
    font-size: 18px;
}

.error {
    color: #e74c3c;
}

.album-header {
    display: flex;
    gap: 30px;
    margin-bottom: 40px;
}

.album-image {
    width: 300px;
    height: 300px;
    object-fit: cover;
    border-radius: 8px;
}

.album-info h1 {
    font-size: 2.5em;
    margin: 0 0 10px 0;
}

.album-year {
    font-size: 1em;
    color: #666;
}

.album-artist {
    font-size: 1.2em;
    color: #666;
    margin: 10px 0;
}

.release-date,
.genre {
    margin: 5px 0;
    color: #888;
}

.tracks-section h2 {
    margin-bottom: 20px;
}

.tracks {
    width: 100%;
}

.tracks-container {
    width: 100%;
    background: var(--card-background);
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
}

.tracks-table {
    width: 100%;
    table-layout: fixed;
}

.track-row {
    display: grid;
    grid-template-columns: 60px 1fr 100px 120px; /* added fourth column for playcount */
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
    align-items: center;
    min-height: 48px;
    will-change: background-color;
}

.track-header-row {
    display: grid;
    grid-template-columns: 60px 1fr 100px 120px; /* match the rows */
    padding: 8px 16px;
    font-weight: 600;
    border-bottom: 1px solid var(--border-color);
}

.track-number-column {
    color: var(--text-secondary);
    font-size: 0.9em;
    text-align: center;
    font-weight: bold;
}

.track-name-column {
    color: var(--text-color);
    font-size: 0.9em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.track-duration-column {
    color: var(--text-secondary);
    font-size: 0.9em;
    text-align: right;
}

.track-playcount-column {
    color: var(--text-secondary);
    font-size: 0.9em;
    text-align: right;
}

.playcount-item {
    display: inline-block;
    margin-left: 2px;
}

.track-row:last-child {
    border-bottom: none;
}

.track-row:hover {
    background: rgba(255, 255, 255, 0.05);
}

.track-name {
    color: var(--text-color);
}

.track-playcount {
    color: var(--text-secondary);
    font-size: 0.9em;
    text-align: center;
}

.no-tracks {
    text-align: center;
    color: #888;
    padding: 40px;
}

.album-summary {
    width: 100%;
    margin: 20px 0;
    padding: 16px;
    color: var(--text-secondary);
    font-size: 0.95em;
}

.album-summary-content {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    gap: 12px 12px;
    align-items: flex-start;
}

.album-summary-links {
    display: flex;
    flex-direction: row;
    gap: 10px;
}

.album-summary-content p {
    margin: 0;
    display: flex;
    flex: 1 1 auto;
    flex-direction: row;
    gap: 5px;
}

.album-summary a {
    color: var(--accent-color);
    text-decoration: underline;
}

.logo {
    width: 24px;
    height: 24px;
    vertical-align: middle;
}
</style>
