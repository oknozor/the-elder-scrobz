import { defineStore } from 'pinia';
import axios from 'axios';
import {
	PulseData,
	Track,
	RecentTrack,
	MusicStats,
	User,
	Album,
	Artist,
} from '@/types/music';

const API_BASE_URL = 'http://localhost:8000';

export const useStatsStore = defineStore('stats', {
	state: () => ({
		users: [] as User[],
		pulses: [] as PulseData[],
		albums: [] as Album[],
		artists: [] as Artist[],
		tracks: [] as Track[],
		recentTracks: [] as RecentTrack[],
		allMusicStatistics: {} as MusicStats,
		sse: null as EventSource | null,
		error: null as string | null,
		isLoading: false,
	}),

	actions: {
		/** Fetch all users */
		async fetchUsers() {
			try {
				this.isLoading = true;
				this.error = null;
				const { data } = await axios.get(
					`${API_BASE_URL}/api/v1/users`
				);
				this.users = data;
			} catch (error) {
				this.error = 'Failed to fetch users';
				console.error('Error fetching users:', error);
			} finally {
				this.isLoading = false;
			}
		},

		/** Fetch pulses (play count over time) */
		async fetchPulses(username: string, mode: string = 'years') {
			try {
				this.error = null;
				const { data } = await axios.get(
					`${API_BASE_URL}/api/v1/charts/pulses?username=${username}&mode=${mode}`
				);
				this.pulses = data;
			} catch (error) {
				this.error = 'Failed to fetch pulse data';
				console.error('Error fetching pulses:', error);
			}
		},

		/** Fetch top albums */
		async fetchAlbums(
			username: string,
			period: string = 'this-month',
			page = 1,
			pageSize = 15
		) {
			try {
				this.error = null;
				const { data } = await axios.get(
					`${API_BASE_URL}/api/v1/charts/albums?username=${username}&period=${period}&page_size=${pageSize}&page=${page}`
				);
				this.albums = data;
			} catch (error) {
				this.error = 'Failed to fetch albums';
				console.error('Error fetching albums:', error);
			}
		},

		/** Fetch top artists */
		async fetchArtists(
			username: string,
			period: string = 'this-month',
			page = 1,
			pageSize = 15
		) {
			try {
				this.error = null;
				const { data } = await axios.get(
					`${API_BASE_URL}/api/v1/charts/artists?username=${username}&period=${period}&page_size=${pageSize}&page=${page}`
				);
				this.artists = data;
			} catch (error) {
				this.error = 'Failed to fetch artists';
				console.error('Error fetching artists:', error);
			}
		},

		/** Fetch top tracks */
		async fetchTracks(
			username: string,
			period: string = 'this-month',
			page = 1,
			pageSize = 15
		) {
			try {
				this.error = null;
				const { data } = await axios.get(
					`${API_BASE_URL}/api/v1/charts/tracks?username=${username}&period=${period}&page_size=${pageSize}&page=${page}`
				);
				this.tracks = data;
			} catch (error) {
				this.error = 'Failed to fetch tracks';
				console.error('Error fetching tracks:', error);
			}
		},

		/** Fetch recent listens */
		async fetchRecentTracks(page = 1, pageSize = 20) {
			try {
				this.error = null;
				const { data } = await axios.get(
					`${API_BASE_URL}/api/v1/listens/recent?page_size=${pageSize}&page=${page}`
				);
				this.recentTracks = data;
			} catch (error) {
				this.error = 'Failed to fetch recent tracks';
				console.error('Error fetching recent tracks:', error);
			}
		},

		/** Calculate time statistics from recent tracks */
		calculateTimeStats() {
			const now = new Date();
			const today = new Date(
				now.getFullYear(),
				now.getMonth(),
				now.getDate()
			);
			const weekStart = new Date(today);
			weekStart.setDate(today.getDate() - today.getDay());
			const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
			const yearStart = new Date(now.getFullYear(), 0, 1);

			const stats = {
				today: { playCount: 0, duration: 0 },
				week: { playCount: 0, duration: 0 },
				month: { playCount: 0, duration: 0 },
				year: { playCount: 0, duration: 0 },
				all: { playCount: 0, duration: 0 },
			};

			this.recentTracks.forEach((track) => {
				const playedAt = new Date(track.playedAt);
				const duration = track.duration || 0;

				// All time stats
				stats.all.playCount++;
				stats.all.duration += duration;

				// Year stats
				if (playedAt >= yearStart) {
					stats.year.playCount++;
					stats.year.duration += duration;
				}

				// Month stats
				if (playedAt >= monthStart) {
					stats.month.playCount++;
					stats.month.duration += duration;
				}

				// Week stats
				if (playedAt >= weekStart) {
					stats.week.playCount++;
					stats.week.duration += duration;
				}

				// Today stats
				if (playedAt >= today) {
					stats.today.playCount++;
					stats.today.duration += duration;
				}
			});

			return stats;
		},

		/** Start SSE connection for real-time updates */
		startSSE() {
			if (this.sse) {
				this.sse.close();
			}

			this.sse = new EventSource(`${API_BASE_URL}/api/v1/listens/stream`);

			this.sse.onmessage = (event) => {
				const newTrack = JSON.parse(event.data);
				this.recentTracks.unshift(newTrack);
				// Keep only the last 100 tracks
				if (this.recentTracks.length > 100) {
					this.recentTracks.pop();
				}
				// Update time stats
				this.allMusicStatistics.timeStats = this.calculateTimeStats();
			};

			this.sse.onerror = (error) => {
				console.error('SSE Error:', error);
				this.error = 'Real-time updates connection failed';
			};
		},

		/** Stop SSE connection */
		stopSSE() {
			if (this.sse) {
				this.sse.close();
				this.sse = null;
			}
		},

		/** Fetch all music statistics */
		async fetchAllMusicStatistics(
			username: string,
			period: string,
			mode: string,
			page: number
		) {
			try {
				this.isLoading = true;
				this.error = null;

				await Promise.all([
					this.fetchAlbums(username, period),
					this.fetchArtists(username, period),
					this.fetchTracks(username, period),
					this.fetchPulses(username, mode),
					this.fetchRecentTracks(page),
				]);

				this.allMusicStatistics = {
					topAlbums: this.albums,
					topArtists: this.artists,
					topTracks: this.tracks,
					recentTracks: this.recentTracks,
					pulseData: this.pulses,
					timeStats: this.calculateTimeStats(),
				};

				// Start real-time updates after initial data is loaded
				this.startSSE();
			} catch (error) {
				this.error = 'Failed to fetch music statistics';
				console.error('Error fetching all music statistics:', error);
			} finally {
				this.isLoading = false;
			}
		},
	},
});
