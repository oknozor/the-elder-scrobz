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
                        {{ album.name
                        }}<span v-if="album.year" class="album-year">
                            ({{ album.year }})</span
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
            <div class="tracks-section">
                <h2>Tracks</h2>
                <div v-if="album.tracks" class="tracks">
                    <div class="tracks-container">
                        <table class="tracks-table">
                            <tbody>
                                <tr
                                    v-for="(track, index) in album.tracks"
                                    :key="track.number"
                                    class="track-row"
                                >
                                    <td class="track-number-column">
                                        <span>{{ index + 1 }}</span>
                                    </td>
                                    <td class="track-name-column">
                                        <span class="track-name">{{
                                            track.name
                                        }}</span>
                                        <span
                                            v-if="track.artist_display_name"
                                            class="track-artist"
                                        >
                                            - {{ track.artist_display_name }}
                                        </span>
                                    </td>
                                    <td class="track-duration-column">
                                        <span>{{
                                            formatTrackLength(track.length)
                                        }}</span>
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
import { isAxiosError } from "axios";
import { computed, onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";
import apiClient from "@/services/api";
import { useUsersStore } from "@/stores/usersStore";
import type { AlbumDetails } from "@/types/music";
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
        error.value = "No artist ID provided";
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
        const data = response.data;
        album.value = data;
    } catch (err: unknown) {
        console.error("Error fetching album:", err);
        if (isAxiosError(err)) {
            error.value =
                err.response?.status === 404
                    ? "Album not found"
                    : "Failed to load album data";
        } else {
            error.value = "Unexpected error";
        }
    } finally {
        isLoading.value = false;
    }
};
const imageUrl = computed(() => {
    if (album.value?.thumbnail_url) {
        return loadImage(album.value?.thumbnail_url);
    }
    return "";
});

watch(
    () => route.params.id,
    () => {
        if (route.params.id) {
            fetchAlbum();
        }
    },
);

watch(
    () => usersStore.selectedUser,
    () => {
        fetchAlbum();
    },
);

onMounted(() => {
    fetchAlbum();
});
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
    grid-template-columns: 60px 1fr 100px;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
    align-items: center;
    min-height: 48px;
    will-change: background-color;
}

.track-row:last-child {
    border-bottom: none;
}

.track-row:hover {
    background: rgba(255, 255, 255, 0.05);
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

.track-name {
    color: var(--text-color);
}

.track-duration-column {
    color: var(--text-secondary);
    font-size: 0.9em;
    text-align: right;
}

.no-tracks {
    text-align: center;
    color: #888;
    padding: 40px;
}
</style>
