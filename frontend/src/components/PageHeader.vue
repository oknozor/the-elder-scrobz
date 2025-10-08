<template>
    <div class="page-header">
        <div class="header-left">
            <Button
                class="back-button"
                v-if="showBackButton"
                @click="handleBack"
            >
                <span class="back-icon">←</span>
                Back
            </Button>
            <slot name="left"></slot>
        </div>
        <div class="header-right">
            <slot name="right">
                <UserButton
                    :current-user="currentUser"
                    @logout="$emit('logout')"
                />
            </slot>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import Button from "@/base/Button.vue";
import UserButton from "@/components/UserButton.vue";
import type { AppUser } from "@/stores";

const router = useRouter();

defineProps<{
    currentUser: AppUser | null;
    showBackButton?: boolean;
}>();

const emit = defineEmits<{
    (e: "logout"): void;
    (e: "back"): void;
}>();

const handleBack = () => {
    emit("back");
    router.back();
};
</script>

<style scoped>
.page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: var(--header-height);
}

.header-left {
    display: flex;
    align-items: center;
    gap: 16px;
}

.header-right {
    display: flex;
    align-items: center;
}

.back-button {
    padding: 8px 16px;
    height: 40px;
}

.back-icon {
    font-size: 1.2em;
    line-height: 1;
}
</style>
