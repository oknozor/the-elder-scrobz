<template>
  <div class="api-keys">
    <div class="api-keys-section">
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
            <button class="copy-btn" @click="copyToClipboard(apiKey.key)" title="Copy API key">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M8 4v12a2 2 0 002 2h8a2 2 0 002-2V7.242a2 2 0 00-.602-1.43L16.083 2.57A2 2 0 0014.685 2H10a2 2 0 00-2 2z" />
                <path d="M16 18v2a2 2 0 01-2 2H6a2 2 0 01-2-2V9a2 2 0 012-2h2" />
              </svg>
            </button>
            <button class="delete-btn" @click="confirmDeleteApiKey(apiKey)" title="Delete API key">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2" />
                <line x1="10" y1="11" x2="10" y2="17" />
                <line x1="14" y1="11" x2="14" y2="17" />
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

    <!-- Delete API Key Confirmation Modal -->
    <div v-if="showDeleteKeyModal" class="modal-overlay" @click="showDeleteKeyModal = false">
      <div class="modal-content" @click.stop>
        <h3>Delete API Key</h3>
        <p class="confirmation-message">
          Are you sure you want to delete the API key "{{ apiKeyToDelete?.label }}"?
          <br>
          This action cannot be undone.
        </p>
        <div class="modal-actions">
          <button class="cancel-btn" @click="showDeleteKeyModal = false">Cancel</button>
          <button 
            class="delete-confirm-btn" 
            @click="deleteApiKey"
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { User, ApiKey } from '@/types/music'

const user = ref<User>({
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

const showCreateKeyModal = ref(false)
const showDeleteKeyModal = ref(false)
const newKeyLabel = ref('')
const apiKeyToDelete = ref<ApiKey | null>(null)

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
    const newKey = {
      id: "string",
      label: "string",
      key: "string",
      createdAt: "string",
      lastUsed: "string"
    } // TODO await createApiKeyService(user.value.id, newKeyLabel.value)
    user.value.apiKeys.push(newKey)
    showCreateKeyModal.value = false
    newKeyLabel.value = ''
  } catch (error) {
    console.error('Error creating API key:', error)
  }
}

const confirmDeleteApiKey = (apiKey: ApiKey) => {
  apiKeyToDelete.value = apiKey
  showDeleteKeyModal.value = true
}

const deleteApiKey = () => {
  if (!apiKeyToDelete.value) return

  try {
    // Remove the API key from the user's apiKeys array
    user.value.apiKeys = user.value.apiKeys.filter(key => key.id !== apiKeyToDelete.value?.id)

    // Close the modal and reset the apiKeyToDelete
    showDeleteKeyModal.value = false
    apiKeyToDelete.value = null
  } catch (error) {
    console.error('Error deleting API key:', error)
  }
}

onMounted(async () => {
  try {
    user.value = {
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
    } //TODO await getUserProfile('1')
  } catch (error) {
    console.error('Error fetching user profile:', error)
  }
})
</script>

<style scoped>
.api-keys {
  padding-top: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.api-keys-section {
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

.delete-btn {
  padding: 4px;
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.delete-btn:hover {
  color: #e74c3c; /* Red color for delete action */
}

.icon {
  width: 16px;
  height: 16px;
}

.confirmation-message {
  margin: 16px 0;
  color: var(--text-color);
  line-height: 1.5;
}

.delete-confirm-btn {
  padding: 8px 16px;
  background: #e74c3c; /* Red color for delete action */
  border: none;
  border-radius: 4px;
  color: white;
  cursor: pointer;
  transition: all 0.2s;
}

.delete-confirm-btn:hover {
  background: #c0392b; /* Darker red on hover */
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
</style>
