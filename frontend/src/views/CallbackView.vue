<template>
    <div class="callback">
        <div v-if="error" class="error">
            <h1>Authentication Error</h1>
            <p>{{ error }}</p>
        </div>
        <div v-else class="loading">
            <h1>Authenticating...</h1>
            <p>Please wait while we complete the authentication process.</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "@/stores";

const router = useRouter();
const authStore = useAuthStore();
const error = ref<string | null>(null);

onMounted(async () => {
    try {
        await authStore.handleLoginCallback();
        await router.push("/");
    } catch (err) {
        console.error("Authentication callback error:", err);
        if (err instanceof Error) {
            error.value = `Authentication error: ${err.message}`;
        } else {
            error.value =
                "An error occurred during authentication. Please try again.";
        }
    }
});
</script>

<style scoped>
.callback {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    text-align: center;
    padding: 0 20px;
}

.loading,
.error {
    max-width: 500px;
}

.error {
    color: #e74c3c;
}
</style>
