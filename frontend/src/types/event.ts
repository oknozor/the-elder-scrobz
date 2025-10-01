export type PlayingNow = {
    type: "PlayingNow";
    user: string;
    track_name: string;
    artist: string;
    album: string;
    cover_art_url?: string;
};

export type ScrobzEvent = PlayingNow;
