<template>
    <AdminOnly>
        <div class="admin-view">
            <div class="admin-header">
                <h1>Administration Dashboard</h1>
                <p class="admin-description">
                    Manage database operations, view statistics, and monitor
                    system health.
                </p>
                <div class="admin-user-info">
                    <p class="text-sm text-gray-600">
                        Logged in as:
                        <span class="font-semibold">{{ userName }}</span>
                        <span
                            v-if="userRole"
                            class="ml-2 text-xs bg-gray-200 px-2 py-1 rounded"
                        >
                            Role: {{ userRole }}
                        </span>
                    </p>
                </div>
            </div>

            <div class="admin-content">
                <div class="admin-section">
                    <AdminConfig />
                </div>

                <div class="admin-section">
                    <StatsVue />
                </div>

                <div class="admin-section">
                    <ScanForm />
                </div>

                <div class="admin-section">
                    <ScrobbleJsonViewer />
                </div>

                <div class="admin-section">
                    <ErroredScrobblesViewer />
                </div>
            </div>
        </div>
    </AdminOnly>
</template>

<script setup lang="ts">
import { computed } from "vue";
import AdminOnly from "@/components/AdminOnly.vue";
import AdminConfig from "@/components/admin/AdminConfig.vue";
import ErroredScrobblesViewer from "@/components/admin/ErroredScrobblesViewer.vue";
import ScanForm from "@/components/admin/ScanForm.vue";
import ScrobbleJsonViewer from "@/components/admin/ScrobbleJsonViewer.vue";
import StatsVue from "@/components/admin/stats/StatsVue.vue";
import { useAuthStore } from "@/stores/authStore";

const authStore = useAuthStore();

const userName = computed(() => authStore.userName);
const userRole = computed(() => authStore.isAdmin);
</script>

<style scoped>
.admin-view {
    padding: 20px;
    max-width: 1200px;
    margin: 0 auto;
}

.admin-header {
    text-align: center;
    margin-bottom: 40px;
}

.admin-header h1 {
    color: var(--text-color);
    margin: 0;
    font-size: 2.2em;
    font-weight: 600;
}

.admin-user-info {
    margin-top: 16px;
    text-align: center;
}

.admin-description {
    color: var(--text-secondary);
    font-size: 1.1em;
    margin: 0;
    max-width: 600px;
    margin-left: auto;
    margin-right: auto;
    line-height: 1.5;
}

.admin-content {
    display: flex;
    flex-direction: column;
    gap: 40px;
}

.admin-section {
    background: var(--card-background);
    border-radius: 12px;
    padding: 32px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
}

.section-header {
    margin-bottom: 24px;
    text-align: center;
}

.section-header h2 {
    color: var(--text-color);
    margin: 0 0 8px 0;
    font-size: 1.6em;
    font-weight: 600;
}

.section-description {
    color: var(--text-secondary);
    font-size: 1em;
    margin: 0;
    line-height: 1.5;
}

@media (max-width: 768px) {
    .admin-view {
        padding: 16px;
    }

    .admin-header h1 {
        font-size: 1.8em;
    }

    .admin-section {
        padding: 20px;
    }

    .section-header h2 {
        font-size: 1.4em;
    }
}
</style>
