<template>
  <div class="user-button" ref="buttonRef">
    <div class="user-info" @click="isOpen = !isOpen">
      <svg class="user-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M12 12C14.21 12 16 10.21 16 8C16 5.79 14.21 4 12 4C9.79 4 8 5.79 8 8C8 10.21 9.79 12 12 12ZM12 14C9.33 14 4 15.34 4 18V20H20V18C20 15.34 14.67 14 12 14Z" fill="currentColor"/>
      </svg>
    </div>
    <div v-if="isOpen" class="user-dropdown">
      <div class="user-option" @click="handleLogout">
        <span class="option-text">Logout</span>
      </div>
      <div class="user-option" @click="handleProfile">
        <span class="option-text">Profile</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import type { User } from '@/types/music'

const props = defineProps<{
  currentUser?: User | null
}>()

const emit = defineEmits<{
  (e: 'logout'): void
  (e: 'profile'): void
}>()

const isOpen = ref(false)
const buttonRef = ref<HTMLElement | null>(null)

const handleClickOutside = (event: MouseEvent) => {
  if (buttonRef.value && !buttonRef.value.contains(event.target as Node)) {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})

const handleLogout = () => {
  emit('logout')
  isOpen.value = false
}

const handleProfile = () => {
  emit('profile')
  isOpen.value = false
}
</script>

<style scoped>
.user-button {
  position: relative;
}

.user-info {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: var(--card-background);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.user-info:hover {
  background: rgba(255, 255, 255, 0.05);
}

.user-icon {
  width: 20px;
  height: 20px;
  color: var(--primary-color);
}

.user-dropdown {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 4px;
  background: var(--card-background);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  z-index: 1000;
  min-width: 150px;
}

.user-option {
  padding: 8px 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.user-option:hover {
  background: rgba(255, 255, 255, 0.05);
}

.option-text {
  color: var(--text-color);
  font-size: 0.9em;
}
</style> 