<template>
  <span
    v-if="isAdmin"
    class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300"
  >
    <svg
      class="w-3 h-3 mr-1"
      fill="currentColor"
      viewBox="0 0 20 20"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        fill-rule="evenodd"
        d="M3 6a3 3 0 013-3h10a1 1 0 01.8 1.6L14.25 8l2.55 3.4A1 1 0 0116 13H6a1 1 0 00-1 1v3a1 1 0 11-2 0V6z"
        clip-rule="evenodd"
      />
    </svg>
    Admin
  </span>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useAuthStore } from "@/stores/authStore";

const authStore = useAuthStore();

const isAdmin = computed(() => authStore.isAdmin);
</script>

<style scoped>
.admin-badge {
  @apply inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium;
  @apply bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300;
}
</style>
