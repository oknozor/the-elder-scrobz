export type TimeRange = 'today' | 'week' | 'month' | 'year' | 'all';
export type PulseTimeRange = '12days' | '12weeks' | '12months' | '12years';

export interface Artist {
  id: string;
  name: string;
  imageUrl: string;
  playCount: number;
  duration: number; // in minutes
}

export interface ArtistDetails extends Artist {
  topTracks: Track[];
  albums: Album[];
  recentListens: RecentTrack[];
}

export interface Track {
  id: string;
  title: string;
  artist: string;
  album?: string;  // Make it optional since not all tracks might have an album
  imageUrl: string;
  playCount: number;
  duration: number; // in minutes
}

export interface Album {
  id: string;
  title: string;
  artist: string;
  imageUrl: string;
  playCount: number;
  duration: number; // in minutes
}

export interface AlbumDetails extends Album {
  topTracks: Track[];
}

export interface RecentTrack {
  id: string;
  title: string;
  artist: string;
  imageUrl: string;
  lastPlayed: string;
  user: string;
}

export interface TimePeriodStats {
  playCount: number;
  duration: number; // in minutes
}

export interface PulseData {
  period: string;
  playCount: number;
}

export interface MusicStats {
  topArtists: Artist[];
  topTracks: Track[];
  topAlbums: Album[];
  recentTracks: RecentTrack[];
  timeStats: {
    today: TimePeriodStats;
    week: TimePeriodStats;
    month: TimePeriodStats;
    year: TimePeriodStats;
    all: TimePeriodStats;
  };
  pulseData: PulseData[];
}

export interface ApiKey {
  id: string;
  label: string;
  key: string;
  createdAt: string;
  lastUsed?: string;
}

export interface User {
  id: string;
  name: string;
  imageUrl: string;
  lastActive: string;
  apiKeys: ApiKey[];
  stats: {
    totalPlays: number;
    totalDuration: number;
    topArtists: Artist[];
    topAlbums: Album[];
    topTracks: Track[];
  };
} 
