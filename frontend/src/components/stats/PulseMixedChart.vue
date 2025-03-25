<template>
	<div class="pulse-chart">
		<Chart type="bar" :data="chartData" :options="chartOptions" />
	</div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { Chart } from 'vue-chartjs';
import {
	Chart as ChartJS,
	CategoryScale,
	LinearScale,
	BarElement,
	PointElement,
	LineElement,
	Title,
	Tooltip,
	Legend,
} from 'chart.js';

ChartJS.register(
	CategoryScale,
	LinearScale,
	BarElement,
	PointElement,
	LineElement,
	Title,
	Tooltip,
	Legend
);

interface Props {
	pulseData: Array<{
		period: string;
		playCount: number;
	}>;
}

const props = defineProps<Props>();

const chartData = computed(() => {
	const primaryColor = getComputedStyle(document.documentElement)
		.getPropertyValue('--primary-color')
		.trim();
	return {
		labels: props.pulseData.map((d) => d.period),
		datasets: [
			{
				type: 'bar' as const,
				label: 'Plays',
				data: props.pulseData.map((d) => d.playCount),
				backgroundColor: primaryColor + '40',
				borderColor: primaryColor,
				borderRadius: 4,
				barPercentage: 0.5,
			},
			{
				type: 'line' as const,
				label: 'Trend',
				data: props.pulseData.map((d) => d.playCount),
				borderColor: primaryColor,
				backgroundColor: primaryColor + '20',
				fill: true,
				tension: 0.4,
				pointRadius: 4,
				pointBackgroundColor: primaryColor,
				pointBorderColor: 'white',
				pointBorderWidth: 2,
			},
		],
	};
});

const chartOptions = {
	responsive: true,
	maintainAspectRatio: false,
	plugins: {
		legend: {
			display: false,
		},
		tooltip: {
			callbacks: {
				label: (context: any) => `${context.raw} plays`,
			},
		},
	},
	scales: {
		y: {
			beginAtZero: true,
			grid: {
				display: false,
			},
			ticks: {
				color: 'white',
			},
			border: {
				display: false,
			},
		},
		x: {
			grid: {
				display: false,
			},
			ticks: {
				color: 'white',
			},
			border: {
				display: false,
			},
		},
	},
};
</script>

<style scoped>
.pulse-chart {
	width: 100%;
	background: var(--card-background);
	border-radius: 8px;
	padding: 20px;
	box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	border: 1px solid var(--border-color);
	height: 300px;
}
</style>
