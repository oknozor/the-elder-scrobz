<template>
    <div v-if="isAdmin">
        <slot />
    </div>
    <div v-else-if="!isLoading" class="admin-only-error">
        <div class="max-w-md mx-auto bg-white rounded-lg shadow-md p-6">
            <div
                class="flex items-center justify-center w-12 h-12 mx-auto bg-red-100 rounded-full"
            >
                <svg
                    class="w-6 h-6 text-red-600"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.082 16.5c-.77.833.192 2.5 1.732 2.5z"
                    />
                </svg>
            </div>
            <div class="mt-4 text-center">
                <h3 class="text-lg font-medium text-gray-900">Access Denied</h3>
                <p class="mt-2 text-sm text-gray-500">
                    {{
                        errorMessage ||
                        "You need administrator privileges to access this content."
                    }}
                </p>
                <div class="mt-4">
                    <button
                        @click="goBack"
                        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    >
                        Go Back
                    </button>
                </div>
            </div>
        </div>
    </div>
    <div v-else class="admin-loading">
        <div class="flex items-center justify-center p-4">
            <svg
                class="animate-spin -ml-1 mr-3 h-5 w-5 text-gray-500"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
            >
                <circle
                    class="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                ></circle>
                <path
                    class="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                ></path>
            </svg>
            <span class="text-gray-500">Checking permissions...</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "@/stores/authStore";

interface Props {
    errorMessage?: string;
    redirectTo?: string;
    showError?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
    errorMessage: "",
    redirectTo: "/",
    showError: true,
});

const emit = defineEmits<{
    unauthorized: [];
    authorized: [];
}>();

const authStore = useAuthStore();
const router = useRouter();
const isLoading = ref(true);

const isAdmin = computed(() => {
    return authStore.isAuthenticated && authStore.isAdmin;
});

const goBack = () => {
    if (props.redirectTo) {
        router.push(props.redirectTo);
    } else {
        router.back();
    }
};

onMounted(async () => {
    // Wait for auth store to initialize if it's still loading
    if (authStore.isLoading) {
        await authStore.initialize();
    }

    isLoading.value = false;

    // Emit events based on authorization status
    if (isAdmin.value) {
        emit("authorized");
    } else {
        emit("unauthorized");
    }
});
</script>

<style scoped>
.admin-only-error {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--background-color);
    padding: 48px 16px;
}

.admin-loading {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--background-color);
}
</style>
