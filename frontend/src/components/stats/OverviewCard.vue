<template>
	<div class="overview-card">
		<h3 class="card-title">{{ title }}</h3>
		<div class="card-value">{{ value }}</div>
		<div class="percentage-change-container">
			<div
				v-if="percentageChange !== null && percentageChange !== 0"
				class="percentage-change"
				:class="{
					increase: percentageChange > 0,
					decrease: percentageChange < 0,
				}"
			>
				{{ percentageChange > 0 ? '+' : '' }}{{ percentageChange.toFixed(2) }}%
				{{ comparisonText }}
			</div>
			<div v-else class="percentage-change-placeholder"></div>
		</div>
	</div>
</template>

<script setup lang="ts">
defineProps({
	title: {
		type: String,
		required: true,
	},
	value: {
		type: [String, Number],
		required: true,
	},
	percentageChange: {
		type: Number,
		default: null,
	},
	comparisonText: {
		type: String,
		default: '',
	},
});
</script>

<style scoped>
.overview-card {
	background: var(--card-background);
	border-radius: 8px;
	padding: 16px;
	box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	border: 1px solid var(--border-color);
	display: flex;
	flex-direction: column;
	flex: 1;
}

.card-title {
	color: var(--text-secondary);
	font-size: 1em;
	margin: 0 0 8px 0;
	font-weight: 500;
}

.card-value {
	color: var(--text-color);
	font-size: 1.8em;
	font-weight: bold;
	margin-bottom: 8px;
}

.percentage-change-container {
	height: 20px;
}

.percentage-change {
	font-size: 0.9em;
	font-weight: 500;
}

.percentage-change-placeholder {
	height: 20px;
}

.increase {
	color: #4caf50;
}

.decrease {
	color: #f44336;
}

@media screen and (max-width: 500px) {
	.overview-card {
		padding: 8px;
	}
	.card-title {
		font-size: 0.8em;
		margin-bottom: 4px;
		text-align: center;
	}

	.card-value {
		font-size: 1.2em;
		margin-bottom: 4px;
		text-align: center;
		text-wrap: nowrap;
	}
	.percentage-change-container {
		height: auto;
	}
	.percentage-change {
		font-size: 0.8em;
		text-align: center;
	}
}
</style>
