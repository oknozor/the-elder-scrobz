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
			:to="link ? { name: link.name, params: { id: id } } : undefined"
		>
			<img
				:src="imageUrl || 'will-trigger-error'"
				:alt="title"
				class="card-image"
				@error="onImageError"
			/>
		</component>
		<div class="card-content">
			<h3>{{ title }}</h3>
			<p v-if="artist && !isArtist(item)">{{ artist }}</p>
			<p>
				{{ playCount }} plays
				{{
					duration ? `/ ${formatMillisecondsToMinutes(duration)}` : ''
				}}
			</p>
		</div>
	</div>
</template>

<script setup lang="ts">
import { formatMillisecondsToMinutes } from '@/utils/formatter';
import type { Artist, Track, Album } from '@/types/music';
import { computed } from 'vue';

interface Props {
	item: Artist | Track | Album;
	rank?: number;
	link?: {
		name: string;
	};
	step?: number;
}

const props = defineProps<Props>();

function isTrack(item: Artist | Track | Album): item is Track {
	return (item as Track).track_id !== undefined;
}

function isArtist(item: Artist | Track | Album): item is Artist {
	return (item as Artist).artist_id !== undefined;
}

function isAlbum(item: Artist | Track | Album): item is Album {
	return (item as Album).release_id !== undefined;
}

const id = computed(() => {
	if (isTrack(props.item)) {
		return props.item.track_id;
	} else if (isArtist(props.item)) {
		return props.item.artist_id;
	} else if (isAlbum(props.item)) {
		return props.item.release_id;
	}
});

const title = computed(() => {
	if (isTrack(props.item)) {
		return props.item.track_name;
	} else if (isArtist(props.item)) {
		return props.item.artist_name;
	} else if (isAlbum(props.item)) {
		return props.item.release_name;
	}
	return 'Unknown';
});

const imageUrl = computed(() => {
	if (isTrack(props.item)) {
		return import.meta.env.VITE_API_BASE_URL + props.item.cover_art_url;
	} else if (isArtist(props.item)) {
		return props.item.thumbnail_url;
	} else if (isAlbum(props.item)) {
		return import.meta.env.VITE_API_BASE_URL + props.item.cover_art_url;
	}
	return '';
});

const artist = computed(() => {
	if (isTrack(props.item)) {
		return props.item.artist_name;
	} else if (isArtist(props.item)) {
		return props.item.artist_name;
	} else if (isAlbum(props.item)) {
		return props.item.artist_name;
	}
	return '';
});

const playCount = computed(() => {
	if (isTrack(props.item)) {
		return props.item.listens;
	} else if (isArtist(props.item)) {
		return props.item.listens;
	} else if (isAlbum(props.item)) {
		return props.item.listens;
	}
	return 0;
});

const duration = computed(() => {
	if (isTrack(props.item)) {
		return props.item.track_length;
	}
	return null;
});

const onImageError = (event: Event) => {
	const target = event.target as HTMLImageElement;
	target.src = '/img/photo-off.svg';
	target.style.opacity = '0.5';
	target.style.transform = 'scale(0.4)';
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
