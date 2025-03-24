<template>
  <div class="stats">
    <div class="global-filters">
      <div class="controls">
        <UsernameSelector v-model="selectedUser" :users="users" />
      </div>
      <div class="user-controls">
        <UserButton 
          :current-user="currentUser"
          @logout="handleLogout"
          @profile="handleProfile"
        />
      </div>
    </div>

    <div class="stats-section">
      <div class="section-header">
        <h2>
          <svg class="title-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
            <circle cx="12" cy="7" r="4"/>
          </svg>
          Artists Chart
        </h2>
        <TimeRangeSelector v-model="timeRanges.artists" @update:modelValue="fetchStats" />
      </div>
      <div class="grid-container">
        <div v-for="(artist, index) in stats.topArtists.slice(0, 5)" :key="artist.id" class="card">
          <div class="rank-badge">#{{ index + 1 }}</div>
          <router-link :to="{ name: 'artist', params: { id: artist.id }}">
            <img :src="artist.imageUrl" :alt="artist.name" class="card-image" />
          </router-link>
          <div class="card-content">
            <h3>{{ artist.name }}</h3>
            <p>{{ artist.playCount }} plays / {{ formatDuration(artist.duration) }}</p>
          </div>
        </div>
      </div>
    </div>

    <div class="stats-section">
      <div class="section-header">
        <h2>
          <svg class="title-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"/>
            <path d="M19 10v2a7 7 0 0 1-14 0v-2"/>
            <line x1="12" y1="19" x2="12" y2="23"/>
          </svg>
          Tracks Chart
        </h2>
        <TimeRangeSelector v-model="timeRanges.tracks" @update:modelValue="fetchStats" />
      </div>
      <div class="grid-container">
        <div v-for="(track, index) in stats.topTracks.slice(0, 5)" :key="track.id" class="card">
          <div class="rank-badge">#{{ index + 1 }}</div>
          <img :src="track.imageUrl" :alt="track.title" class="card-image" />
          <div class="card-content">
            <h3>{{ track.title }}</h3>
            <p>{{ track.artist }}</p>
            <p>{{ track.playCount }} plays / {{ formatDuration(track.duration) }}</p>
          </div>
        </div>
      </div>
    </div>

    <div class="stats-section">
      <div class="section-header">
        <h2>
          <svg class="title-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <circle cx="12" cy="12" r="3"/>
            <path d="M12 2v4"/>
            <path d="M12 18v4"/>
            <path d="M2 12h4"/>
            <path d="M18 12h4"/>
          </svg>
          Albums Chart
        </h2>
        <TimeRangeSelector v-model="timeRanges.albums" @update:modelValue="fetchStats" />
      </div>
      <div class="grid-container">
        <div v-for="(album, index) in stats.topAlbums.slice(0, 5)" :key="album.id" class="card">
          <div class="rank-badge">#{{ index + 1 }}</div>
          <router-link :to="{ name: 'album', params: { id: album.id }}">
            <img :src="album.imageUrl" :alt="album.title" class="card-image" />
          </router-link>
          <div class="card-content">
            <h3>{{ album.title }}</h3>
            <p>{{ album.artist }}</p>
            <p>{{ album.playCount }} plays / {{ formatDuration(album.duration) }}</p>
          </div>
        </div>
      </div>
    </div>

    <div class="stats-section">
      <div class="section-header">
        <h2>
          <svg class="title-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polygon points="5 3 19 12 5 21 5 3"/>
          </svg>
          Recent Tracks
        </h2>
        <div class="view-toggle">
          <button 
            class="toggle-btn" 
            :class="{ active: showDuration }"
            @click="showDuration = !showDuration"
            :title="showDuration ? 'Show Plays' : 'Show Duration'"
          >
            <svg v-if="showDuration" class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <path d="M12 6v6l4 2"/>
            </svg>
            <svg v-else class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M8 5v14l11-7z"/>
            </svg>
          </button>
        </div>
      </div>
      <div class="stats-summary">
        <div class="stat-item">
          <span class="stat-label">Today</span>
          <span class="stat-value">
            {{ showDuration ? formatDuration(stats.timeStats.today.duration) : stats.timeStats.today.playCount }}
          </span>
        </div>
        <div class="stat-item">
          <span class="stat-label">This Week</span>
          <span class="stat-value">
            {{ showDuration ? formatDuration(stats.timeStats.week.duration) : stats.timeStats.week.playCount }}
          </span>
        </div>
        <div class="stat-item">
          <span class="stat-label">This Month</span>
          <span class="stat-value">
            {{ showDuration ? formatDuration(stats.timeStats.month.duration) : stats.timeStats.month.playCount }}
          </span>
        </div>
        <div class="stat-item">
          <span class="stat-label">This Year</span>
          <span class="stat-value">
            {{ showDuration ? formatDuration(stats.timeStats.year.duration) : stats.timeStats.year.playCount }}
          </span>
        </div>
        <div class="stat-item">
          <span class="stat-label">All Time</span>
          <span class="stat-value">
            {{ showDuration ? formatDuration(stats.timeStats.all.duration) : stats.timeStats.all.playCount }}
          </span>
        </div>
      </div>
      <div class="recent-tracks-table">
        <div v-for="track in paginatedTracks" :key="track.id" class="recent-track-row">
          <div class="time-column">{{ formatTimeAgo(track.lastPlayed) }}</div>
          <div class="user-column">{{ track.user }}</div>
          <div class="track-column">
            <img :src="track.imageUrl" :alt="track.title" class="track-thumbnail" />
            {{ track.artist }} - {{ track.title }}
          </div>
        </div>
      </div>
      <div class="pagination">
        <button 
          class="page-btn" 
          :disabled="currentPage === 1"
          @click="currentPage--"
        >
          Previous
        </button>
        <span class="page-info">Page {{ currentPage }} of {{ totalPages }}</span>
        <button 
          class="page-btn" 
          :disabled="currentPage === totalPages"
          @click="currentPage++"
        >
          Next
        </button>
      </div>
    </div>

    <div class="stats-section">
      <div class="section-header">
        <h2>
          <svg class="title-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v4"/>
            <path d="M12 18v4"/>
            <path d="M4.93 4.93l2.83 2.83"/>
            <path d="M16.24 16.24l2.83 2.83"/>
            <path d="M2 12h4"/>
            <path d="M18 12h4"/>
            <path d="M4.93 19.07l2.83-2.83"/>
            <path d="M16.24 7.76l2.83-2.83"/>
          </svg>
          Pulse
        </h2>
        <div class="pulse-filters">
          <button
            v-for="range in pulseTimeRanges"
            :key="range"
            class="pulse-filter-btn"
            :class="{ active: selectedPulseRange === range }"
            @click="selectedPulseRange = range"
          >
            {{ range.replace(/(\d+)(\w+)/, '$1 $2') }}
          </button>
        </div>
      </div>
      <div class="pulse-table">
        <div v-for="data in stats.pulseData" :key="data.period" class="pulse-row">
          <div class="pulse-period">{{ data.period }}</div>
          <div class="pulse-count">{{ data.playCount }} plays</div>
          <div class="pulse-bar-container">
            <div 
              class="pulse-bar"
              :style="{ width: `${(data.playCount / Math.max(...stats.pulseData.map(d => d.playCount))) * 100}%` }"
            ></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { TimeRange, MusicStats, User, PulseTimeRange } from '@/types/music'
import { fetchMusicStats } from '@/services/mockData'
import { fetchUsers } from '@/services/mockUsers'
import TimeRangeSelector from '@/components/TimeRangeSelector.vue'
import UsernameSelector from '@/components/UsernameSelector.vue'
import UserButton from '@/components/UserButton.vue'
import { useRouter } from 'vue-router'

interface TimeRanges {
  artists: TimeRange
  tracks: TimeRange
  albums: TimeRange
  recent: TimeRange
}

const timeRanges = ref<TimeRanges>({
  artists: 'week',
  tracks: 'week',
  albums: 'week',
  recent: 'week'
})

const selectedUsers = ref<string[]>(['all'])

const stats = ref<MusicStats>({
  topArtists: [],
  topTracks: [],
  topAlbums: [],
  recentTracks: [],
  timeStats: {
    today: { playCount: 0, duration: 0 },
    week: { playCount: 0, duration: 0 },
    month: { playCount: 0, duration: 0 },
    year: { playCount: 0, duration: 0 },
    all: { playCount: 0, duration: 0 }
  },
  pulseData: []
})

const users = ref<User[]>([])
const selectedUser = ref<User | null>(null)

const ITEMS_PER_PAGE = 20
const currentPage = ref(1)

const totalPages = computed(() => {
  return Math.ceil(stats.value.recentTracks.length / ITEMS_PER_PAGE)
})

const paginatedTracks = computed(() => {
  const start = (currentPage.value - 1) * ITEMS_PER_PAGE
  const end = start + ITEMS_PER_PAGE
  return stats.value.recentTracks.slice(start, end)
})

const showDuration = ref(false)

const pulseTimeRanges: PulseTimeRange[] = ['12days', '12weeks', '12months', '12years']
const selectedPulseRange = ref<PulseTimeRange>('12months')

const router = useRouter()

const fetchStats = async () => {
  try {
    const [statsData, usersData] = await Promise.all([
      fetchMusicStats(timeRanges.value.artists, selectedPulseRange.value),
      fetchUsers()
    ])
    stats.value = statsData
    users.value = usersData
    currentPage.value = 1 // Reset to first page when fetching new data
  } catch (error) {
    console.error('Error fetching data:', error)
  }
}

// Watch for changes in the pulse range
watch(selectedPulseRange, () => {
  fetchStats()
})

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

const formatDuration = (minutes: number): string => {
  if (minutes < 60) {
    return `${minutes}m`
  } else {
    const hours = Math.floor(minutes / 60)
    const remainingMinutes = minutes % 60
    return remainingMinutes > 0 ? `${hours}h ${remainingMinutes}m` : `${hours}h`
  }
}

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

const handleProfile = () => {
  router.push({ name: 'profile' })
}

onMounted(async () => {
  try {
    const [statsData, usersData] = await Promise.all([
      fetchMusicStats(timeRanges.value.artists),
      fetchUsers()
    ])
    stats.value = statsData
    users.value = usersData
  } catch (error) {
    console.error('Error fetching data:', error)
  }
})
</script>

<style scoped>
.stats {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.global-filters {
  margin-bottom: 30px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 20px;
}

.controls {
  display: flex;
  gap: 20px;
}

.user-controls {
  display: flex;
  align-items: center;
}

.stats-section {
  margin-bottom: 40px;
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
  display: flex;
  align-items: center;
  gap: 8px;
}

.title-icon {
  width: 20px;
  height: 20px;
  color: var(--primary-color);
}

.grid-container {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 12px;
  margin-bottom: 20px;
  align-items: start;
}

.grid-container .card {
  grid-column: span 1;
  grid-row: 1 / span 2;
}

.card {
  background: var(--card-background);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  transition: transform 0.2s;
  border: 1px solid var(--border-color);
  position: relative;
  aspect-ratio: 1;
}

.card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.4);
}

.card-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  cursor: pointer;
}

.card-content {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 8px;
  background: linear-gradient(transparent, rgba(0,0,0,0.8));
  color: white;
}

.card-content h3 {
  margin: 0 0 4px 0;
  font-size: 0.9em;
  color: white;
}

.card-content p {
  margin: 2px 0;
  color: rgba(255,255,255,0.8);
  font-size: 0.8em;
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

.recent-tracks-table {
  background: var(--card-background);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
  margin-bottom: 16px;
}

.recent-track-row {
  display: grid;
  grid-template-columns: 120px 150px 1fr;
  padding: 8px 12px;
  border-bottom: 1px solid var(--border-color);
  align-items: center;
}

.recent-track-row:last-child {
  border-bottom: none;
}

.recent-track-row:hover {
  background: rgba(255, 255, 255, 0.05);
}

.time-column {
  color: var(--text-secondary);
  font-size: 0.9em;
}

.user-column {
  color: var(--text-color);
  font-size: 0.9em;
}

.track-column {
  color: var(--text-color);
  font-size: 0.9em;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: flex;
  align-items: center;
  gap: 8px;
}

.track-thumbnail {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  object-fit: cover;
  flex-shrink: 0;
}

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 16px;
  margin-top: 16px;
}

.page-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--card-background);
  color: var(--text-color);
  cursor: pointer;
  font-size: 0.9em;
  transition: all 0.2s;
}

.page-btn:hover:not(:disabled) {
  background: var(--background-color);
}

.page-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.page-info {
  color: var(--text-secondary);
  font-size: 0.9em;
}

.stats-summary {
  display: flex;
  gap: 20px;
  margin-bottom: 20px;
  padding: 16px;
  background: var(--card-background);
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 1;
}

.stat-label {
  color: var(--text-secondary);
  font-size: 0.9em;
  margin-bottom: 4px;
}

.stat-value {
  color: var(--text-color);
  font-size: 1.2em;
  font-weight: bold;
}

.view-toggle {
  display: flex;
  align-items: center;
}

.toggle-btn {
  padding: 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--card-background);
  color: var(--text-color);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.toggle-btn:hover {
  background: var(--background-color);
}

.toggle-btn.active {
  background: var(--primary-color);
  color: var(--background-color);
  border-color: var(--primary-color);
}

.icon {
  width: 20px;
  height: 20px;
}

.pulse-filters {
  display: flex;
  gap: 8px;
}

.pulse-filter-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--card-background);
  color: var(--text-color);
  cursor: pointer;
  font-size: 0.9em;
  transition: all 0.2s;
}

.pulse-filter-btn:hover {
  background: var(--background-color);
}

.pulse-filter-btn.active {
  background: var(--primary-color);
  color: var(--background-color);
  border-color: var(--primary-color);
}

.pulse-table {
  background: var(--card-background);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
}

.pulse-row {
  display: grid;
  grid-template-columns: 150px 120px 1fr;
  padding: 12px;
  border-bottom: 1px solid var(--border-color);
  align-items: center;
}

.pulse-row:last-child {
  border-bottom: none;
}

.pulse-row:hover {
  background: rgba(255, 255, 255, 0.05);
}

.pulse-period {
  color: var(--text-color);
  font-size: 0.9em;
}

.pulse-count {
  color: var(--text-secondary);
  font-size: 0.9em;
}

.pulse-bar-container {
  height: 6px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
  overflow: hidden;
}

.pulse-bar {
  height: 100%;
  background: var(--primary-color);
  border-radius: 3px;
  transition: width 0.3s ease;
}
</style> 