<template>
	<Dropdown>
		<template #trigger>
			<span class="user-name">{{
				modelValue?.username || 'All Users'
			}}</span>
		</template>
		<template #content>
			<div class="user-dropdown">
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
		</template>
	</Dropdown>
</template>

<script setup lang="ts">
import Dropdown from './Dropdown.vue';
import { AppUser } from '@/stores/usersStore';

defineProps<{
	modelValue: AppUser | null;
	users: AppUser[];
}>();

const emit = defineEmits<{
	(e: 'update:modelValue', value: AppUser | null): void;
}>();

const selectUser = (user: AppUser) => {
	emit('update:modelValue', user);
};

const selectAll = () => {
	emit('update:modelValue', null);
};
</script>

<style scoped>
.user-dropdown {
	background: var(--card-background);
	border: 1px solid var(--border-color);
	border-radius: 6px;
	box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
	z-index: 100;
	max-height: 300px;
	overflow-y: auto;
	transform-origin: top;
	scrollbar-width: none;
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
