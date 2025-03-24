<template>
  <div class="profile">
    <div class="page-header">
      <button class="back-button" @click="router.back()">
        <span class="back-icon">←</span>
        Back
      </button>
      <div class="user-controls">
        <UserButton 
          :current-user="currentUser"
          @logout="handleLogout"
          @profile="handleProfile"
        />
      </div>
    </div>

    <div class="profile-section">
      <h2>Profile</h2>
      <div class="profile-info">
        <img :src="user.imageUrl" :alt="user.name" class="profile-image" />
        <div class="profile-details">
          <h3>{{ user.name }}</h3>
          <p>Last active: {{ formatTimeAgo(user.lastActive) }}</p>
        </div>
      </div>
    </div>

    <div class="profile-section">
      <h2>Listening Stats</h2>
      <div class="stats-grid">
        <div class="stat-card">
          <span class="stat-value">{{ currentUser?.stats?.totalPlays || 0 }}</span>
          <span class="stat-label">Total Plays</span>
        </div>
        <div class="stat-card">
          <span class="stat-value">{{ formatDuration(currentUser?.stats?.totalDuration || 0) }}</span>
          <span class="stat-label">Time Listened</span>
        </div>
      </div>

      <div class="top-items">
        <div class="top-section">
          <h3>Top Artists</h3>
          <div class="items-grid">
            <div v-for="artist in currentUser?.stats?.topArtists?.slice(0, 5)" :key="artist.id" class="item-card">
              <img :src="artist.imageUrl" :alt="artist.name" class="item-image" />
              <div class="item-info">
                <h4>{{ artist.name }}</h4>
                <p>{{ artist.playCount }} plays / {{ formatDuration(artist.duration) }}</p>
              </div>
            </div>
          </div>
        </div>

        <div class="top-section">
          <h3>Top Albums</h3>
          <div class="items-grid">
            <div v-for="album in currentUser?.stats?.topAlbums?.slice(0, 5)" :key="album.id" class="item-card">
              <img :src="album.imageUrl" :alt="album.title" class="item-image" />
              <div class="item-info">
                <h4>{{ album.title }}</h4>
                <p>{{ album.playCount }} plays / {{ formatDuration(album.duration) }}</p>
              </div>
            </div>
          </div>
        </div>

        <div class="top-section">
          <h3>Top Tracks</h3>
          <div class="items-grid">
            <div v-for="track in currentUser?.stats?.topTracks?.slice(0, 5)" :key="track.id" class="item-card">
              <img :src="track.imageUrl" :alt="track.title" class="item-image" />
              <div class="item-info">
                <h4>{{ track.title }}</h4>
                <p>{{ track.playCount }} plays / {{ formatDuration(track.duration) }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="profile-section">
      <div class="section-header">
        <h2>API Keys</h2>
        <button class="create-key-btn" @click="showCreateKeyModal = true">
          Create API Key
        </button>
      </div>

      <div class="api-keys-list">
        <div v-for="apiKey in user.apiKeys" :key="apiKey.id" class="api-key-card">
          <div class="api-key-info">
            <h3>{{ apiKey.label }}</h3>
            <div class="api-key-details">
              <p>Created: {{ formatDate(apiKey.createdAt) }}</p>
              <p v-if="apiKey.lastUsed">Last used: {{ formatTimeAgo(apiKey.lastUsed) }}</p>
            </div>
          </div>
          <div class="api-key-value">
            <code>{{ apiKey.key }}</code>
            <button class="copy-btn" @click="copyToClipboard(apiKey.key)">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M8 4v12a2 2 0 002 2h8a2 2 0 002-2V7.242a2 2 0 00-.602-1.43L16.083 2.57A2 2 0 0014.685 2H10a2 2 0 00-2 2z" />
                <path d="M16 18v2a2 2 0 01-2 2H6a2 2 0 01-2-2V9a2 2 0 012-2h2" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Create API Key Modal -->
    <div v-if="showCreateKeyModal" class="modal-overlay" @click="showCreateKeyModal = false">
      <div class="modal-content" @click.stop>
        <h3>Create New API Key</h3>
        <div class="form-group">
          <label for="key-label">Label</label>
          <input 
            id="key-label"
            v-model="newKeyLabel"
            type="text"
            placeholder="e.g., Personal App"
            class="input"
          />
        </div>
        <div class="modal-actions">
          <button class="cancel-btn" @click="showCreateKeyModal = false">Cancel</button>
          <button 
            class="create-btn" 
            @click="createApiKey"
            :disabled="!newKeyLabel.trim()"
          >
            Create
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { getUserProfile, createApiKey as createApiKeyService } from '@/services/mockData'
import UserButton from '@/components/UserButton.vue'
import type { User, ApiKey } from '@/types/music'

const router = useRouter()
const user = ref<User>({
  id: '1',
  name: 'John Doe',
  imageUrl: 'https://picsum.photos/32/32?random=1',
  lastActive: new Date().toISOString(),
  apiKeys: [],
  stats: {
    totalPlays: 1234,
    totalDuration: 45678,
    topArtists: [],
    topAlbums: [],
    topTracks: []
  }
})

const currentUser = ref<User | null>({
  id: '1',
  name: 'John Doe',
  imageUrl: 'https://picsum.photos/32/32?random=1',
  lastActive: new Date().toISOString(),
  apiKeys: [],
  stats: {
    totalPlays: 1234,
    totalDuration: 45678,
    topArtists: [],
    topAlbums: [],
    topTracks: []
  }
})

const showCreateKeyModal = ref(false)
const newKeyLabel = ref('')

const handleLogout = () => {
  // Implement logout logic here
  console.log('Logout clicked')
}

const handleProfile = () => {
  // Implement profile navigation logic here
  console.log('Profile clicked')
}

const formatTimeAgo = (timestamp: string): string => {
  const date = new Date(timestamp)
  const now = new Date()
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000)

  if (diffInSeconds < 60) {
    return 'just now'
  } else if (diffInSeconds < 3600) {
    const minutes = Math.floor(diffInSeconds / 60)
    return `${minutes}m ago`
  } else if (diffInSeconds < 86400) {
    const hours = Math.floor(diffInSeconds / 3600)
    return `${hours}h ago`
  } else if (diffInSeconds < 604800) {
    const days = Math.floor(diffInSeconds / 86400)
    return `${days}d ago`
  } else if (diffInSeconds < 2592000) {
    const weeks = Math.floor(diffInSeconds / 604800)
    return `${weeks}w ago`
  } else if (diffInSeconds < 31536000) {
    const months = Math.floor(diffInSeconds / 2592000)
    return `${months}mo ago`
  } else {
    const years = Math.floor(diffInSeconds / 31536000)
    return `${years}y ago`
  }
}

const formatDate = (timestamp: string): string => {
  return new Date(timestamp).toLocaleDateString()
}

const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text)
    // You could add a toast notification here
  } catch (err) {
    console.error('Failed to copy text:', err)
  }
}

const createApiKey = async () => {
  if (!newKeyLabel.value.trim()) return

  try {
    const newKey = await createApiKeyService(user.value.id, newKeyLabel.value)
    user.value.apiKeys.push(newKey)
    showCreateKeyModal.value = false
    newKeyLabel.value = ''
  } catch (error) {
    console.error('Error creating API key:', error)
  }
}

const formatDuration = (minutes: number): string => {
  if (minutes < 60) {
    return `${minutes}m`
  } else {
    const hours = Math.floor(minutes / 60)
    const remainingMinutes = minutes % 60
    return remainingMinutes > 0 ? `${hours}h ${remainingMinutes}m` : `${hours}h`
  }
}

onMounted(async () => {
  try {
    user.value = await getUserProfile('1')
    currentUser.value = user.value
  } catch (error) {
    console.error('Error fetching user profile:', error)
  }
})
</script>

<style scoped>
.profile {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
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
}

.back-button:hover {
  background: rgba(255, 255, 255, 0.05);
  transform: translateX(-2px);
}

.back-icon {
  font-size: 1.2em;
  line-height: 1;
}

.profile-section {
  background: var(--card-background);
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

h2 {
  color: var(--text-color);
  margin: 0;
}

h3 {
  color: var(--text-color);
  margin: 0 0 8px 0;
}

.profile-info {
  display: flex;
  align-items: center;
  gap: 20px;
}

.profile-image {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  object-fit: cover;
}

.profile-details p {
  color: var(--text-secondary);
  margin: 0;
}

.create-key-btn {
  padding: 8px 16px;
  background: var(--primary-color);
  color: var(--background-color);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9em;
  transition: all 0.2s;
}

.create-key-btn:hover {
  opacity: 0.9;
}

.api-keys-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.api-key-card {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 6px;
  padding: 16px;
}

.api-key-info {
  margin-bottom: 12px;
}

.api-key-details {
  display: flex;
  gap: 16px;
  color: var(--text-secondary);
  font-size: 0.9em;
}

.api-key-value {
  display: flex;
  align-items: center;
  gap: 8px;
  background: rgba(0, 0, 0, 0.2);
  padding: 8px 12px;
  border-radius: 4px;
  font-family: monospace;
  font-size: 0.9em;
  color: var(--text-color);
}

.copy-btn {
  padding: 4px;
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.copy-btn:hover {
  color: var(--text-color);
}

.icon {
  width: 16px;
  height: 16px;
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--card-background);
  border-radius: 8px;
  padding: 24px;
  width: 100%;
  max-width: 400px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  color: var(--text-color);
}

.input {
  width: 100%;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-color);
  font-size: 0.9em;
}

.input:focus {
  outline: none;
  border-color: var(--primary-color);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.cancel-btn {
  padding: 8px 16px;
  background: none;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-color);
  cursor: pointer;
  transition: all 0.2s;
}

.cancel-btn:hover {
  background: rgba(255, 255, 255, 0.05);
}

.create-btn {
  padding: 8px 16px;
  background: var(--primary-color);
  border: none;
  border-radius: 4px;
  color: var(--background-color);
  cursor: pointer;
  transition: all 0.2s;
}

.create-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.create-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-bottom: 30px;
}

.stat-card {
  background: var(--card-background);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
  text-align: center;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
}

.stat-value {
  display: block;
  color: var(--text-color);
  font-size: 2em;
  font-weight: bold;
  margin-bottom: 8px;
}

.stat-label {
  color: var(--text-secondary);
  font-size: 0.9em;
}

.top-items {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 30px;
}

.top-section {
  background: var(--card-background);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
}

.top-section h3 {
  color: var(--text-color);
  margin: 0 0 20px 0;
  font-size: 1.2em;
}

.items-grid {
  display: grid;
  gap: 16px;
}

.item-card {
  display: flex;
  gap: 12px;
  padding: 12px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 6px;
  transition: transform 0.2s;
}

.item-card:hover {
  transform: translateX(4px);
}

.item-image {
  width: 60px;
  height: 60px;
  border-radius: 4px;
  object-fit: cover;
}

.item-info {
  flex: 1;
}

.item-info h4 {
  color: var(--text-color);
  margin: 0 0 4px 0;
  font-size: 1em;
}

.item-info p {
  color: var(--text-secondary);
  margin: 0;
  font-size: 0.9em;
}
</style> 