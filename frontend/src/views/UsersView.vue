<template>
  <div class="users-page">
    <div class="page-header">
      <div class="header-left">
        <button class="back-button" @click="router.back()">
          <span class="back-icon">←</span>
          Back
        </button>
      </div>
      <UserButton 
        :current-user="currentUser"
        @logout="handleLogout"
        @profile="handleProfile"
      />
    </div>

    <div class="users-section">
      <h1>Users</h1>

      <div class="table-container">
        <table class="users-table">
          <thead>
            <tr>
              <th>Username</th>
              <th>Last Login</th>
              <th>Play Count</th>
              <th>Play Duration</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="user in users" :key="user.id">
              <td>
                <div class="user-cell">
                  <img :src="user.imageUrl" :alt="user.name" class="user-avatar" />
                  <span>{{ user.name }}</span>
                </div>
              </td>
              <td>{{ formatDate(user.lastActive) }}</td>
              <td>{{ formatNumber(user.stats.totalPlays) }}</td>
              <td>{{ formatDuration(user.stats.totalDuration) }}</td>
              <td>
                <button class="delete-btn" @click="handleDeleteUser(user.id)">
                  <svg class="trash-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M3 6h18"/>
                    <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/>
                    <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>
                  </svg>
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>

  <!-- Delete Confirmation Modal -->
  <div v-if="showDeleteConfirmation" class="modal-overlay" @click="showDeleteConfirmation = false">
    <div class="modal-content" @click.stop>
      <h3>Confirm Deletion</h3>
      <p>Are you sure you want to delete this user? This action cannot be undone.</p>
      <div class="modal-actions">
        <button class="cancel-btn" @click="showDeleteConfirmation = false">Cancel</button>
        <button class="delete-confirm-btn" @click="confirmDeleteUser">Delete</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import UserButton from '@/components/UserButton.vue'
import type { User } from '@/types/music'

const router = useRouter()

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

// Mock data for users
const users = ref<User[]>([
  {
    id: '1',
    name: 'John Doe',
    imageUrl: 'https://picsum.photos/32/32?random=1',
    lastActive: new Date().toISOString(),
    apiKeys: [],
    stats: {
      totalPlays: 1234,
      totalDuration: 3600000,
      topArtists: [],
      topAlbums: [],
      topTracks: []
    }
  },
  {
    id: '2',
    name: 'Jane Smith',
    imageUrl: 'https://picsum.photos/32/32?random=2',
    lastActive: new Date(Date.now() - 86400000).toISOString(), // 1 day ago
    apiKeys: [],
    stats: {
      totalPlays: 567,
      totalDuration: 1800000,
      topArtists: [],
      topAlbums: [],
      topTracks: []
    }
  }
])

const handleLogout = () => {
  // Implement logout logic here
  console.log('Logout clicked')
}

const handleProfile = () => {
  router.push({ name: 'profile' })
}

const showDeleteConfirmation = ref(false)
const userToDelete = ref<string | null>(null)

const handleDeleteUser = (userId: string) => {
  userToDelete.value = userId
  showDeleteConfirmation.value = true
}

const confirmDeleteUser = () => {
  if (userToDelete.value) {
    // TODO: Implement user deletion logic
    console.log('Delete user:', userToDelete.value)
    users.value = users.value.filter(user => user.id !== userToDelete.value)
    showDeleteConfirmation.value = false
    userToDelete.value = null
  }
}

const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const formatNumber = (num: number): string => {
  return num.toLocaleString()
}

const formatDuration = (ms: number): string => {
  const hours = Math.floor(ms / (1000 * 60 * 60))
  const minutes = Math.floor((ms % (1000 * 60 * 60)) / (1000 * 60))

  if (hours > 0) {
    return `${hours}h ${minutes}m`
  }
  return `${minutes}m`
}
</script>

<style scoped>
.users-page {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

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

.users-section {
  background: var(--card-background);
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
  padding: 24px;
}

h1 {
  color: var(--text-color);
  margin: 0 0 24px 0;
  font-size: 1.8em;
}

.table-container {
  overflow-x: auto;
}

.users-table {
  width: 100%;
  border-collapse: collapse;
}

.users-table th,
.users-table td {
  padding: 12px;
  text-align: left;
  border-bottom: 1px solid var(--border-color);
}

.users-table th {
  color: var(--text-secondary);
  font-weight: 500;
  font-size: 0.9em;
}

.user-cell {
  display: flex;
  align-items: center;
  gap: 12px;
}

.user-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
}

.delete-btn {
  background: none;
  border: none;
  padding: 8px;
  cursor: pointer;
  color: var(--text-secondary);
  transition: all 0.2s;
}

.delete-btn:hover {
  color: #ff4444;
}

.trash-icon {
  width: 20px;
  height: 20px;
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--card-background);
  border-radius: 8px;
  padding: 24px;
  width: 400px;
  max-width: 90%;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-color);
}

.modal-content h3 {
  margin-top: 0;
  margin-bottom: 16px;
  color: var(--text-color);
}

.modal-content p {
  margin-bottom: 24px;
  color: var(--text-secondary);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.cancel-btn {
  padding: 8px 16px;
  background-color: transparent;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-color);
  cursor: pointer;
  transition: all 0.2s;
}

.cancel-btn:hover {
  background-color: rgba(255, 255, 255, 0.05);
}

.delete-confirm-btn {
  padding: 8px 16px;
  background-color: #ff4444;
  border: none;
  border-radius: 4px;
  color: white;
  cursor: pointer;
  transition: all 0.2s;
}

.delete-confirm-btn:hover {
  background-color: #ff6666;
}
</style> 
