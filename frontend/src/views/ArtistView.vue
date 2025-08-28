<template>
    <div class="artist-page">
        <div v-if="isLoading" class="loading-container">
            <div class="loading-spinner"></div>
            <p>Loading artist...</p>
        </div>

        <div v-else-if="error" class="error-container">
            <h2>Error Loading Artist</h2>
            <p>{{ error }}</p>
            <button @click="fetchArtist" class="retry-button">Retry</button>
        </div>

        <div v-else-if="artist" class="artist-content">
            <div class="artist-header">
                <img
                    :src="artist.thumbnail_url || '/img/photo-off.svg'"
                    :alt="artist.name"
                    class="artist-image"
                    @error="handleImageError"
                />
                <div class="artist-info">
                    <h1>{{ artist.name }}</h1>
                    <div class="artist-description">
                        <p>{{ artist.description }}</p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { isAxiosError } from "axios";
import { computed, onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";
import apiClient from "@/services/api";
import { useUsersStore } from "@/stores";
import type { Artist } from "@/types";

const route = useRoute();
const usersStore = useUsersStore();

const isLoading = ref(true);
const error = ref<string | null>(null);
const artist = ref<Artist | null>(null);

const artistId = computed(() => route.params.id as string);

const handleImageError = (event: Event) => {
    const img = event.target as HTMLImageElement;
    img.src = "/img/photo-off.svg";
};

const fetchArtist = async () => {
    if (!artistId.value) {
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

        const response = await apiClient.get(
            `/artists/${artistId.value}${usernameParam}`,
        );
        const data = response.data;

        artist.value = data.artist || data;
    } catch (err: unknown) {
        console.error("Error fetching artist:", err);
        if (isAxiosError(err)) {
            error.value =
                err.response?.status === 404
                    ? "Artist not found"
                    : "Failed to load artist data";
        } else {
            error.value = "Unexpected error";
        }
    } finally {
        isLoading.value = false;
    }
};

watch(
    () => route.params.id,
    () => {
        if (route.params.id) {
            fetchArtist();
        }
    },
);

watch(
    () => usersStore.selectedUser,
    () => {
        fetchArtist();
    },
);

onMounted(() => {
    fetchArtist();
});
</script>

<style scoped>
.artist-page {
    padding: 20px;
    max-width: 1200px;
    margin: 0 auto;
    min-height: 60vh;
}

.loading-container,
.error-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    text-align: center;
}

.loading-spinner {
    width: 40px;
    height: 40px;
    border: 3px solid rgba(255, 255, 255, 0.1);
    border-top: 3px solid var(--primary-color);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 20px;
}

@keyframes spin {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg);
    }
}

.retry-button {
    padding: 10px 20px;
    background: var(--primary-color);
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    margin-top: 10px;
    transition: background 0.2s;
}

.retry-button:hover {
    background: var(--primary-color-hover);
}

.artist-header {
    display: flex;
    gap: 30px;
    margin-bottom: 40px;
    padding: 20px;
    background: var(--card-background);
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
}

.artist-image {
    width: 200px;
    height: 200px;
    border-radius: 8px;
    object-fit: cover;
    background: var(--card-background);
}

.artist-info {
    display: flex;
    flex-direction: column;
    justify-content: center;
}

h1 {
    color: var(--text-color);
    margin: 0 0 8px 0;
    font-size: 1.8em;
}
</style>
