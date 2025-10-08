<template>
	<div v-if="modelValue" class="modal-overlay" @click="handleOverlayClick">
		<div class="modal-content" @click.stop>
			<div class="modal-header">
				<h3>{{ title }}</h3>
				<Button variant="close"
					v-if="showCloseButton"
					@click="close"
				>
					<svg
						class="icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</Button>
			</div>
			<div class="modal-body">
				<slot></slot>
			</div>
			<div v-if="$slots.footer" class="modal-footer">
				<slot name="footer"></slot>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import Button from "@/base/Button.vue";

interface Props {
    modelValue: boolean;
    title: string;
    showCloseButton?: boolean;
    closeOnOverlayClick?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
    showCloseButton: true,
    closeOnOverlayClick: true,
});

const emit = defineEmits<(e: "update:modelValue", value: boolean) => void>();

const close = () => {
    emit("update:modelValue", false);
};

const handleOverlayClick = () => {
    if (props.closeOnOverlayClick) {
        close();
    }
};
</script>

<style scoped>
.modal-overlay {
	position: fixed;
	top: 0;
	left: 0;
	right: 0;
	bottom: 0;
	background: rgba(0, 0, 0, 0.5);
	display: flex;
	align-items: center;
	justify-content: center;
	z-index: 1000;
}

.modal-content {
	background: var(--card-background);
	border-radius: 8px;
	padding: 24px;
	width: 100%;
	max-width: 400px;
	box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.modal-header {
	display: flex;
	justify-content: space-between;
	align-items: center;
	margin-bottom: 20px;
}

.modal-header h3 {
	margin: 0;
	color: var(--text-color);
}

.icon {
	width: 20px;
	height: 20px;
}

.modal-body {
	margin-bottom: 20px;
}

.modal-footer {
	display: flex;
	justify-content: flex-end;
	gap: 12px;
}
</style>
