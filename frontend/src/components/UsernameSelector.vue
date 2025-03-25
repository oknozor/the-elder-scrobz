<template>
  <div class="user-selector" ref="selectorRef">
    <div class="selected-user" @click="isOpen = !isOpen">
      <span class="user-name">{{ modelValue?.name || 'All Users' }}</span>
      <span class="dropdown-icon">{{ isOpen ? '▼' : '▶' }}</span>
    </div>
    <div v-if="isOpen" class="user-dropdown">
      <div 
        class="user-option"
        :class="{ 'selected': !modelValue }"
        @click="selectAll"
      >
        <span class="user-name">All Users</span>
      </div>
      <div 
        v-for="user in users" 
        :key="user.id" 
        class="user-option"
        :class="{ 'selected': modelValue?.id === user.id }"
        @click="selectUser(user)"
      >
        <span class="user-name">{{ user.name }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import type { User } from '@/types/music'

const props = defineProps<{
  modelValue: User | null
  users: User[]
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: User | null): void
}>()

const isOpen = ref(false)
const selectorRef = ref<HTMLElement | null>(null)

const handleClickOutside = (event: MouseEvent) => {
  if (selectorRef.value && !selectorRef.value.contains(event.target as Node)) {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})

const selectUser = (user: User) => {
  emit('update:modelValue', user)
  isOpen.value = false
}

const selectAll = () => {
  emit('update:modelValue', null)
  isOpen.value = false
}
</script>

<style scoped>
.user-selector {
  position: relative;
  width: 200px;
}

.selected-user {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--card-background);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.selected-user:hover {
  background: rgba(255, 255, 255, 0.05);
}

.user-name {
  flex: 1;
  color: var(--text-color);
  font-size: 0.9em;
}

.dropdown-icon {
  color: var(--text-secondary);
  font-size: 0.8em;
}

.user-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background: var(--card-background);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  z-index: 1000;
  max-height: 300px;
  overflow-y: auto;
}

.user-option {
  padding: 8px 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.user-option:hover {
  background: rgba(255, 255, 255, 0.05);
}

.user-option.selected {
  background: rgba(255, 255, 255, 0.1);
}

.user-option:first-child {
  border-bottom: 1px solid var(--border-color);
  margin-bottom: 4px;
  padding-bottom: 12px;
}
</style> 