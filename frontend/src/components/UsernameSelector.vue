<template>
	<div class="user-selector" ref="selectorRef">
		<div class="selected-user" @click="isOpen = !isOpen">
			<span class="user-name">{{
				modelValue?.username || 'All Users'
			}}</span>
			<span class="dropdown-icon">
				<svg
					:style="{
						transform: isOpen ? 'rotate(90deg)' : 'rotate(0deg)',
					}"
					class="dropdown-icon"
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
					<path d="M9 6l6 6l-6 6"></path>
				</svg>
			</span>
		</div>

		<transition name="dropdown" @enter="onEnter" @leave="onLeave">
			<div v-if="isOpen" class="user-dropdown">
				<div
					class="user-option"
					:class="{ selected: !modelValue }"
					@click="selectAll"
				>
					<span class="user-name">All Users</span>
				</div>
				<div
					v-for="user in users"
					:key="user.username"
					class="user-option"
					:class="{
						selected: modelValue?.username === user.username,
					}"
					@click="selectUser(user)"
				>
					<span class="user-name">{{ user.username }}</span>
				</div>
			</div>
		</transition>
	</div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { AppUser } from '@/stores/usersStore';

defineProps<{
	modelValue: AppUser | null;
	users: AppUser[];
}>();

const emit = defineEmits<{
	(e: 'update:modelValue', value: AppUser | null): void;
}>();

const isOpen = ref(false);
const selectorRef = ref<HTMLElement | null>(null);

const handleClickOutside = (event: MouseEvent) => {
	if (
		selectorRef.value &&
		!selectorRef.value.contains(event.target as Node)
	) {
		isOpen.value = false;
	}
};

onMounted(() => {
	document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
	document.removeEventListener('click', handleClickOutside);
});

const selectUser = (user: AppUser) => {
	emit('update:modelValue', user);
	isOpen.value = false;
};

const selectAll = () => {
	emit('update:modelValue', null);
	isOpen.value = false;
};

const onEnter = (el: Element) => {
	(el as HTMLElement).style.height = 'auto';
	const height = (el as HTMLElement).offsetHeight;
	(el as HTMLElement).style.height = '0';
	requestAnimationFrame(() => {
		(el as HTMLElement).style.height = `${height}px`;
	});
};

const onLeave = (el: Element) => {
	(el as HTMLElement).style.height = '0';
};
</script>

<style scoped>
.user-selector {
	position: relative;
	width: 200px;
}

.selected-user {
	display: flex;
	align-items: center;
	gap: 8px;
	padding: 8px 12px;
	background: var(--card-background);
	border: 1px solid var(--border-color);
	border-radius: 6px;
	cursor: pointer;
	transition: all 0.2s;
}

.selected-user:hover {
	background: rgba(255, 255, 255, 0.05);
}

.user-name {
	flex: 1;
	color: var(--text-color);
	font-size: 0.9em;
}

.dropdown-icon {
	color: var(--text-secondary);
	font-size: 0.8em;
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

.user-dropdown {
	position: absolute;
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

.dropdown-icon {
	padding-top: 2px;
	transition: transform 0.2s;
}

.user-option {
	padding: 8px 12px;
	cursor: pointer;
	transition: all 0.2s;
}

.user-option:hover {
	background: rgba(255, 255, 255, 0.05);
}

.user-option.selected {
	background: rgba(255, 255, 255, 0.1);
}

.user-option:first-child {
	border-bottom: 1px solid var(--border-color);
	margin-bottom: 4px;
	padding-bottom: 12px;
}
</style>
