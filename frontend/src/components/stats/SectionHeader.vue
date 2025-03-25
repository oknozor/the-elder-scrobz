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
		</h2>
		<slot name="controls"></slot>
	</div>
</template>

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
</style>
