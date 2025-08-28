import type { Album, Artist, Track } from ".";

export interface User {
    id: string;
    name: string;
    imageUrl: string;
    lastActive: string;
    isAdmin?: boolean;
    apiKeys: ApiKey[];
    stats: {
        totalPlays: number;
        totalDuration: number;
        topArtists: Artist[];
        topAlbums: Album[];
        topTracks: Track[];
    };
}

export interface ApiKey {
    id: string;
    label: string;
    api_key: string;
    created_at: string;
    lastUsed?: string;
}
