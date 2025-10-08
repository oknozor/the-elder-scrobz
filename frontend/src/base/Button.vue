<template>
<button
    :type="props.type"
    :disabled="props.disabled"
    :class="[
      'button',
      `button-${props.variant}`,
      { 'active': props.active },
    ]"
     @click="handleClick"
 >
     <slot/>
 </button>
</template>

<script setup lang="ts">
import { PropType } from "vue";

const props = defineProps({
    type: {
        type: String as PropType<"button" | "submit" | "reset">,
        default: "button",
    },
    disabled: {
        type: Boolean,
        default: false,
    },
    active: {
        type: Boolean,
        default: false,
    },
    variant: {
        type: String as PropType<"default" | "confirm" | "close">,
        default: "default",
    },
});

const emit = defineEmits<(e: "click", event: MouseEvent) => void>();

const handleClick = (e: MouseEvent) => {
    if (props.disabled) {
        e.preventDefault();
        return;
    }

    emit("click", e);
};
</script>

<style scoped lang="scss">
.button {
    padding: 8px 16px;
    font-weight: 500;
    display: flex;
    align-items: center;
    text-align: center;
    gap: 8px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 0.9em;
    cursor: pointer;
    transition: all 0.2s;

    &.active {
        background: var(--primary-color);
        color: var(--background-color);
        border-color: var(--primary-color);
    }

    &:disabled {
      opacity: 0.6;
      cursor: not-allowed;
    }

    &-default {
        background: var(--card-background);
        color: var(--text-color);

        &:not(.active):not(:disabled):hover {
            background: rgba(255, 255, 255, 0.05);
            transform: translateX(-1px);
        }
    }

    &-confirm {
        background: var(--primary-color);
        color: var(--background-color);

        &:not(.active):not(:disabled):hover {
            opacity: 0.8;
            transform: translateX(-1px);
        }
    }

    &-close {
        flex-shrink: 0;
        text-align: right;
        background: none;
    	border: none;
    	padding: 4px;
    	cursor: pointer;
    	color: var(--text-secondary);
    	transition: color 0.2s;
    	min-width: 23px;

        &:hover {
            color: var(--text-color);
        }
    }
}
</style>
