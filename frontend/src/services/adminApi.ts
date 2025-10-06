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

class AdminApiService {}

export default new AdminApiService();
