<template>
    <div class="card">
        <div
            v-if="rank"
            class="rank-badge"
            :style="
                step && rank > step
                    ? { fontSize: '0.7em', width: '30px' }
                    : { fontSize: '0.9em', width: '24px' }
            "
        >
            #{{ rank }}
        </div>
        <component
            :is="link ? 'router-link' : 'div'"
            :to="
                link ? { name: link.name, params: { id: item.id } } : undefined
            "
        >
            <img
                :src="imageUrl || 'will-trigger-error'"
                :alt="item.name"
                class="card-image"
                @error="onImageError"
            />
        </component>
        <div class="card-content">
            <div class="card-summary" style="display:">
                <h3>{{ item.name }}</h3>
                <p v-if="artist && !isArtist(item)">{{ artist }}</p>
                <p>
                    {{ playCount }} plays
                    {{
                        duration
                            ? `/ ${formatMillisecondsToMinutes(duration)}`
                            : ""
                    }}
                </p>
            </div>
            <div class="card-navidrome-link">
                <a
                    v-if="item.subsonic_url"
                    :href="item.subsonic_url"
                    target="_blank"
                >
                    <img
                        src="/img/navidrome-logo.png"
                        alt="Navidrome"
                        class="navidrome-logo"
                    />
                </a>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { formatMillisecondsToMinutes } from "@/utils/formatter";
import type { Artist, Track, Album, Item } from "@/types/music";
import { computed } from "vue";
import { loadImage } from "@/utils/thumbail";

interface Props {
    item: Artist | Track | Album;
    rank?: number;
    link?: {
        name: string;
    };
    step?: number;
}

const props = defineProps<Props>();

function isArtist(item: Item): item is Artist {
    return item.type === "Artist";
}

const imageUrl = computed(() => {
    return loadImage(props.item.thumbnail_url);
});

const artist = computed(() => {
    switch (props.item.type) {
        case "Track":
            return (props.item as Track).artist_display_name;
        case "Artist":
            return props.item.name;
        case "Album":
            return (props.item as Album).artist_name;
        default:
            return "";
    }
});

const playCount = computed(() => {
    return props.item.listens ?? 0;
});

const duration = computed(() => {
    if (props.item.type == "Track") {
        return (props.item as Track).length;
    }
    return null;
});

const onImageError = (event: Event) => {
    const target = event.target as HTMLImageElement;
    target.src = "/img/photo-off.svg";
    target.style.opacity = "0.5";
    target.style.transform = "scale(0.4)";
};
</script>

<style scoped>
.card {
    background: var(--card-background);
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    transition: transform 0.2s;
    border: 2px solid var(--border-color);
    position: relative;
    aspect-ratio: 1;
}

.card:hover {
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
}

.card-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
    cursor: pointer;
}

.card-content {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    padding: 8px;
    background: rgba(0, 0, 0, 0.7);
    color: white;
    opacity: 0;
    transform: translateY(100%);
    transition: all 0.2s ease;
    display: flex;
    justify-content: space-between;
}

.card:hover .card-content {
    opacity: 1;
    transform: translateY(0);
}

.card-content h3 {
    margin: 0 0 4px 0;
    font-size: 0.9em;
    color: white;
}

.card-content p {
    margin: 2px 0;
    color: rgba(255, 255, 255, 0.8);
    font-size: 0.8em;
}

.rank-badge {
    position: absolute;
    top: 8px;
    left: 8px;
    background: var(--primary-color);
    color: var(--background-color);
    width: 24px;
    height: auto;
    padding: 2px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 0.9em;
    z-index: 1;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.card-image-placeholder {
    width: 100%;
    height: 100%;
    background-color: var(--background-color);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-color);
}

.navidrome-logo {
    width: 32px;
    height: 32px;
    vertical-align: middle;
}

.card-summary {
    display: flex;
    flex-direction: column;
}

.card-navidrome-link {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    justify-content: center;
}

@media screen and (max-width: 500px) {
    .card-content {
        opacity: 1;
        transform: translateY(0);
        background: rgba(0, 0, 0, 0.3);
    }
    .card-content h3 {
        font-size: 0.75em;
        text-wrap: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
    .card-content p {
        font-size: 0.7em;
    }
}
</style>
