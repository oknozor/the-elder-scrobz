<template>
  <div class="album-page">
    <div class="page-header">
      <div class="header-left">
        <button class="back-button" @click="router.back()">
          <span class="back-icon">←</span>
          Back
        </button>
        <UsernameSelector v-model="selectedUser" :users="users" @update:modelValue="fetchAlbumData" />
      </div>
      <UserButton 
        :current-user="currentUser"
        @logout="handleLogout"
        @profile="handleProfile"
      />
    </div>
    <div class="album-header">
      <img :src="album.imageUrl" :alt="album.title" class="album-image" />
      <div class="album-info">
        <h1>{{ album.title }}</h1>
        <p class="artist-name">{{ album.artist }}</p>
        <div class="album-stats">
          <div class="stat">
            <span class="stat-value">{{ album.playCount }}</span>
            <span class="stat-label">plays</span>
          </div>
          <div class="stat">
            <span class="stat-value">{{ formatDuration(album.duration) }}</span>
            <span class="stat-label">listened</span>
          </div>
        </div>
      </div>
    </div>

    <div class="content-section">
      <h2>Top Tracks</h2>
      <div class="tracks-table">
        <div v-for="(track, index) in album.topTracks" :key="track.id" class="track-row">
          <div class="track-rank">#{{ index + 1 }}</div>
          <img :src="track.imageUrl" :alt="track.title" class="track-thumbnail" />
          <div class="track-info">
            <h3>{{ track.title }}</h3>
            <p>{{ track.playCount }} plays / {{ formatDuration(track.duration) }}</p>
          </div>
          <div class="track-progress">
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: `${(track.playCount / album.topTracks[0].playCount) * 100}%` }"
              ></div>
            </div>
            <span class="progress-value">{{ Math.round((track.playCount / album.topTracks[0].playCount) * 100) }}%</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { fetchAlbumDetails } from '@/services/mockData'
import { fetchUsers } from '@/services/mockUsers'
import UserButton from '@/components/UserButton.vue'
import UsernameSelector from '@/components/UsernameSelector.vue'
import type { User, AlbumDetails } from '@/types/music'

const route = useRoute()
const router = useRouter()
const album = ref<AlbumDetails>({
  id: '',
  title: '',
  artist: '',
  imageUrl: '',
  playCount: 0,
  duration: 0,
  topTracks: []
})

const users = ref<User[]>([])
const selectedUser = ref<User | null>(null)

const currentUser = ref<User | null>({
  id: '1',
  name: 'John Doe',
  imageUrl: 'https://picsum.photos/32/32?random=1',
  lastActive: new Date().toISOString(),
  apiKeys: []
})

const handleLogout = () => {
  // Implement logout logic here
  console.log('Logout clicked')
}

const handleProfile = () => {
  router.push({ name: 'profile' })
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

const fetchAlbumData = async () => {
  const albumId = route.params.id as string
  album.value = await fetchAlbumDetails(albumId)
}

onMounted(async () => {
  try {
    const [albumData, usersData] = await Promise.all([
      fetchAlbumDetails(route.params.id as string),
      fetchUsers()
    ])
    album.value = albumData
    users.value = usersData
  } catch (error) {
    console.error('Error fetching data:', error)
  }
})
</script>

<style scoped>
.album-page {
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
  margin-bottom: 0;
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

.album-header {
  display: flex;
  gap: 30px;
  margin-bottom: 40px;
  padding: 20px;
  background: var(--card-background);
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
}

.album-image {
  width: 200px;
  height: 200px;
  border-radius: 8px;
  object-fit: cover;
}

.album-info {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.album-info h1 {
  color: var(--text-color);
  margin: 0 0 8px 0;
  font-size: 2em;
}

.artist-name {
  color: var(--text-secondary);
  margin: 0 0 20px 0;
  font-size: 1.2em;
}

.album-stats {
  display: flex;
  gap: 30px;
}

.stat {
  display: flex;
  flex-direction: column;
}

.stat-value {
  color: var(--text-color);
  font-size: 1.5em;
  font-weight: bold;
}

.stat-label {
  color: var(--text-secondary);
  font-size: 0.9em;
}

.content-section {
  margin-bottom: 40px;
}

.content-section h2 {
  color: var(--text-color);
  margin: 0 0 20px 0;
}

.tracks-table {
  background: var(--card-background);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
  display: table;
  width: 100%;
}

.track-row {
  display: table-row;
  border-bottom: 1px solid var(--border-color);
}

.track-row:last-child {
  border-bottom: none;
}

.track-row:hover {
  background: rgba(255, 255, 255, 0.05);
}

.track-rank {
  display: table-cell;
  color: var(--text-secondary);
  font-size: 0.9em;
  font-weight: bold;
  text-align: center;
  padding: 12px;
  width: 60px;
  vertical-align: middle;
}

.track-thumbnail {
  display: table-cell;
  padding: 12px;
  width: 60px;
  vertical-align: middle;
}

.track-thumbnail img {
  width: 48px;
  height: 48px;
  border-radius: 4px;
  object-fit: cover;
}

.track-info {
  display: table-cell;
  padding: 12px;
  vertical-align: middle;
}

.track-info h3 {
  color: var(--text-color);
  margin: 0 0 4px 0;
  font-size: 1em;
}

.track-info p {
  color: var(--text-secondary);
  margin: 0;
  font-size: 0.9em;
}

.track-progress {
  display: table-cell;
  padding: 12px;
  width: 200px;
  vertical-align: middle;
}

.progress-bar {
  flex: 1;
  height: 6px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--primary-color);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.progress-value {
  color: var(--text-secondary);
  font-size: 0.9em;
  min-width: 45px;
  text-align: right;
}

:deep(.username-selector) {
  height: 40px;
}

:deep(.username-selector select) {
  height: 100%;
  padding: 8px 16px;
  background: var(--card-background);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-color);
  font-size: 0.9em;
  cursor: pointer;
  transition: all 0.2s;
}

:deep(.username-selector select:hover) {
  background: rgba(255, 255, 255, 0.05);
}
</style> 