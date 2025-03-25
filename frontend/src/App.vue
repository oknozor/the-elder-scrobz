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
    <router-view></router-view>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import PageHeader from '@/components/PageHeader.vue'
import UsernameSelector from '@/components/UsernameSelector.vue'
import { fetchUsers } from '@/services/mockUsers'
import type { User } from '@/types/music'

const router = useRouter()
const route = useRoute()

// Determine if back button should be shown based on the current route
const showBackButton = computed(() => {
  return route.name !== 'stats'
})

// Determine if user selector should be shown based on the current route
const showUserSelector = computed(() => {
  return ['stats', 'artist', 'album'].includes(route.name as string)
})

const users = ref<User[]>([])
const selectedUser = ref<User | null>(null)

const currentUser = ref<User | null>({
  id: '1',
  name: 'John Doe',
  imageUrl: 'https://picsum.photos/32/32?random=1',
  lastActive: new Date().toISOString(),
  apiKeys: [],
  stats: {
    totalPlays: 0,
    totalDuration: 0,
    topArtists: [],
    topAlbums: [],
    topTracks: []
  }
})

const handleLogout = () => {
  // Implement logout logic here
  console.log('Logout clicked')
}

const handleUserChange = (user: User | null) => {
  selectedUser.value = user
  console.log('User changed:', user?.name || 'All Users')

  // Note: In a real application, you might want to use a state management solution
  // like Pinia or Vuex to share the selectedUser state with the views,
  // or implement a more sophisticated event system to trigger data fetching in the views.
}

onMounted(async () => {
  try {
    users.value = await fetchUsers()
  } catch (error) {
    console.error('Error fetching users:', error)
  }
})
</script>

<style>
.app {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}
</style> 
