<template>
  <div class="admin-users-list">
    <div class="section-header">
      <h2>Users Management</h2>
      <p class="section-description">
        Manage user accounts, roles, and permissions
      </p>
    </div>

    <div class="users-controls">
      <div class="search-container">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search users..."
          class="search-input"
        />
        <svg class="search-icon" fill="currentColor" viewBox="0 0 20 20">
          <path
            fill-rule="evenodd"
            d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
            clip-rule="evenodd"
          />
        </svg>
      </div>

      <div class="view-toggle">
        <button
          :class="['toggle-btn', { active: viewMode === 'table' }]"
          @click="viewMode = 'table'"
        >
          Table
        </button>
        <button
          :class="['toggle-btn', { active: viewMode === 'cards' }]"
          @click="viewMode = 'cards'"
        >
          Cards
        </button>
      </div>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="loading-spinner">
        <svg class="animate-spin h-8 w-8" viewBox="0 0 24 24">
          <circle
            class="opacity-25"
            cx="12"
            cy="12"
            r="10"
            stroke="currentColor"
            stroke-width="4"
          />
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          />
        </svg>
        <span>Loading users...</span>
      </div>
    </div>

    <div v-else-if="error" class="error-state">
      <div class="error-message">
        <svg class="error-icon" fill="currentColor" viewBox="0 0 20 20">
          <path
            fill-rule="evenodd"
            d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
            clip-rule="evenodd"
          />
        </svg>
        <span>{{ error }}</span>
        <button @click="fetchUsers" class="retry-button">Retry</button>
      </div>
    </div>

    <div v-else>
      <!-- Table View -->
      <div v-if="viewMode === 'table'" class="users-table-container">
        <table class="users-table">
          <thead>
            <tr>
              <th>Username</th>
              <th>Email</th>
              <th>Role</th>
              <th>Last Active</th>
              <th>Total Plays</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="user in filteredUsers" :key="user.username" class="user-row">
              <td class="username-cell">
                <div class="user-info">
                  <div class="username">{{ user.username }}</div>
                </div>
              </td>
              <td>{{ user.email || 'N/A' }}</td>
              <td>
                <select
                  v-model="user.role"
                  @change="updateUserRole(user)"
                  class="role-select"
                  :class="{ 'role-admin': user.role === 'admin' }"
                >
                  <option value="user">User</option>
                  <option value="admin">Admin</option>
                </select>
              </td>
              <td>{{ formatDate(user.lastActive) }}</td>
              <td>{{ user.totalPlays?.toLocaleString() || '0' }}</td>
              <td>
                <div class="action-buttons">
                  <button
                    @click="resetUserApiKeys(user)"
                    class="action-btn reset-btn"
                    title="Reset API Keys"
                  >
                    <svg class="btn-icon" fill="currentColor" viewBox="0 0 20 20">
                      <path
                        fill-rule="evenodd"
                        d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z"
                        clip-rule="evenodd"
                      />
                    </svg>
                  </button>
                  <button
                    @click="clearUserHistory(user)"
                    class="action-btn clear-btn"
                    title="Clear History"
                  >
                    <svg class="btn-icon" fill="currentColor" viewBox="0 0 20 20">
                      <path
                        fill-rule="evenodd"
                        d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z M4 5a2 2 0 012-2v1a2 2 0 002 2h8a2 2 0 002-2V3a2 2 0 012 2v6a2 2 0 01-2 2H6a2 2 0 01-2-2V5zM8 11a1 1 0 012 0v2a1 1 0 11-2 0v-2zm4 0a1 1 0 112 0v2a1 1 0 11-2 0v-2z"
                        clip-rule="evenodd"
                      />
                    </svg>
                  </button>
                  <button
                    @click="confirmDeleteUser(user)"
                    class="action-btn delete-btn"
                    title="Delete User"
                  >
                    <svg class="btn-icon" fill="currentColor" viewBox="0 0 20 20">
                      <path
                        fill-rule="evenodd"
                        d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z M4 5a2 2 0 012-2v1a2 2 0 002 2h8a2 2 0 002-2V3a2 2 0 012 2v6a2 2 0 01-2 2H6a2 2 0 01-2-2V5zM8 11a1 1 0 012 0v2a1 1 0 11-2 0v-2zm4 0a1 1 0 112 0v2a1 1 0 11-2 0v-2z"
                        clip-rule="evenodd"
                      />
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Cards View -->
      <div v-else class="users-cards-container">
        <div class="users-grid">
          <div v-for="user in filteredUsers" :key="user.username" class="user-card">
            <div class="card-header">
              <h3 class="card-username">{{ user.username }}</h3>
              <select
                v-model="user.role"
                @change="updateUserRole(user)"
                class="card-role-select"
                :class="{ 'role-admin': user.role === 'admin' }"
              >
                <option value="user">User</option>
                <option value="admin">Admin</option>
              </select>
            </div>

            <div class="card-info">
              <div class="info-item">
                <span class="info-label">Email:</span>
                <span class="info-value">{{ user.email || 'N/A' }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Last Active:</span>
                <span class="info-value">{{ formatDate(user.lastActive) }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Total Plays:</span>
                <span class="info-value">{{ user.totalPlays?.toLocaleString() || '0' }}</span>
              </div>
            </div>

            <div class="card-actions">
              <button @click="resetUserApiKeys(user)" class="card-action-btn reset-btn">
                Reset API Keys
              </button>
              <button @click="clearUserHistory(user)" class="card-action-btn clear-btn">
                Clear History
              </button>
              <button @click="confirmDeleteUser(user)" class="card-action-btn delete-btn">
                Delete User
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Pagination -->
      <div v-if="pagination" class="pagination">
        <button
          @click="goToPage(pagination.current_page - 1)"
          :disabled="pagination.current_page <= 1"
          class="pagination-btn"
        >
          Previous
        </button>

        <div class="pagination-info">
          Page {{ pagination.current_page }} of {{ pagination.total_pages }}
          ({{ pagination.total }} users)
        </div>

        <button
          @click="goToPage(pagination.current_page + 1)"
          :disabled="pagination.current_page >= pagination.total_pages"
          class="pagination-btn"
        >
          Next
        </button>
      </div>
    </div>

    <!-- Confirmation Modal -->
    <div v-if="showDeleteConfirm" class="modal-overlay" @click="cancelDelete">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>Confirm Delete</h3>
        </div>
        <div class="modal-body">
          <p>
            Are you sure you want to delete user
            <strong>{{ userToDelete?.username }}</strong>?
          </p>
          <p class="warning-text">
            This action cannot be undone and will permanently delete all user data.
          </p>
        </div>
        <div class="modal-actions">
          <button @click="cancelDelete" class="modal-btn cancel-btn">
            Cancel
          </button>
          <button @click="deleteUser" class="modal-btn confirm-btn">
            Delete User
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import adminApi, { type AdminUser } from "@/services/adminApi";
import type { PaginatedResponse } from "@/types";

// Reactive data
const users = ref<AdminUser[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);
const searchQuery = ref("");
const viewMode = ref<"table" | "cards">("table");
const pagination = ref<PaginatedResponse<AdminUser>["pagination"] | null>(null);
const currentPage = ref(1);
const showDeleteConfirm = ref(false);
const userToDelete = ref<AdminUser | null>(null);

// Computed properties
const filteredUsers = computed(() => {
    if (!searchQuery.value) return users.value;

    const query = searchQuery.value.toLowerCase();
    return users.value.filter(
        (user) =>
            user.username.toLowerCase().includes(query) ||
            user.email?.toLowerCase().includes(query) ||
            user.role.toLowerCase().includes(query),
    );
});

// Methods
const fetchUsers = async (page: number = 1) => {
    loading.value = true;
    error.value = null;

    try {
        const response = await adminApi.getAllUsers(page, 20);
        users.value = response.data;
        pagination.value = response.pagination;
        currentPage.value = page;
    } catch (err: any) {
        error.value = err.message || "Failed to fetch users";
        console.error("Error fetching users:", err);
    } finally {
        loading.value = false;
    }
};

const updateUserRole = async (user: AdminUser) => {
    try {
        await adminApi.updateUserRole(user.username, user.role);
        // Refresh the users list to get updated data
        await fetchUsers(currentPage.value);
    } catch (err: any) {
        error.value = err.message || "Failed to update user role";
        console.error("Error updating user role:", err);
    }
};

const resetUserApiKeys = async (user: AdminUser) => {
    try {
        await adminApi.resetUserApiKeys(user.username);
        alert(`API keys reset for user ${user.username}`);
    } catch (err: any) {
        error.value = err.message || "Failed to reset API keys";
        console.error("Error resetting API keys:", err);
    }
};

const clearUserHistory = async (user: AdminUser) => {
    if (
        !confirm(
            `Are you sure you want to clear all listening history for ${user.username}?`,
        )
    ) {
        return;
    }

    try {
        await adminApi.clearUserHistory(user.username);
        alert(`History cleared for user ${user.username}`);
        // Refresh to update play counts
        await fetchUsers(currentPage.value);
    } catch (err: any) {
        error.value = err.message || "Failed to clear user history";
        console.error("Error clearing user history:", err);
    }
};

const confirmDeleteUser = (user: AdminUser) => {
    userToDelete.value = user;
    showDeleteConfirm.value = true;
};

const cancelDelete = () => {
    userToDelete.value = null;
    showDeleteConfirm.value = false;
};

const deleteUser = async () => {
    if (!userToDelete.value) return;

    try {
        await adminApi.deleteUser(userToDelete.value.username);
        await fetchUsers(currentPage.value);
        showDeleteConfirm.value = false;
        userToDelete.value = null;
    } catch (err: any) {
        error.value = err.message || "Failed to delete user";
        console.error("Error deleting user:", err);
    }
};

const goToPage = (page: number) => {
    if (page >= 1 && pagination.value && page <= pagination.value.total_pages) {
        fetchUsers(page);
    }
};

const formatDate = (dateString: string) => {
    if (!dateString) return "Never";
    try {
        return new Date(dateString).toLocaleDateString();
    } catch {
        return "Invalid date";
    }
};

// Watchers
watch(searchQuery, () => {
    // Reset to first page when searching
    if (searchQuery.value) {
        currentPage.value = 1;
    }
});

// Lifecycle
onMounted(() => {
    fetchUsers();
});
</script>

<style scoped>
.admin-users-list {
  max-width: 100%;
}

.section-header {
  margin-bottom: 32px;
  text-align: center;
}

.section-header h2 {
  color: var(--text-color);
  margin: 0 0 8px 0;
  font-size: 1.6em;
  font-weight: 600;
}

.section-description {
  color: var(--text-secondary);
  margin: 0;
  font-size: 1em;
}

.users-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  gap: 16px;
}

.search-container {
  position: relative;
  flex: 1;
  max-width: 400px;
}

.search-input {
  width: 100%;
  padding: 12px 16px 12px 40px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--input-background);
  color: var(--text-color);
  font-size: 14px;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 3px rgba(var(--accent-color-rgb), 0.1);
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  width: 16px;
  height: 16px;
  color: var(--text-secondary);
}

.view-toggle {
  display: flex;
  border-radius: 6px;
  overflow: hidden;
  border: 1px solid var(--border-color);
}

.toggle-btn {
  padding: 8px 16px;
  background: var(--card-background);
  color: var(--text-secondary);
  border: none;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.toggle-btn:hover {
  background: var(--hover-color);
}

.toggle-btn.active {
  background: var(--accent-color);
  color: white;
}

.loading-state,
.error-state {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 200px;
}

.loading-spinner {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  color: var(--text-secondary);
}

.loading-spinner svg {
  width: 32px;
  height: 32px;
}

.error-message {
  display: flex;
  align-items: center;
  gap: 12px;
  color: var(--error-color);
  background: var(--error-background);
  padding: 16px 24px;
  border-radius: 8px;
  border: 1px solid var(--error-border);
}

.error-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.retry-button {
  padding: 6px 12px;
  background: var(--accent-color);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

/* Table Styles */
.users-table-container {
  overflow-x: auto;
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.users-table {
  width: 100%;
  border-collapse: collapse;
  background: var(--card-background);
}

.users-table th {
  background: var(--header-background);
  padding: 16px;
  text-align: left;
  font-weight: 600;
  color: var(--text-color);
  border-bottom: 1px solid var(--border-color);
  white-space: nowrap;
}

.users-table td {
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
  color: var(--text-color);
}

.user-row:hover {
  background: var(--hover-color);
}

.username-cell {
  font-weight: 500;
}

.role-select {
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--input-background);
  color: var(--text-color);
  font-size: 12px;
}

.role-select.role-admin {
  background: var(--error-background);
  color: var(--error-color);
  border-color: var(--error-border);
}

.action-buttons {
  display: flex;
  gap: 8px;
}

.action-btn {
  padding: 6px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-icon {
  width: 14px;
  height: 14px;
}

.reset-btn {
  background: var(--warning-background);
  color: var(--warning-color);
}

.reset-btn:hover {
  background: var(--warning-color);
  color: white;
}

.clear-btn {
  background: var(--info-background);
  color: var(--info-color);
}

.clear-btn:hover {
  background: var(--info-color);
  color: white;
}

.delete-btn {
  background: var(--error-background);
  color: var(--error-color);
}

.delete-btn:hover {
  background: var(--error-color);
  color: white;
}

/* Cards Styles */
.users-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 24px;
}

.user-card {
  background: var(--card-background);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 24px;
  transition: all 0.2s;
}

.user-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  border-color: var(--accent-color);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.card-username {
  font-size: 1.1em;
  font-weight: 600;
  color: var(--text-color);
  margin: 0;
}

.card-role-select {
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--input-background);
  color: var(--text-color);
  font-size: 12px;
}

.card-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 20px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.info-label {
  font-weight: 500;
  color: var(--text-secondary);
  font-size: 13px;
}

.info-value {
  color: var(--text-color);
  font-size: 13px;
}

.card-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.card-action-btn {
  flex: 1;
  padding: 8px 12px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s;
  min-width: 80px;
}

/* Pagination */
.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 16px;
  margin-top: 32px;
}

.pagination-btn {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  background: var(--card-background);
  color: var(--text-color);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.pagination-btn:hover:not(:disabled) {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.pagination-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.pagination-info {
  color: var(--text-secondary);
  font-size: 14px;
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: var(--card-background);
  border-radius: 8px;
  padding: 0;
  max-width: 400px;
  width: 90vw;
  border: 1px solid var(--border-color);
}

.modal-header {
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 {
  margin: 0;
  color: var(--text-color);
  font-size: 1.2em;
}

.modal-body {
  padding: 20px 24px;
}

.modal-body p {
  margin: 0 0 12px 0;
  color: var(--text-color);
}

.warning-text {
  color: var(--error-color);
  font-size: 13px;
}

.modal-actions {
  display: flex;
  gap: 12px;
  padding: 20px 24px;
  border-top: 1px solid var(--border-color);
  justify-content: flex-end;
}

.modal-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.cancel-btn {
  background: var(--card-background);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}

.cancel-btn:hover {
  background: var(--hover-color);
}

.confirm-btn {
  background: var(--error-color);
  color: white;
}

.confirm-btn:hover {
  background: var(--error-hover);
}

/* Responsive */
@media (max-width: 768px) {
  .users-controls {
    flex-direction: column;
    align-items: stretch;
  }

  .search-container {
    max-width: none;
  }

  .view-toggle {
    align-self: center;
  }

  .users-table-container {
    font-size: 12px;
  }

  .users-table th,
  .users-table td {
    padding: 8px;
  }

  .users-grid {
    grid-template-columns: 1fr;
  }

  .modal-content {
    margin: 20px;
  }
}
</style>
