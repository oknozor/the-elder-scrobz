<template>
    <div class="dropdown" ref="dropdownRef">
        <div class="dropdown-trigger" @click="toggleDropdown">
            <slot name="trigger"></slot>
            <span class="dropdown-icon">
                <svg
                    :style="{
                        transform: isOpen ? 'rotate(90deg)' : 'rotate(0deg)',
                    }"
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
                    <path d="M9 6l6 6-6 6"></path>
                </svg>
            </span>
        </div>

        <transition name="dropdown" @enter="onEnter" @leave="onLeave">
            <div v-if="isOpen" class="dropdown-content">
                <slot name="content"></slot>
            </div>
        </transition>
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

const isOpen = ref(false);
const dropdownRef = ref<HTMLElement | null>(null);

const toggleDropdown = () => {
    isOpen.value = !isOpen.value;
};

const handleClickOutside = (event: MouseEvent) => {
    if (
        dropdownRef.value &&
        !dropdownRef.value.contains(event.target as Node)
    ) {
        isOpen.value = false;
    }
};

onMounted(() => {
    document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
    document.removeEventListener("click", handleClickOutside);
});

const onEnter = (el: Element) => {
    const element = el as HTMLElement;
    const content = element.children[0] as HTMLElement;
    element.style.height = `${content.offsetHeight}px`;
    const height = content.offsetHeight;
    element.style.height = "0";
    requestAnimationFrame(() => {
        element.style.height = `${height}px`;
    });
};

const onLeave = (el: Element) => {
    (el as HTMLElement).style.height = "0";
};
</script>

<style scoped>
.dropdown {
    position: relative;
    width: 200px;
}

.dropdown-trigger {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 8px 12px;
    background: var(--card-background);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
}

.dropdown-trigger:hover {
    background: rgba(255, 255, 255, 0.05);
}

.dropdown-icon {
    color: var(--text-secondary);
    font-size: 0.8em;
    transition: transform 0.2s;
    display: flex;
    justify-content: center;
    align-items: center;
}

.dropdown-enter-active,
.dropdown-leave-active {
    transition: all 0.2s ease-out;
    overflow: hidden;
}

.dropdown-enter-from,
.dropdown-leave-to {
    opacity: 0;
    transform: translateY(-10px);
    height: 0;
}

.dropdown-enter-to,
.dropdown-leave-from {
    opacity: 1;
    transform: translateY(0);
}

.dropdown-content {
    position: absolute;
    z-index: 50;
    top: 100%;
    left: 0;
    right: 0;
    margin-top: 4px;
    background: var(--card-background);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    z-index: 1000;
    max-height: 300px;
    overflow-y: auto;
    transform-origin: top;
    scrollbar-width: none;
}
</style>
