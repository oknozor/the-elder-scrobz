<template>
	<div class="app">
		<PageHeader
			:current-user="currentUser"
			:show-back-button="showBackButton"
			@logout="handleLogout"
		>
			<template #left>
				<UsernameSelector
					v-if="showUserSelector"
					v-model="selectedUser"
					:users="users"
					@update:modelValue="handleUserChange"
				/>
			</template>
			<template #right>
				<slot name="header-right"></slot>
			</template>
		</PageHeader>
		<router-view></router-view>
	</div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import PageHeader from '@/components/PageHeader.vue';
import UsernameSelector from '@/components/UsernameSelector.vue';
import { AppUser, useUsersStore } from './stores/usersStore';

const route = useRoute();
const usersStore = useUsersStore();

// Determine if back button should be shown based on the current route
const showBackButton = computed(() => {
	return route.name !== 'stats';
});

// Determine if user selector should be shown based on the current route
const showUserSelector = computed(() => {
	return ['stats', 'artist', 'album'].includes(route.name as string);
});

const users = ref<AppUser[]>([]);
const selectedUser = ref<AppUser | null>(null);
const currentUser = ref<AppUser | null>(null);

const handleLogout = () => {
	// Implement logout logic here
	usersStore.selectedUser = null;
	console.log('Logout clicked');
};

const handleUserChange = (user: AppUser | null) => {
	selectedUser.value = user;
	usersStore.selectedUser = user;
};

onMounted(async () => {
	try {
		await usersStore.fetchUsers();
		users.value = usersStore.users;
		currentUser.value = usersStore.selectedUser;
	} catch (error) {
		console.error('Error fetching users:', error);
	}
});
</script>

<style>
.app {
	max-width: 1200px;
	margin: 0 auto;
	padding: 20px;
}
</style>
