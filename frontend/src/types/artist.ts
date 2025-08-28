import type { Album, Item, RecentTrack, Track } from ".";

export interface Artist extends Item {
    description?: string;
    last_listened_at: string;
}

export interface ArtistDetails extends Artist {
    topTracks: Track[];
    albums: Album[];
    recentListens: RecentTrack[];
    description?: string;
}
