<template>
  <div class="artist-page">
    <div class="page-header">
      <div class="header-left">
        <button class="back-button" @click="router.back()">
          <span class="back-icon">←</span>
          Back
        </button>
        <UsernameSelector v-model="selectedUser" :users="users" @update:modelValue="fetchArtistData" />
      </div>
      <UserButton 
        :current-user="currentUser"
        @logout="handleLogout"
        @profile="handleProfile"
      />
    </div>
    <div class="artist-header">
      <img :src="artist.imageUrl" :alt="artist.name" class="artist-image" />
      <div class="artist-info">
        <h1>{{ artist.name }}</h1>
        <div class="artist-stats">
          <div class="stat">
            <span class="stat-value">{{ artist.playCount }}</span>
            <span class="stat-label">plays</span>
          </div>
          <div class="stat">
            <span class="stat-value">{{ formatDuration(artist.duration) }}</span>
            <span class="stat-label">listened</span>
          </div>
        </div>
      </div>
    </div>

    <div class="content-section">
      <h2>Top Tracks</h2>
      <div class="tracks-table">
        <div v-for="(track, index) in artist.topTracks" :key="track.id" class="track-row">
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
                :style="{ width: `${(track.playCount / artist.topTracks[0].playCount) * 100}%` }"
              ></div>
            </div>
            <span class="progress-value">{{ Math.round((track.playCount / artist.topTracks[0].playCount) * 100) }}%</span>
          </div>
        </div>
      </div>
    </div>

    <div class="content-section">
      <h2>Albums</h2>
      <div class="albums-grid">
        <div v-for="album in artist.albums" :key="album.id" class="album-card">
          <router-link :to="{ name: 'album', params: { id: album.id }}">
            <img :src="album.imageUrl" :alt="album.title" class="album-image" />
          </router-link>
          <div class="album-info">
            <h3>{{ album.title }}</h3>
            <p>{{ album.playCount }} plays / {{ formatDuration(album.duration) }}</p>
          </div>
        </div>
      </div>
    </div>

    <div class="content-section">
      <h2>Recent Listens</h2>
      <div class="recent-listens-table">
        <table>
          <thead>
            <tr>
              <th>Track</th>
              <th>Last Played</th>
              <th>User</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="track in artist.recentListens" :key="track.id">
              <td class="track-cell">
                <img :src="track.imageUrl" :alt="track.title" class="track-thumbnail" />
                <span>{{ track.title }}</span>
              </td>
              <td>{{ formatTimeAgo(track.lastPlayed) }}</td>
              <td>{{ track.user }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { fetchArtistDetails } from '@/services/mockData'
import { fetchUsers } from '@/services/mockUsers'
import UserButton from '@/components/UserButton.vue'
import UsernameSelector from '@/components/UsernameSelector.vue'
import type { User, ArtistDetails } from '@/types/music'

const route = useRoute()
const router = useRouter()
const artist = ref<ArtistDetails>({
  id: '',
  name: '',
  imageUrl: '',
  playCount: 0,
  duration: 0,
  topTracks: [],
  albums: []
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

const fetchArtistData = async () => {
  const artistId = route.params.id as string
  artist.value = await fetchArtistDetails(artistId)
}

onMounted(async () => {
  try {
    const [artistData, usersData] = await Promise.all([
      fetchArtistDetails(route.params.id as string),
      fetchUsers()
    ])
    artist.value = artistData
    users.value = usersData
  } catch (error) {
    console.error('Error fetching data:', error)
  }
})
</script>

<style scoped>
.artist-page {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.back-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  margin-bottom: 0;
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

.artist-header {
  display: flex;
  gap: 30px;
  margin-bottom: 40px;
  padding: 20px;
  background: var(--card-background);
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
}

.artist-image {
  width: 200px;
  height: 200px;
  border-radius: 8px;
  object-fit: cover;
}

.artist-info {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.artist-info h1 {
  color: var(--text-color);
  margin: 0 0 20px 0;
  font-size: 2em;
}

.artist-stats {
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

.tracks-table, .albums-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 20px;
}

.track-card, .album-card {
  background: var(--card-background);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
  position: relative;
  transition: transform 0.2s;
}

.track-card:hover, .album-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.4);
}

.track-image, .album-image {
  width: 100%;
  aspect-ratio: 1;
  object-fit: cover;
}

.track-info, .album-info {
  padding: 12px;
}

.track-info h3, .album-info h3 {
  color: var(--text-color);
  margin: 0 0 4px 0;
  font-size: 1em;
}

.track-info p, .album-info p {
  color: var(--text-secondary);
  margin: 0;
  font-size: 0.9em;
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

.rank-badge {
  position: absolute;
  top: 8px;
  left: 8px;
  background: var(--primary-color);
  color: var(--background-color);
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  font-size: 0.9em;
  z-index: 1;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
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

.user-controls {
  display: flex;
  align-items: center;
  gap: 16px;
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

/* Recent Listens Table Styles */
.recent-listens-table {
  background: var(--card-background);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
  width: 100%;
}

.recent-listens-table table {
  width: 100%;
  border-collapse: collapse;
}

.recent-listens-table th {
  text-align: left;
  padding: 12px 16px;
  color: var(--text-secondary);
  font-weight: 500;
  font-size: 0.9em;
  border-bottom: 1px solid var(--border-color);
}

.recent-listens-table td {
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
  color: var(--text-color);
}

.recent-listens-table tr:last-child td {
  border-bottom: none;
}

.recent-listens-table tr:hover {
  background: rgba(255, 255, 255, 0.05);
}

.track-cell {
  display: flex;
  align-items: center;
  gap: 12px;
}

.track-cell img {
  width: 40px;
  height: 40px;
  border-radius: 4px;
  object-fit: cover;
}
</style> 
