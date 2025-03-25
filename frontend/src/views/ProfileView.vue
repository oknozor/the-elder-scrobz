<template>
  <div class="profile">

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
            <router-link 
              v-for="artist in currentUser?.stats?.topArtists?.slice(0, 5)" 
              :key="artist.id" 
              :to="{ name: 'artist', params: { id: artist.id }}"
              class="item-card"
            >
              <img :src="artist.imageUrl" :alt="artist.name" class="item-image" />
              <div class="item-info">
                <h4>{{ artist.name }}</h4>
                <p>{{ artist.playCount }} plays / {{ formatDuration(artist.duration) }}</p>
              </div>
            </router-link>
          </div>
        </div>

        <div class="top-section">
          <h3>Top Albums</h3>
          <div class="items-grid">
            <router-link 
              v-for="album in currentUser?.stats?.topAlbums?.slice(0, 5)" 
              :key="album.id" 
              :to="{ name: 'album', params: { id: album.id }}"
              class="item-card"
            >
              <img :src="album.imageUrl" :alt="album.title" class="item-image" />
              <div class="item-info">
                <h4>{{ album.title }}</h4>
                <p>{{ album.playCount }} plays / {{ formatDuration(album.duration) }}</p>
              </div>
            </router-link>
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

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { getUserProfile } from '@/services/mockData'
import type { User } from '@/types/music'

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
  padding-top: 20px;
  max-width: 1200px;
  margin: 0 auto;
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
  text-decoration: none;
  color: inherit;
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
