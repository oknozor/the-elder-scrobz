<template>
	<Transition name="toast">
		<div v-if="modelValue" class="toast" :class="type">
			<div class="toast-content">
				<slot></slot>
			</div>
			<button class="close-button" @click="close">
				<svg
					class="icon"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M18 6L6 18M6 6l12 12" />
				</svg>
			</button>
		</div>
	</Transition>
</template>

<script setup lang="ts">
import { watch } from "vue";

interface Props {
    modelValue: boolean;
    type?: "success" | "error" | "info";
    duration?: number;
}

const props = withDefaults(defineProps<Props>(), {
    type: "info",
    duration: 3000,
});

const emit = defineEmits<(e: "update:modelValue", value: boolean) => void>();

const close = () => {
    emit("update:modelValue", false);
};

// Auto-close after duration
watch(
    () => props.modelValue,
    (newValue: boolean) => {
        if (newValue) {
            setTimeout(() => {
                close();
            }, props.duration);
        }
    },
);
</script>

<style scoped>
.toast {
	position: fixed;
	bottom: 20px;
	right: 20px;
	padding: 12px 24px;
	border-radius: 6px;
	background: var(--card-background);
	color: var(--text-color);
	box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	display: flex;
	align-items: center;
	gap: 12px;
	z-index: 1000;
	min-width: 200px;
	max-width: 400px;
}

.toast.success {
	border-left: 4px solid #2ecc71;
}

.toast.error {
	border-left: 4px solid #e74c3c;
}

.toast.info {
	border-left: 4px solid #3498db;
}

.toast-content {
	flex: 1;
}

.close-button {
	background: none;
	border: none;
	padding: 4px;
	cursor: pointer;
	color: var(--text-secondary);
	transition: color 0.2s;
}

.close-button:hover {
	color: var(--text-color);
}

.icon {
	width: 16px;
	height: 16px;
}

/* Toast transitions */
.toast-enter-active,
.toast-leave-active {
	transition: all 0.3s ease;
}

.toast-enter-from {
	opacity: 0;
	transform: translateY(20px);
}

.toast-leave-to {
	opacity: 0;
	transform: translateY(20px);
}
</style>
