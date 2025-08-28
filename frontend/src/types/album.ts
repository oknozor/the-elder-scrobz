import type { Item, PlayCount } from ".";

export interface Album extends Item {
    artist_id: string;
    artist_name: string;
    description?: string;
    last_listened_at: string;
    year: number;
}

export interface AlbumDetails extends Album {
    tracks: AlbumTrack[];
    musicbrainz_url?: string;
    release_date?: string;
    genres?: string[];
}

export interface AlbumTrack {
    mbid: string;
    subsonic_id?: string;
    name: string;
    artist_name?: string;
    number?: number;
    length?: number;
    total_playcount?: number;
    total_listen_duration?: number;
    playcount?: PlayCount[];
}
