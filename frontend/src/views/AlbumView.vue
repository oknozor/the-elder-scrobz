<template>
    <div class="album-view">
        <div v-if="isLoading" class="loading">Loading...</div>
        <div v-else-if="error" class="error">{{ error }}</div>
        <div v-else-if="album" class="album-content">
            <div class="album-header">
                <img
                    :src="album.cover_art_url || '/img/photo-off.svg'"
                    :alt="album.release_name"
                    class="album-image"
                    @error="handleImageError"
                />
                <div class="album-info">
                    <h1>{{ album.release_name }}</h1>
                    <p class="album-artist">by {{ album.artist_name }}</p>
                    <p v-if="album.release_date" class="release-date">
                        Released: {{ album.release_date }}
                    </p>
                    <p v-if="album.genres" class="genre">
                        Genre: {{ album.genres.join(", ") }}
                    </p>
                </div>
            </div>
            <div class="tracks-section">
                <h2>Tracks</h2>
                <div v-if="album.tracks" class="tracks-list">
                    <div
                        v-for="track in album.tracks"
                        :key="track.track_id"
                        class="track-item"
                    >
                        <span class="track-number">{{ track.track_name }}</span>
                        <span class="track-name">{{ track.track_name }}</span>
                        <span class="track-duration">{{
                            track.track_length
                        }}</span>
                    </div>
                </div>
                <p v-else class="no-tracks">No tracks available</p>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed } from "vue";
import { useRoute } from "vue-router";
import { useUsersStore } from "@/stores/usersStore";
import apiClient from "@/services/api";
import { AlbumDetails } from "@/types/music";

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
        console.log("Fetching album details...");
        const selectedUsername = usersStore.selectedUser?.username;
        const usernameParam = selectedUsername
            ? `?username=${selectedUsername}`
            : "";

        const response = await apiClient.get<AlbumDetails>(
            `/albums/${albumId.value}${usernameParam}`,
        );
        console.log(response);
        const data = response.data;

        album.value = data;
    } catch (err: any) {
        console.error("Error fetching album:", err);
        error.value =
            err.response?.status === 404
                ? "Album not found"
                : "Failed to load album data";
    } finally {
        isLoading.value = false;
    }
};

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

.tracks-list {
    background: #f9f9f9;
    border-radius: 8px;
    overflow: hidden;
}

.track-item {
    display: flex;
    align-items: center;
    padding: 12px 20px;
    transition: background-color 0.2s;
}

.track-item:hover {
    background-color: #f0f0f0;
}

.track-item:last-child {
    border-bottom: none;
}

.track-number {
    width: 40px;
    text-align: center;
    color: #666;
    font-weight: bold;
}

.track-name {
    flex: 1;
    margin-left: 20px;
}

.track-duration {
    color: #888;
    font-size: 0.9em;
}

.no-tracks {
    text-align: center;
    color: #888;
    padding: 40px;
}
</style>
