<template>
    <div class="user-button" ref="buttonRef">
        <div class="user-info" @click="isOpen = !isOpen">
            <div class="user-icon-container">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="icon"
                >
                    <path
                        d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
                    ></path>
                    <circle cx="12" cy="12" r="3"></circle>
                </svg>
            </div>
        </div>
        <transition name="user-dropdown">
            <div v-if="isOpen" class="user-dropdown">
                <div class="user-option" @click="handleImport">
                    <span class="option-text">Import</span>
                </div>
                <div class="user-option" @click="handleExport">
                    <span class="option-text">Export</span>
                </div>
                <div class="user-option" @click="handleUsers">
                    <span class="option-text">Users</span>
                </div>
                <div class="user-option" @click="handleApiKeys">
                    <span class="option-text">API Keys</span>
                </div>
                <div v-if="isAdmin" class="user-option" @click="handleAdmin">
                    <span class="option-text"> Administration </span>
                </div>
                <div class="user-option" @click="handleLogout">
                    <span class="option-text">Logout</span>
                </div>
            </div>
        </transition>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "@/stores/authStore";

const router = useRouter();
const authStore = useAuthStore();
const emit = defineEmits<(e: "logout") => void>();

const isAdmin = computed(() => authStore.isAdmin);

const isOpen = ref(false);
const buttonRef = ref<HTMLElement | null>(null);

const handleClickOutside = (event: MouseEvent) => {
    if (buttonRef.value && !buttonRef.value.contains(event.target as Node)) {
        isOpen.value = false;
    }
};

onMounted(() => {
    document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
    document.removeEventListener("click", handleClickOutside);
});

const handleLogout = () => {
    emit("logout");
    isOpen.value = false;
};

const handleImport = () => {
    router.push({ name: "import" });
    isOpen.value = false;
};

const handleExport = () => {
    router.push({ name: "export" });
    isOpen.value = false;
};

const handleUsers = () => {
    router.push({ name: "users" });
    isOpen.value = false;
};

const handleApiKeys = () => {
    router.push({ name: "apiKeys" });
    isOpen.value = false;
};

const handleAdmin = () => {
    router.push({ name: "admin" });
    isOpen.value = false;
};
</script>

<style scoped>
.user-button {
    position: relative;
}

.user-info {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    background: var(--card-background);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
}

.user-icon-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
}

.user-info:hover {
    background: rgba(255, 255, 255, 0.05);
    transform: translateX(2px);
}

.icon {
    width: 20px;
    height: 20px;
    color: var(--primary-color);
}

.user-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 8px;
    background: var(--card-background);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    z-index: 1000;
    min-width: 150px;
}

.user-option {
    padding: 12px 16px;
    cursor: pointer;
    transition: background-color 0.2s;
}

.user-option:hover {
    background: rgba(255, 255, 255, 0.05);
}

.option-text {
    color: var(--text-color);
    font-size: 0.9em;
}

.user-dropdown-enter-active,
.user-dropdown-leave-active {
    transition: opacity 0.2s ease-out;
    overflow: hidden;
}

.user-dropdown-enter-from,
.user-dropdown-leave-to {
    opacity: 0;
}

.user-dropdown-enter-to,
.user-dropdown-leave-from {
    opacity: 1;
}
</style>
