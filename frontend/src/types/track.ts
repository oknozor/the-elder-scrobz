import type { Item } from ".";

export interface Track extends Item {
    release_name: string;
    artist_display_name: string;
    length: number;
    number: number;
}

export interface RecentTrack extends Item {
    artist_id: string;
    artist_name: string;
    listened_at: string;
    user: string;
    duration: number;
}
