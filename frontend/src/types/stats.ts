import type { RecentTrack } from "./track";

export interface TimePeriodStats {
    playCount: number;
    duration: number; // in minutes
}

export interface PulseData {
    period: string;
    listens: number;
}

export interface MusicStats {
    recentTracks: RecentTrack[];
    timeStats: {
        today: TimePeriodStats;
        week: TimePeriodStats;
        month: TimePeriodStats;
        year: TimePeriodStats;
        all: TimePeriodStats;
    };
}

export interface Overview {
    artist_listened: number;
    track_listened: number;
    time_listened: number;
    artist_listened_percentage_increase?: number;
    track_listened_percentage_increase?: number;
    time_listened_percentage_increase?: number;
}
