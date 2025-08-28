import type { Item, Track } from ".";

export interface Album extends Item {
    artist_id: string;
    artist_name: string;
    description?: string;
    last_listened_at: string;
    year: number;
}

export interface AlbumDetails extends Album {
    tracks: Track[];
    release_date?: string;
    genres?: string[];
}
