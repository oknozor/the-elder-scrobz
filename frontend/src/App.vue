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
        <RouterView v-slot="{ Component }">
            <Transition name="slide" mode="out-in">
                <component :is="Component" />
            </Transition>
        </RouterView>
    </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useRoute } from "vue-router";
import PageHeader from "@/components/PageHeader.vue";
import UsernameSelector from "@/components/UsernameSelector.vue";
import { useAuthStore } from "./stores/authStore";
import { type AppUser, useUsersStore } from "./stores/usersStore";

const route = useRoute();
const usersStore = useUsersStore();
const authStore = useAuthStore();

const showBackButton = computed(() => {
    return route.name !== "stats";
});

const showUserSelector = computed(() => {
    return ["stats", "artist", "album"].includes(route.name as string);
});

const users = computed(() => usersStore.users);
const currentUser = computed(() => usersStore.selectedUser);
const selectedUser = ref<AppUser | null>(null);

watch(
    () => usersStore.selectedUser,
    (newUser) => {
        selectedUser.value = newUser;
    },
    { immediate: true },
);

watch(
    () => authStore.isAuthenticated,
    async (isAuthenticated) => {
        if (isAuthenticated) {
            try {
                await usersStore.fetchUsers();
            } catch (error) {
                console.error("Error fetching users:", error);
            }
        }
    },
    { immediate: true },
);

const handleLogout = () => {
    // Implement logout logic here
    usersStore.selectedUser = null;
};

const handleUserChange = (user: AppUser | null) => {
    selectedUser.value = user;
    usersStore.updateSelectedUser(user);
};
</script>

<style>
.app {
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--app-padding);
    display: flex;
    flex-direction: column;
    gap: var(--app-padding);
}
.slide-enter-active,
.slide-leave-active {
    transition:
        transform 0.5s ease-in-out,
        opacity 0.5s;
}
.slide-enter-from {
    transform: translateX(100%);
    opacity: 0;
}
.slide-leave-to {
    transform: translateX(-100%);
    opacity: 0;
}
</style>
