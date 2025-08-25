export type TimeRange = "today" | "week" | "month" | "year" | "all";

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  page_size: number;
}

export interface Item {
  type: string;
  id: string;
  name: string;
  thumbnail_url: string;
  subsonic_url?: string;
  listens?: number;
}

export interface Artist extends Item {
  description?: string;
  last_listened_at: string;
}

export interface Track extends Item {
  release_name: string;
  artist_display_name: string;
  length: number;
  number: number;
}

export interface Album extends Item {
  artist_id: string;
  artist_name: string;
  description?: string;
  last_listened_at: string;
  year: number;
}

export interface ArtistDetails extends Artist {
  topTracks: Track[];
  albums: Album[];
  recentListens: RecentTrack[];
  description?: string;
}

export interface AlbumDetails extends Album {
  tracks: Track[];
  release_date?: string;
  genres?: string[];
}

export interface RecentTrack extends Item {
  artist_id: string;
  artist_name: string;
  listened_at: string;
  user: string;
  duration: number;
}

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

export interface ApiKey {
  id: string;
  label: string;
  api_key: string;
  created_at: string;
  lastUsed?: string;
}

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
