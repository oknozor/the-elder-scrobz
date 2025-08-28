<template>
    <div class="pulse-chart">
        <Chart type="bar" :data="chartData" :options="chartOptions" />
    </div>
</template>

<script setup lang="ts">
import {
    BarController,
    BarElement,
    CategoryScale,
    Chart as ChartJS,
    Filler,
    Legend,
    LinearScale,
    LineController,
    LineElement,
    PointElement,
    Title,
    Tooltip,
    TooltipItem,
} from "chart.js";
import { computed } from "vue";
import { Chart } from "vue-chartjs";

ChartJS.register(
    CategoryScale,
    LinearScale,
    BarElement,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
    BarController,
    LineController,
    Filler,
);

interface Props {
    pulseData: Array<{
        period: string;
        listens: number;
    }>;
}

const props = defineProps<Props>();

const chartData = computed(() => {
    const primaryColor = getComputedStyle(document.documentElement)
        .getPropertyValue("--primary-color")
        .trim();
    return {
        labels: props.pulseData.map((d) => d.period),
        datasets: [
            {
                type: "bar" as const,
                label: "Plays",
                data: props.pulseData.map((d) => d.listens),
                backgroundColor: `${primaryColor}40`,
                borderColor: primaryColor,
                borderRadius: 4,
                barPercentage: 0.5,
            },
            {
                type: "line" as const,
                label: "Trend",
                data: props.pulseData.map((d) => d.listens),
                borderColor: primaryColor,
                backgroundColor: `${primaryColor}20`,
                fill: true,
                tension: 0.4,
                pointRadius: 4,
                pointBackgroundColor: primaryColor,
                pointBorderColor: "white",
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
                label: (context: TooltipItem<"bar">) => `${context.raw} plays`,
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
                color: "white",
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
                color: "white",
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
