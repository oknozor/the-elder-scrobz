<template>
    <div class="playing-now-card">
        <div class="album-art">
            <img
                v-if="playingNow.cover_art_url"
                :src="playingNow.cover_art_url"
                :alt="`${playingNow.album} cover`"
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
                <span class="user-name">{{ playingNow.user }}</span>
                <span class="now-playing-text">is now playing</span>
            </div>
            <div class="track-name">{{ playingNow.track_name }}</div>
            <div class="track-details">
                <span class="artist">{{ playingNow.artist }}</span>
                <span class="separator">•</span>
                <span class="album">{{ playingNow.album }}</span>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
interface PlayingNow {
    user: string;
    track_name: string;
    artist: string;
    album: string;
    cover_art_url?: string;
}

interface Props {
    playingNow: PlayingNow;
}

defineProps<Props>();

const handleImageError = (event: Event) => {
    const img = event.target as HTMLImageElement;
    img.style.display = "none";
};
</script>

<style scoped>
.playing-now-card {
    background: var(--card-background);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
    overflow: hidden;
    backdrop-filter: blur(10px);
    display: flex;
    gap: 12px;
    padding: 12px;
    align-items: center;
    width: 100%;
    min-width: 0;
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

@media screen and (max-width: 500px) {
    .playing-now-card {
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
