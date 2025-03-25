<template>
	<div class="card">
		<div v-if="rank" class="rank-badge">#{{ rank }}</div>
		<component
			:is="link ? 'router-link' : 'div'"
			:to="
				link ? { name: link.name, params: { id: item.id } } : undefined
			"
		>
			<img
				:src="item.imageUrl"
				:alt="item.title || item.name"
				class="card-image"
			/>
		</component>
		<div class="card-content">
			<h3>{{ item.title || item.name }}</h3>
			<p v-if="item.artist">{{ item.artist }}</p>
			<p>
				{{ item.playCount }} plays / {{ formatDuration(item.duration) }}
			</p>
		</div>
	</div>
</template>

<script setup lang="ts">
import { formatDuration } from '@/utils/formatter';

interface Props {
	item: {
		id: string;
		name?: string;
		title?: string;
		artist?: string;
		imageUrl: string;
		playCount: number;
		duration: number;
	};
	rank?: number;
	link?: {
		name: string;
	};
	expandable?: boolean;
}

defineProps<Props>();
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
	height: 24px;
	border-radius: 4px;
	display: flex;
	align-items: center;
	justify-content: center;
	font-weight: bold;
	font-size: 0.9em;
	z-index: 1;
	box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}
</style>
