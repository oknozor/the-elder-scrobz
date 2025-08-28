<template>
	<div class="section-header">
		<h2>
			<svg
				class="title-icon"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<slot name="icon"></slot>
			</svg>
			<slot></slot>
			<div
				class="link-icon"
				v-if="link && route.name !== link.name"
				@click="navigateToLink"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-linecap="round"
					stroke-linejoin="round"
					width="24"
					height="24"
					stroke-width="2"
				>
					<path d="M9 15l6 -6"></path>
					<path
						d="M11 6l.463 -.536a5 5 0 0 1 7.071 7.072l-.534 .464"
					></path>
					<path
						d="M13 18l-.397 .534a5.068 5.068 0 0 1 -7.127 0a4.972 4.972 0 0 1 0 -7.071l.524 -.463"
					></path>
				</svg>
			</div>
		</h2>
		<slot name="controls"></slot>
	</div>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from "vue-router";

const props = defineProps<{
    link?: {
        name: string;
    };
}>();

const route = useRoute();
const router = useRouter();

const navigateToLink = () => {
    if (props.link) {
        router.push({ name: props.link.name });
    }
};
</script>
<style scoped>
.section-header {
	display: flex;
	justify-content: space-between;
	align-items: center;
	margin-bottom: 20px;
}

h2 {
	color: var(--text-color);
	margin: 0;
	display: flex;
	align-items: center;
	gap: 8px;
}

.title-icon {
	width: 20px;
	height: 20px;
	color: var(--primary-color);
}

.link-icon {
	cursor: pointer;
	display: flex;
	align-items: center;
	justify-content: center;
}
</style>
