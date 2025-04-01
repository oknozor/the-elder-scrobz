import { defineStore } from 'pinia';
import {
	PulseData,
	Track,
	RecentTrack,
	MusicStats,
	User,
	Album,
	Artist,
	PaginatedResponse,
} from '@/types/music';
import apiClient from '@/services/api';

export const useStatsStore = defineStore('stats', {
	state: () => ({
		users: [] as User[],
		pulses: [] as PulseData[],
		topAlbumsForStatsView: [] as Album[],
		topArtistsForStatsView: [] as Artist[],
		topTracksForStatsView: [] as Track[],
		recentTracks: {} as PaginatedResponse<RecentTrack>,
		topAlbums: {} as PaginatedResponse<Album>,
		topArtists: {} as PaginatedResponse<Artist>,
		topTracks: {} as PaginatedResponse<Track>,
		allMusicStatistics: {} as MusicStats,
		error: null as string | null,
		isLoading: false,
	}),

	actions: {
		/** Fetch all users */
		async fetchUsers() {
			try {
				this.isLoading = true;
				this.error = null;
				const { data } = await apiClient.get('/users');
				this.users = data;
			} catch (error) {
				this.error = 'Failed to fetch users';
				console.error('Error fetching users:', error);
			} finally {
				this.isLoading = false;
			}
		},

		// STATS VIEW:
		// Fetch top albums for stats view
		async fetchTopAlbumsForStatsView(
			username: string | null,
			period: string = 'week',
			page = 1,
			pageSize = 15
		): Promise<void> {
			const response = await this.fetchTopAlbums(
				username,
				period,
				page,
				pageSize
			);
			if (response) {
				this.topAlbumsForStatsView = response.data;
			}
		},
		// Fetch top artists for stats view
		async fetchTopArtistsForStatsView(
			username: string | null,
			period: string = 'week',
			page = 1,
			pageSize = 15
		): Promise<void> {
			const response = await this.fetchTopArtists(
				username,
				period,
				page,
				pageSize
			);
			if (response) {
				this.topArtistsForStatsView = response.data;
			}
		},
		// Fetch top tracks for stats view
		async fetchTopTracksForStatsView(
			username: string | null,
			period: string = 'week',
			page = 1,
			pageSize = 15
		): Promise<void> {
			const response = await this.fetchTopTracks(
				username,
				period,
				page,
				pageSize
			);
			if (response) {
				this.topTracksForStatsView = response.data;
			}
		},

		// ALBUMS VIEW:
		async fetchTopAlbumsForAlbumsView(
			username: string | null,
			period: string,
			page: number,
			pageSize: number
		): Promise<void> {
			const response: PaginatedResponse<Album> | undefined =
				await this.fetchTopAlbums(username, period, page, pageSize);
			if (response) {
				this.topAlbums = {
					...this.topAlbums,
					total: response.total,
					page: response.page,
					page_size: response.page_size,
					data: response.data,
				};
			}
		},
		// ARTISTS VIEW:
		async fetchTopArtistsForArtistsView(
			username: string | null,
			period: string,
			page: number,
			pageSize: number
		): Promise<void> {
			const response: PaginatedResponse<Artist> | undefined =
				await this.fetchTopArtists(username, period, page, pageSize);
			if (response) {
				this.topArtists = {
					...this.topArtists,
					total: response.total,
					page: response.page,
					page_size: response.page_size,
					data: response.data,
				};
			}
		},
		// TRACKS VIEW:
		async fetchTopTracksForTracksView(
			username: string | null,
			period: string = 'week',
			page: number,
			pageSize: number
		): Promise<void> {
			const response: PaginatedResponse<Track> | undefined =
				await this.fetchTopTracks(username, period, page, pageSize);
			if (response) {
				this.topTracks = {
					...this.topTracks,
					total: response.total,
					page: response.page,
					page_size: response.page_size,
					data: response.data,
				};
			}
		},

		// Fetch top albums
		async fetchTopAlbums(
			username: string | null,
			period: string = 'week',
			page: number,
			pageSize: number
		): Promise<PaginatedResponse<Album> | undefined> {
			const usernameParam = username ? `&username=${username}&` : '';
			try {
				this.error = null;
				return await apiClient
					.get<PaginatedResponse<Album>>(
						`/charts/albums?${usernameParam}period=${period}&page_size=${pageSize}&page=${page}`
					)
					.then((response) => response.data);
			} catch (error) {
				this.error = 'Failed to fetch albums';
				console.error('Error fetching albums:', error);
				return undefined;
			}
		},

		// Fetch top artists
		async fetchTopArtists(
			username: string | null,
			period: string = 'week',
			page = 1,
			pageSize = 15
		): Promise<PaginatedResponse<Artist> | undefined> {
			const usernameParam = username ? `&username=${username}&` : '';
			try {
				this.error = null;
				return await apiClient
					.get<PaginatedResponse<Artist>>(
						`/charts/artists?${usernameParam}period=${period}&page_size=${pageSize}&page=${page}`
					)
					.then((response) => response.data);
			} catch (error) {
				this.error = 'Failed to fetch artists';
				console.error('Error fetching artists:', error);
				return undefined;
			}
		},

		// Fetch top tracks
		async fetchTopTracks(
			username: string | null,
			period: string = 'week',
			page = 1,
			pageSize = 15
		): Promise<PaginatedResponse<Track> | undefined> {
			const usernameParam = username ? `&username=${username}&` : '';
			try {
				this.error = null;
				return await apiClient
					.get<PaginatedResponse<Track>>(
						`/charts/tracks?${usernameParam}period=${period}&page_size=${pageSize}&page=${page}`
					)
					.then((response) => response.data);
			} catch (error) {
				this.error = 'Failed to fetch tracks';
				console.error('Error fetching tracks:', error);
				return undefined;
			}
		},

		// Fetch pulses
		async fetchPulses(username: string | null, period: string = 'week') {
			const usernameParam = username ? `&username=${username}&` : '';
			try {
				this.error = null;
				const { data } = await apiClient.get(
					`/charts/pulses?${usernameParam}period=${period}`
				);
				this.pulses = data;
			} catch (error) {
				this.error = 'Failed to fetch pulse data';
				console.error('Error fetching pulses:', error);
			}
		},

		async fetchRecentTracks(
			username: string | null,
			page: number = 1,
			pageSize: number = 15
		) {
			const usernameParam = username ? `&username=${username}&` : '';
			try {
				this.error = null;
				const { data } = await apiClient.get(
					`/listens/recent?${usernameParam}page=${page}&page_size=${pageSize}`
				);
				// Will change when api is updated:
				this.recentTracks = {
					data: data,
					total: 100,
					page_size: pageSize,
					page: page,
				};
			} catch (error) {
				this.error = 'Failed to fetch recent tracks';
				console.error('Error fetching recent tracks:', error);
			}
		},
	},
});
