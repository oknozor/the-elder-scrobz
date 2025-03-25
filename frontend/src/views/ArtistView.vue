<template>
  <div class="artist-page">
    <div class="artist-header">
      <img :src="artist.imageUrl" :alt="artist.name" class="artist-image" />
      <div class="artist-info">
        <h1>{{ artist.name }}</h1>
        <p v-if="artist.description" class="artist-description">{{ artist.description }}</p>
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
          <div class="track-thumbnail-container">
            <img :src="track.imageUrl" :alt="track.title" class="track-thumbnail" />
            <div class="track-play-icon">
              <svg class="play-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M8 5v14l11-7z"/>
              </svg>
            </div>
          </div>
          <div class="track-info">
            <h3>{{ track.title }}</h3>
          </div>
          <div class="track-progress">
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: `${(track.playCount / artist.topTracks[0].playCount) * 100}%` }"
              ></div>
            </div>
            <p class="track-stats">{{ track.playCount }} plays / {{ formatDuration(track.duration) }}</p>
          </div>
        </div>
      </div>
    </div>

    <div class="content-section">
      <h2>Albums</h2>
      <div class="albums-grid">
        <div v-for="album in artist.albums" :key="album.id" class="album-card">
          <router-link :to="{ name: 'album', params: { id: album.id }}">
            <div class="album-image-container">
              <img :src="album.imageUrl" :alt="album.title" class="album-image" />
            </div>
          </router-link>
          <div class="album-content">
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
                <div class="track-cell-container">
                  <img :src="track.imageUrl" :alt="track.title" class="track-thumbnail" />
                  <span>{{ track.title }}</span>
                  <svg class="play-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M8 5v14l11-7z"/>
                  </svg>
                </div>
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
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { fetchArtistDetails } from '@/services/mockData'
import { fetchUsers } from '@/services/mockUsers'
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
  albums: [],
  recentListens: []
})

const users = ref<User[]>([])
const selectedUser = ref<User | null>(null)

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
  const username = selectedUser.value?.name || null
  artist.value = await fetchArtistDetails(artistId, username)
}

// Watch for changes in the selected user
watch(selectedUser, () => {
  fetchArtistData()
})

onMounted(async () => {
  try {
    const username = selectedUser.value?.name || null
    const [artistData, usersData] = await Promise.all([
      fetchArtistDetails(route.params.id as string, username),
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
  padding-top: 20px;
  max-width: 1200px;
  margin: 0 auto;
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
  margin: 0 0 10px 0;
  font-size: 2em;
}

.artist-description {
  color: var(--text-secondary);
  margin: 0 0 20px 0;
  font-size: 1em;
  line-height: 1.5;
  max-width: 800px;
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

.album-card {
  position: relative;
  overflow: hidden;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
  transition: transform 0.2s;
  aspect-ratio: 1;
}

.album-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.4);
}

.album-image-container {
  position: relative;
  width: 100%;
  aspect-ratio: 1;
}

.album-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.album-content {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 8px;
  background: rgba(0, 0, 0, 0.7);
  color: white;
  opacity: 0;
  transform: translateY(100%);
  transition: all 0.2s ease;
}

.album-card:hover .album-content {
  opacity: 1;
  transform: translateY(0);
}

.album-content h3 {
  margin: 0 0 4px 0;
  font-size: 0.9em;
  color: white;
}

.album-content p {
  margin: 2px 0;
  color: rgba(255,255,255,0.8);
  font-size: 0.8em;
}

.album-play-icon {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: rgba(0, 0, 0, 0.6);
  border-radius: 50%;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.2s;
}

.album-play-icon .play-icon {
  width: 24px;
  height: 24px;
  color: white;
}

.album-image-container:hover .album-play-icon {
  opacity: 1;
}

.track-info {
  padding: 12px;
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

.track-thumbnail-container {
  position: relative;
  display: table-cell;
  padding: 12px;
  width: 60px;
  vertical-align: middle;
}

.track-thumbnail {
  width: 48px;
  height: 48px;
  border-radius: 4px;
  object-fit: cover;
}

.track-play-icon {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: rgba(0, 0, 0, 0.6);
  border-radius: 50%;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.2s;
}

.track-play-icon .play-icon {
  width: 16px;
  height: 16px;
  color: white;
}

.track-thumbnail-container:hover .track-play-icon {
  opacity: 1;
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
  width: 65%;
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

.track-stats {
  color: var(--text-secondary);
  font-size: 0.9em;
  margin: 6px 0 0 0;
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
}

.track-cell-container {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
}

.track-cell img {
  width: 40px;
  height: 40px;
  border-radius: 4px;
  object-fit: cover;
}

.track-cell .play-icon {
  width: 16px;
  height: 16px;
  color: var(--primary-color);
  opacity: 0;
  transition: opacity 0.2s;
  cursor: pointer;
  margin-left: 8px;
}

.track-cell:hover .play-icon {
  opacity: 1;
}
</style> 
