<template>
    <div class="toast-container">
        <TransitionGroup name="toast" tag="div">
            <div
                v-for="(event, index) in messages"
                :key="`${event.user}-${event.track_name}-${index}`"
                class="toast"
            >
                <div
                    v-if="event.type === 'PlayingNow'"
                    class="playing-now-toast"
                >
                    <div class="album-art">
                        <img
                            v-if="event.cover_art_url"
                            :src="event.cover_art_url"
                            :alt="`${event.album} cover`"
                            @error="handleImageError"
                        />
                        <div v-else class="album-art-placeholder">
                            <svg viewBox="0 0 24 24" class="music-icon">
                                <path
                                    fill="currentColor"
                                    d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"
                                />
                            </svg>
                        </div>
                    </div>
                    <div class="track-info">
                        <div class="track-header">
                            <span class="user-name">{{ event.user }}</span>
                            <span class="now-playing-text">is now playing</span>
                        </div>
                        <div class="track-name">{{ event.track_name }}</div>
                        <div class="track-details">
                            <span class="artist">{{ event.artist }}</span>
                            <span class="separator">•</span>
                            <span class="album">{{ event.album }}</span>
                        </div>
                    </div>
                </div>
            </div>
        </TransitionGroup>
    </div>
</template>

<script setup lang="ts">
import { useSseStore } from "@/stores";

const sseStore = useSseStore();
const { messages } = sseStore;

const handleImageError = (event: Event) => {
    const img = event.target as HTMLImageElement;
    img.style.display = "none";
};
</script>

<style scoped>
.toast-container {
    position: fixed;
    top: 80px;
    right: var(--app-padding);
    display: flex;
    flex-direction: column;
    gap: 12px;
    z-index: 9999;
    max-width: 400px;
}

.toast {
    background: var(--card-background);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
    overflow: hidden;
    backdrop-filter: blur(10px);
}

.playing-now-toast {
    display: flex;
    gap: 12px;
    padding: 12px;
    align-items: center;
}

.album-art {
    width: 60px;
    height: 60px;
    border-radius: 6px;
    overflow: hidden;
    flex-shrink: 0;
    background: var(--background-color);
    display: flex;
    align-items: center;
    justify-content: center;
}

.album-art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

.album-art-placeholder {
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
}

.music-icon {
    width: 24px;
    height: 24px;
}

.track-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.track-header {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.85em;
}

.user-name {
    color: var(--primary-color);
    font-weight: 600;
}

.now-playing-text {
    color: var(--text-secondary);
}

.track-name {
    color: var(--text-color);
    font-weight: 600;
    font-size: 0.95em;
    line-height: 1.3;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.track-details {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.8em;
    color: var(--text-secondary);
    overflow: hidden;
}

.artist,
.album {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.separator {
    flex-shrink: 0;
}

/* Toast transitions */
.toast-enter-active {
    transition: all 0.4s ease;
}

.toast-leave-active {
    transition: all 0.3s ease;
}

.toast-enter-from {
    opacity: 0;
    transform: translateX(100%);
}

.toast-leave-to {
    opacity: 0;
    transform: translateX(100%) scale(0.95);
}

.toast-move {
    transition: transform 0.3s ease;
}

@media screen and (max-width: 500px) {
    .toast-container {
        top: 70px;
        right: 10px;
        left: 10px;
        max-width: none;
    }

    .playing-now-toast {
        padding: 10px;
        gap: 10px;
    }

    .album-art {
        width: 50px;
        height: 50px;
    }

    .track-name {
        font-size: 0.9em;
    }

    .track-header {
        font-size: 0.8em;
    }

    .track-details {
        font-size: 0.75em;
    }
}
</style>
