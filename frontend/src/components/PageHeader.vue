<template>
  <div class="page-header">
    <div class="header-left">
      <button v-if="showBackButton" class="back-button" @click="handleBack">
        <span class="back-icon">←</span>
        Back
      </button>
      <slot name="left"></slot>
    </div>
    <div class="header-right">
      <slot name="right">
        <UserButton 
          :current-user="currentUser"
          @logout="$emit('logout')"
          @profile="$emit('profile')"
        />
      </slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import UserButton from '@/components/UserButton.vue'
import type { User } from '@/types/music'

const router = useRouter()

const props = defineProps<{
  currentUser: User | null
  showBackButton?: boolean
}>()

const emit = defineEmits<{
  (e: 'logout'): void
  (e: 'profile'): void
  (e: 'back'): void
}>()

const handleBack = () => {
  emit('back')
  router.back()
}
</script>

<style scoped>
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
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
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: var(--card-background);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-color);
  font-size: 0.9em;
  cursor: pointer;
  transition: all 0.2s;
  height: 40px;
}

.back-button:hover {
  background: rgba(255, 255, 255, 0.05);
  transform: translateX(-2px);
}

.back-icon {
  font-size: 1.2em;
  line-height: 1;
}
</style>