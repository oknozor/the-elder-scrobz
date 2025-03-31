<template>
	<div class="time-range-icons">
		<svg
			v-if="timeRange === 'today'"
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
			<path
				d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12z"
			></path>
			<path d="M16 3v4"></path>
			<path d="M8 3v4"></path>
			<path d="M4 11h16"></path>
			<path d="M11 15h1"></path>
			<path d="M12 15v3"></path>
		</svg>
		<svg
			v-if="timeRange === 'week'"
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
			<path
				d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12z"
			></path>
			<path d="M16 3v4"></path>
			<path d="M8 3v4"></path>
			<path d="M4 11h16"></path>
			<path d="M8 14v4"></path>
			<path d="M12 14v4"></path>
			<path d="M16 14v4"></path>
		</svg>
		<svg
			v-if="timeRange === 'month'"
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
			<path
				d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12z"
			></path>
			<path d="M16 3v4"></path>
			<path d="M8 3v4"></path>
			<path d="M4 11h16"></path>
			<path d="M7 14h.013"></path>
			<path d="M10.01 14h.005"></path>
			<path d="M13.01 14h.005"></path>
			<path d="M16.015 14h.005"></path>
			<path d="M13.015 17h.005"></path>
			<path d="M7.01 17h.005"></path>
			<path d="M10.01 17h.005"></path>
		</svg>
		<svg
			v-if="timeRange === 'year'"
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
			<path
				d="M11.795 21h-6.795a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4"
			></path>
			<path d="M18 18m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0"></path>
			<path d="M15 3v4"></path>
			<path d="M7 3v4"></path>
			<path d="M3 11h16"></path>
			<path d="M18 16.496v1.504l1 1"></path>
		</svg>
		<svg
			v-if="timeRange === 'all'"
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
			<path
				d="M9.828 9.172a4 4 0 1 0 0 5.656a10 10 0 0 0 2.172 -2.828a10 10 0 0 1 2.172 -2.828a4 4 0 1 1 0 5.656a10 10 0 0 1 -2.172 -2.828a10 10 0 0 0 -2.172 -2.828"
			></path>
		</svg>
	</div>
</template>
<script setup lang="ts">
defineProps({
	timeRange: {
		type: String,
		required: true,
		validator: (value: string) =>
			['today', 'week', 'month', 'year', 'all'].includes(value),
	},
});
</script>
