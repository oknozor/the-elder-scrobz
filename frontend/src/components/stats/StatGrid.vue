<template>
    <div class="grid-wrapper">
        <div
            class="grid-container top-items"
            ref="gridContainer"
            @scroll="checkScroll"
        >
            <Card
                v-if="items.length > 0"
                v-for="(item, index) in items.slice(0, step)"
                :key="generateKey(item, index)"
                :item="item"
                :rank="index + 1"
                :link="link"
                :step="step"
            />
            <div
                v-if="items.length > step && windowWidth < 500"
                class="blur-overlay blur-right"
                :class="{ active: canScrollRight }"
            ></div>
            <div
                v-if="items.length > step && windowWidth < 500"
                class="blur-overlay blur-left"
                :class="{ active: canScrollLeft }"
            ></div>
        </div>
        <div
            v-if="items.length > step && windowWidth > 500"
            class="grid-container bottom-items"
            :style="{ height: bottomHeight }"
            ref="bottomItems"
        >
            <Card
                v-for="(item, index) in items.slice(step, limit)"
                :key="generateKey(item, index)"
                :item="item"
                :rank="index + step + 1"
                :link="link"
                :step="step"
                @mouseenter="handleMouseEnter"
                @mouseleave="handleMouseLeave"
                class="small-card"
            />
        </div>
    </div>
</template>

<script setup lang="ts">
import { nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import { useWindowWidth } from "@/composables/useWindowWidth";
import type { Album, Artist, Track } from "@/types/music";
import Card from "../Card.vue";

interface Props {
    items: Artist[] | Track[] | Album[];
    limit?: number;
    step?: number;
    link?: {
        name: string;
    };
}

const props = withDefaults(defineProps<Props>(), {
    step: 5,
});

const gridContainer = ref<HTMLDivElement | null>(null);
const bottomItems = ref<HTMLDivElement | null>(null);
const bottomHeight = ref("7rem");
const canScrollRight = ref(false);
const canScrollLeft = ref(false);
const windowWidth = useWindowWidth();

const checkScroll = () => {
    if (!gridContainer.value) return;

    const { scrollLeft, scrollWidth, clientWidth } = gridContainer.value;
    canScrollRight.value = scrollLeft < scrollWidth - clientWidth - 1; // -1 for rounding errors
    canScrollLeft.value = scrollLeft > 0;
};

const handleMouseEnter = () => {
    if (windowWidth.value < 650) {
        return;
    } else {
        bottomHeight.value = "14rem";
    }
};

const handleMouseLeave = () => {
    if (windowWidth.value < 650) {
        return;
    } else {
        bottomHeight.value = "7rem";
    }
};

watch(
    () => props.items.length,
    async () => {
        await nextTick();
        checkScroll();
    },
);

onMounted(() => {
    checkScroll();
    window.addEventListener("resize", checkScroll);
});

onUnmounted(() => {
    window.removeEventListener("resize", checkScroll);
});

const generateKey = (item: Artist | Track | Album, index: number) => {
    if (item.id) {
        return item.id;
    }
    return index;
};
</script>

<style scoped>
.grid-wrapper {
    position: relative;
}
.grid-container {
    display: flex;
    padding: 0;
    --gap: 12px;
    gap: var(--gap);
    align-items: flex-start;
    flex-wrap: nowrap;
    margin-bottom: var(--gap);
}

.grid-container.top-items {
    justify-content: flex-start;
}

.grid-container.top-items .card {
    flex: 1;
}

.grid-container.bottom-items {
    justify-content: flex-start;
    gap: var(--gap);
    width: 100%;
    overflow-x: auto;
    scrollbar-width: none;
    -ms-overflow-style: none;
}

.bottom-items {
    transition: all 0.3s ease;
}

.grid-container .card {
    transition: all 0.3s ease;
}

.top-items .card:hover {
    transform: translateY(-4px);
}

.card {
    max-height: 14rem;
    max-width: calc(20% - var(--gap));
}

.small-card {
    height: auto;
    flex: 0 0 calc(10% - var(--gap));
    max-width: calc(10% - var(--gap));
    min-width: calc(10% - var(--gap));
}

/* Override hover behavior for small cards to maintain consistent sizing */
.small-card:hover {
    flex: 0 0 calc(20% - var(--gap));
    max-width: calc(20% - var(--gap));
    min-width: calc(20% - var(--gap));
}

@media screen and (max-width: 650px) {
    .small-card {
        height: auto;
        flex: 1;
        max-width: calc(20% - var(--gap));
        min-width: calc(20% - var(--gap));
    }
    .small-card:hover {
        flex: 1;
        max-width: calc(20% - var(--gap));
        min-width: calc(20% - var(--gap));
    }
}

@media screen and (max-width: 500px) {
    .grid-container.top-items {
        overflow-x: auto;
        scrollbar-width: none;
        -ms-overflow-style: none;
        --gap: 10px;
        gap: var(--gap);
    }
    .card {
        height: auto;
        flex: 0 0 calc(37.5% - var(--gap));
        max-width: calc(37.5% - var(--gap));
        min-width: calc(37.5% - var(--gap));
    }
    .small-card:hover {
        flex: 1;
        max-width: calc(20% - var(--gap));
        min-width: calc(20% - var(--gap));
    }
    .blur-overlay {
        position: absolute;
        top: 0;
        bottom: 0;
        width: 100px;
        pointer-events: none;
        opacity: 0;
        transition: opacity 0.3s ease;
    }

    .blur-overlay.active {
        opacity: 1;
    }

    .blur-right {
        right: 0;
        background: linear-gradient(
            to right,
            transparent,
            var(--background-color, #ffffff) 90%
        );
    }

    .blur-left {
        left: 0;
        background: linear-gradient(
            to left,
            transparent,
            var(--background-color, #ffffff) 90%
        );
    }
}
</style>
