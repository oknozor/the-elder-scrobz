import { defineStore } from 'pinia';
import {
	PulseData,
	Track,
	RecentTrack,
	MusicStats,
	User,
	Album,
	Artist,
} from '@/types/music';
import apiClient from '@/services/api';

export const useStatsStore = defineStore('stats', {
	state: () => ({
		users: [] as User[],
		pulses: [] as PulseData[],
		topAlbums: [] as Album[],
		topArtists: [] as Artist[],
		topTracks: [] as Track[],
		recentTracks: [] as RecentTrack[],
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

		/** Fetch top albums */
		async fetchTopAlbums(
			username: string | null,
			period: string = 'week',
			page = 1,
			pageSize = 15
		) {
			const usernameParam = username ? `&username=${username}&` : '';
			try {
				this.error = null;
				const { data } = await apiClient.get(
					`/charts/albums?${usernameParam}period=${period}&page_size=${pageSize}&page=${page}`
				);
				this.topAlbums = data;
			} catch (error) {
				this.error = 'Failed to fetch albums';
				console.error('Error fetching albums:', error);
			}
		},

		/** Fetch top artists */
		async fetchTopArtists(
			username: string | null,
			period: string = 'week',
			page = 1,
			pageSize = 15
		) {
			const usernameParam = username ? `&username=${username}&` : '';
			try {
				this.error = null;
				const { data } = await apiClient.get(
					`/charts/artists?${usernameParam}period=${period}&page_size=${pageSize}&page=${page}`
				);
				this.topArtists = data;
			} catch (error) {
				this.error = 'Failed to fetch artists';
				console.error('Error fetching artists:', error);
			}
		},

		/** Fetch top tracks */
		async fetchTopTracks(
			username: string | null,
			period: string = 'week',
			page = 1,
			pageSize = 15
		) {
			const usernameParam = username ? `&username=${username}&` : '';
			try {
				this.error = null;
				const { data } = await apiClient.get(
					`/charts/tracks?${usernameParam}period=${period}&page_size=${pageSize}&page=${page}`
				);
				this.topTracks = data;
			} catch (error) {
				this.error = 'Failed to fetch tracks';
				console.error('Error fetching tracks:', error);
			}
		},

		/** Fetch pulses */
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
	},
});
