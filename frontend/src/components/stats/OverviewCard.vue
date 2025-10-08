<template>
    <FlexVertical>
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
                {{ percentageChange > 0 ? "+" : ""
                }}{{ percentageChange.toFixed(2) }}%
                {{ comparisonText }}
            </div>
            <div v-else class="percentage-change-placeholder"></div>
        </div>
    </FlexVertical>
</template>

<script setup lang="ts">
import FlexVertical from "@/base/FlexVertical.vue";

defineProps({
    title: {
        type: String,
        required: true,
    },
    value: {
        type: [String, Number],
        required: true,
        default: 0,
    },
    percentageChange: {
        type: Number,
        default: 0,
    },
    comparisonText: {
        type: String,
        default: "",
    },
});
</script>

<style scoped>
.card-title {
    color: var(--text-secondary);
    font-size: 1em;
    font-weight: 500;
}

.card-value {
    color: var(--text-color);
    font-size: 1.8em;
    font-weight: bold;
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
        text-align: center;
    }

    .card-value {
        font-size: 1.2em;
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
