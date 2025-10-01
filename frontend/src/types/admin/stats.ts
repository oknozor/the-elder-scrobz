export interface Stats {
    total_raw_scrobble_count: number;
    total_scrobble_count: number;
    total_track_count: number;
    total_releases_count: number;
    total_artists_count: number;
    unparsable_scrobbles: MissingEntities;
    releases_without_coverart: MissingEntities;
    releases_without_subsonic_id: MissingEntities;
    artists_without_thumbnail: MissingEntities;
    artists_without_subsonic_id: MissingEntities;
    tracks_without_subsonic_id: MissingEntities;
}

export interface MissingEntities {
    ids: string[];
    count: number;
}
