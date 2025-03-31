import { defineStore } from "pinia";
import {
  PulseData,
  Track,
  RecentTrack,
  MusicStats,
  User,
  Album,
  Artist,
  Page,
  PaginatedResponse,
} from "@/types/music";
import apiClient from "@/services/api";

export const useStatsStore = defineStore("stats", {
  state: () => ({
    users: [] as User[],
    pulses: [] as PulseData[],
    topAlbums: [] as Album[],
    topArtists: [] as Artist[],
    topTracks: [] as Track[],
    recentTracks: {} as Page<RecentTrack>,
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
        const { data } = await apiClient.get("/users");
        this.users = data;
      } catch (error) {
        this.error = "Failed to fetch users";
        console.error("Error fetching users:", error);
      } finally {
        this.isLoading = false;
      }
    },

    /** Fetch top albums */
    async fetchTopAlbums(
      username: string | null,
      period: string = "week",
      page = 1,
      pageSize = 15,
    ) {
      const usernameParam = username ? `&username=${username}&` : "";
      try {
        this.error = null;
        const { data } = await apiClient
          .get<
            PaginatedResponse<Album>
          >(`/charts/albums?${usernameParam}period=${period}&page_size=${pageSize}&page=${page}`)
          .then((response) => response.data);
        this.topAlbums = data;
      } catch (error) {
        this.error = "Failed to fetch albums";
        console.error("Error fetching albums:", error);
      }
    },

    /** Fetch top artists */
    async fetchTopArtists(
      username: string | null,
      period: string = "week",
      page = 1,
      pageSize = 15,
    ) {
      const usernameParam = username ? `&username=${username}&` : "";
      try {
        this.error = null;
        const { data } = await apiClient
          .get<
            PaginatedResponse<Artist>
          >(`/charts/artists?${usernameParam}period=${period}&page_size=${pageSize}&page=${page}`)
          .then((response) => response.data);
        this.topArtists = data;
      } catch (error) {
        this.error = "Failed to fetch artists";
        console.error("Error fetching artists:", error);
      }
    },

    /** Fetch top tracks */
    async fetchTopTracks(
      username: string | null,
      period: string = "week",
      page = 1,
      pageSize = 15,
    ) {
      const usernameParam = username ? `&username=${username}&` : "";
      try {
        this.error = null;
        const { data } = await apiClient
          .get<
            PaginatedResponse<Track>
          >(`/charts/tracks?${usernameParam}period=${period}&page_size=${pageSize}&page=${page}`)
          .then((response) => response.data);
        this.topTracks = data;
      } catch (error) {
        this.error = "Failed to fetch tracks";
        console.error("Error fetching tracks:", error);
      }
    },

    /** Fetch pulses */
    async fetchPulses(username: string | null, period: string = "week") {
      const usernameParam = username ? `&username=${username}&` : "";
      try {
        this.error = null;
        const { data } = await apiClient.get(
          `/charts/pulses?${usernameParam}period=${period}`,
        );
        this.pulses = data;
      } catch (error) {
        this.error = "Failed to fetch pulse data";
        console.error("Error fetching pulses:", error);
      }
    },

    async fetchRecentTracks(
      username: string | null,
      page: number = 1,
      pageSize: number = 20,
    ) {
      const usernameParam = username ? `&username=${username}&` : "";
      try {
        this.error = null;
        const { data } = await apiClient.get(
          `/listens/recent?${usernameParam}page=${page}&page_size=${pageSize}`,
        );
        // Will change when api is updated:
        this.recentTracks = {
          content: data,
          totalElements: 100,
          totalPages: 10,
          pageSize: pageSize,
          page: page,
        };
      } catch (error) {
        this.error = "Failed to fetch recent tracks";
        console.error("Error fetching recent tracks:", error);
      }
    },
  },
});
