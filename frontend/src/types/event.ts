export type ScrobzEvent = PlayingNow;

export type PlayingNow = {
    type: "PlayingNow";
    user: string;
    track_name: string;
    artist: string;
    album: string;
    cover_art_url?: string;
    track_duration: number;
};
