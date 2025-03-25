<template>
  <div class="import-page">
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

    <div class="import-section">
      <h1>Import Listening History</h1>
      <p class="description">Upload your listening history file to import your data.</p>

      <div class="format-selector">
        <label class="format-label">Import Format:</label>
        <div class="format-options">
          <button 
            v-for="format in importFormats" 
            :key="format.id"
            class="format-option"
            :class="{ 'selected': selectedFormat === format.id }"
            @click="selectedFormat = format.id"
          >
            {{ format.name }}
          </button>
        </div>
      </div>

      <div class="upload-area" 
           @dragover.prevent 
           @drop.prevent="handleDrop"
           :class="{ 'is-dragging': isDragging }">
        <input 
          type="file" 
          ref="fileInput"
          @change="handleFileSelect"
          :accept="selectedFormatInfo.accept"
          class="file-input"
          style="display: none"
        >
        <div class="upload-content">
          <svg class="upload-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="17 8 12 3 7 8"/>
            <line x1="12" y1="3" x2="12" y2="15"/>
          </svg>
          <p>Drag and drop your file here, or <button class="browse-btn" @click="triggerFileInput">browse</button></p>
        </div>
      </div>

      <div v-if="selectedFile" class="selected-file">
        <div class="file-info">
          <span class="file-name">{{ selectedFile.name }}</span>
          <span class="file-size">{{ formatFileSize(selectedFile.size) }}</span>
        </div>
        <button v-if="!isImportSuccessful" class="import-btn" @click="handleImport" :disabled="isLoading">
          <span v-if="!isLoading">Import</span>
          <div v-else class="loader"></div>
        </button>
        <div v-else class="success-container">
          <div class="success-check">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
              <polyline points="22 4 12 14.01 9 11.01"></polyline>
            </svg>
          </div>
        </div>
      </div>

      <div v-if="importHistory.length > 0" class="import-history">
        <h2>Import History</h2>
        <div class="history-list">
          <div v-for="(item, index) in importHistory" :key="index" class="history-item">
            <div class="history-item-info">
              <span class="history-item-name">{{ item.name }}</span>
              <span class="history-item-details">
                {{ formatFileSize(item.size) }} • {{ item.format }} • {{ formatDate(item.timestamp) }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import UserButton from '@/components/UserButton.vue'
import type { User } from '@/types/music'
import { importUserData } from '@/services/mockData'

interface ImportHistoryItem {
  name: string
  size: number
  format: string
  timestamp: string
}

const router = useRouter()
const fileInput = ref<HTMLInputElement | null>(null)
const selectedFile = ref<File | null>(null)
const isDragging = ref(false)
const selectedFormat = ref('lastfm')
const isLoading = ref(false)
const isImportSuccessful = ref(false)
const importHistory = ref<ImportHistoryItem[]>([])

interface ImportFormat {
  id: string
  name: string
  accept: string
  description: string
}

const importFormats: ImportFormat[] = [
  {
    id: 'lastfm',
    name: 'Last.fm',
    accept: '.xml',
    description: 'Last.fm listening history XML file'
  },
  {
    id: 'listenbrainz',
    name: 'ListenBrainz',
    accept: '.json',
    description: 'ListenBrainz listening history JSON file'
  },
  {
    id: 'maloja',
    name: 'Maloja',
    accept: '.json',
    description: 'Maloja listening history JSON file'
  }
]

const selectedFormatInfo = computed(() => {
  return importFormats.find(format => format.id === selectedFormat.value) || importFormats[0]
})

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

const triggerFileInput = () => {
  fileInput.value?.click()
}

const handleFileSelect = (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    selectedFile.value = input.files[0]
  }
}

const handleDrop = (event: DragEvent) => {
  isDragging.value = false
  if (event.dataTransfer?.files && event.dataTransfer.files[0]) {
    selectedFile.value = event.dataTransfer.files[0]
  }
}

const handleImport = async () => {
  if (!selectedFile.value) return

  try {
    isLoading.value = true
    isImportSuccessful.value = false
    console.log('Importing file:', selectedFile.value, 'Format:', selectedFormat.value)

    // Call the mock import function with a delay
    const success = await importUserData(selectedFile.value, selectedFormat.value)

    if (success) {
      // After successful import, show success state
      isImportSuccessful.value = true

      // Add to import history
      importHistory.value.unshift({
        name: selectedFile.value.name,
        size: selectedFile.value.size,
        format: selectedFormatInfo.value.name,
        timestamp: new Date().toISOString()
      })

      // Clear selected file after a short delay to show success state
      setTimeout(() => {
        selectedFile.value = null
        isImportSuccessful.value = false
      }, 2000)
    }
  } catch (error) {
    console.error('Error importing file:', error)
  } finally {
    isLoading.value = false
  }
}

const formatDate = (timestamp: string): string => {
  const date = new Date(timestamp)
  return date.toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
</script>

<style scoped>
.import-page {
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

.import-section {
  max-width: 600px;
  margin: 0 auto;
  padding: 40px;
  background: var(--card-background);
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
}

h1 {
  color: var(--text-color);
  margin: 0 0 8px 0;
  font-size: 1.8em;
}

.description {
  color: var(--text-secondary);
  margin: 0 0 32px 0;
  font-size: 1.1em;
}

.format-selector {
  margin-bottom: 24px;
}

.format-label {
  display: block;
  color: var(--text-color);
  margin-bottom: 12px;
  font-size: 0.9em;
}

.format-options {
  display: flex;
  gap: 12px;
}

.format-option {
  padding: 8px 16px;
  background: var(--card-background);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-color);
  cursor: pointer;
  transition: all 0.2s;
  font-size: 0.9em;
}

.format-option:hover {
  background: rgba(255, 255, 255, 0.05);
}

.format-option.selected {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: var(--background-color);
}

.upload-area {
  border: 2px dashed var(--border-color);
  border-radius: 8px;
  padding: 40px;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s;
  background: rgba(255, 255, 255, 0.02);
}

.upload-area.is-dragging {
  border-color: var(--primary-color);
  background: rgba(255, 255, 255, 0.05);
}

.upload-icon {
  width: 48px;
  height: 48px;
  color: var(--primary-color);
  margin-bottom: 16px;
}

.browse-btn {
  background: none;
  border: none;
  color: var(--primary-color);
  cursor: pointer;
  font-size: inherit;
  padding: 0;
  text-decoration: underline;
}

.browse-btn:hover {
  color: var(--text-color);
}

.file-types {
  color: var(--text-secondary);
  font-size: 0.9em;
  margin-top: 8px;
}

.selected-file {
  margin-top: 24px;
  padding: 16px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 6px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.file-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.file-name {
  color: var(--text-color);
  font-size: 0.9em;
}

.file-size {
  color: var(--text-secondary);
  font-size: 0.8em;
}

.import-btn {
  padding: 8px 16px;
  background: var(--primary-color);
  color: var(--background-color);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9em;
  transition: all 0.2s;
}

.import-btn:hover {
  opacity: 0.9;
}

.import-btn:disabled {
  cursor: not-allowed;
  opacity: 0.7;
}

.loader {
  display: inline-block;
  width: 20px;
  height: 20px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  border-top-color: #fff;
  animation: spin 1s ease-in-out infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.success-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.success-check {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background-color: #4CAF50;
  color: white;
}

.success-check svg {
  width: 24px;
  height: 24px;
}

.import-history {
  margin-top: 32px;
  border-top: 1px solid var(--border-color);
  padding-top: 24px;
}

.import-history h2 {
  color: var(--text-color);
  font-size: 1.4em;
  margin: 0 0 16px 0;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.history-item {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 6px;
  padding: 12px 16px;
  border: 1px solid var(--border-color);
}

.history-item-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.history-item-name {
  color: var(--text-color);
  font-weight: 500;
  font-size: 0.95em;
}

.history-item-details {
  color: var(--text-secondary);
  font-size: 0.85em;
}
</style> 
