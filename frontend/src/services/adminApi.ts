import apiClient from "./api";
import type { PaginatedResponse } from "@/types";
import type { User } from "@/types/user";

export interface AdminUser {
    username: string;
    email: string;
    role: string;
    lastActive: string;
    totalPlays: number;
}

export interface AdminStats {
    totalUsers: number;
    totalPlays: number;
    totalArtists: number;
    totalAlbums: number;
    totalTracks: number;
}

export interface AdminAction {
    id: string;
    action: string;
    targetUser?: string;
    performedBy: string;
    timestamp: string;
    details?: string;
}

class AdminApiService {
    /**
     * Get all users (admin only)
     */
    async getAllUsers(
        page: number = 1,
        perPage: number = 20,
    ): Promise<PaginatedResponse<AdminUser>> {
        try {
            const response = await apiClient.get("/users/admin", {
                params: {
                    page,
                    per_page: perPage,
                },
            });
            return response.data;
        } catch (error: any) {
            if (error.response?.status === 401) {
                throw new Error("Access denied: Admin privileges required");
            }
            throw error;
        }
    }

    /**
     * Get system statistics (admin only)
     */
    async getSystemStats(): Promise<AdminStats> {
        try {
            const response = await apiClient.get("/admin/stats");
            return response.data;
        } catch (error: any) {
            if (error.response?.status === 401) {
                throw new Error("Access denied: Admin privileges required");
            }
            throw error;
        }
    }

    /**
     * Get recent admin actions/audit log
     */
    async getAdminActions(
        page: number = 1,
        perPage: number = 50,
    ): Promise<PaginatedResponse<AdminAction>> {
        try {
            const response = await apiClient.get("/admin/actions", {
                params: {
                    page,
                    per_page: perPage,
                },
            });
            return response.data;
        } catch (error: any) {
            if (error.response?.status === 401) {
                throw new Error("Access denied: Admin privileges required");
            }
            throw error;
        }
    }

    /**
     * Delete a user (admin only)
     */
    async deleteUser(username: string): Promise<void> {
        try {
            await apiClient.delete(
                `/admin/users/${encodeURIComponent(username)}`,
            );
        } catch (error: any) {
            if (error.response?.status === 401) {
                throw new Error("Access denied: Admin privileges required");
            }
            if (error.response?.status === 404) {
                throw new Error("User not found");
            }
            throw error;
        }
    }

    /**
     * Update user role (admin only)
     */
    async updateUserRole(username: string, role: string): Promise<User> {
        try {
            const response = await apiClient.patch(
                `/admin/users/${encodeURIComponent(username)}/role`,
                {
                    role,
                },
            );
            return response.data;
        } catch (error: any) {
            if (error.response?.status === 401) {
                throw new Error("Access denied: Admin privileges required");
            }
            if (error.response?.status === 404) {
                throw new Error("User not found");
            }
            throw error;
        }
    }

    /**
     * Reset user's API keys (admin only)
     */
    async resetUserApiKeys(username: string): Promise<void> {
        try {
            await apiClient.post(
                `/admin/users/${encodeURIComponent(username)}/reset-api-keys`,
            );
        } catch (error: any) {
            if (error.response?.status === 401) {
                throw new Error("Access denied: Admin privileges required");
            }
            if (error.response?.status === 404) {
                throw new Error("User not found");
            }
            throw error;
        }
    }

    /**
     * Clear user's listening history (admin only)
     */
    async clearUserHistory(username: string): Promise<void> {
        try {
            await apiClient.delete(
                `/admin/users/${encodeURIComponent(username)}/history`,
            );
        } catch (error: any) {
            if (error.response?.status === 401) {
                throw new Error("Access denied: Admin privileges required");
            }
            if (error.response?.status === 404) {
                throw new Error("User not found");
            }
            throw error;
        }
    }

    /**
     * Get system health check (admin only)
     */
    async getSystemHealth(): Promise<{
        status: string;
        details: Record<string, any>;
    }> {
        try {
            const response = await apiClient.get("/admin/health");
            return response.data;
        } catch (error: any) {
            if (error.response?.status === 401) {
                throw new Error("Access denied: Admin privileges required");
            }
            throw error;
        }
    }

    /**
     * Trigger system maintenance tasks (admin only)
     */
    async triggerMaintenance(
        taskType: string,
    ): Promise<{ message: string; taskId?: string }> {
        try {
            const response = await apiClient.post("/admin/maintenance", {
                task_type: taskType,
            });
            return response.data;
        } catch (error: any) {
            if (error.response?.status === 401) {
                throw new Error("Access denied: Admin privileges required");
            }
            throw error;
        }
    }
}

export default new AdminApiService();
