export type TimeRange = 'today' | 'week' | 'month' | 'year' | 'all';
export type PulseTimeRange = '12days' | '12weeks' | '12months' | '12years';

export interface PaginatedResponse<T> {
	data: T[];
	total: number;
	page: number;
	page_size: number;
}

export interface Artist {
	artist_id: string;
	artist_name: string;
	thumbnail_url: string;
	last_listened_at: string;
	listens: number;
}

export interface Track {
	track_id: string;
	track_name: string;
	release_name: string;
	artist_name: string;
	cover_art_url: string;
	listens: number;
	track_length: number; // in minutes
}

export interface Album {
	release_id: string;
	release_name: string;
	artist_name: string;
	cover_art_url: string;
	last_listened_at: string;
	listens: number;
}
export interface ArtistDetails extends Artist {
	topTracks: Track[];
	albums: Album[];
	recentListens: RecentTrack[];
	description?: string;
}
export interface AlbumDetails extends Album {
	topTracks: Track[];
}

export interface RecentTrack {
	id: string;
	track_name: string;
	artist_name: string;
	cover_art_url: string;
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
