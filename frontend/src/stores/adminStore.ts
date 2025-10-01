import { defineStore } from "pinia";
import apiClient from "@/services/api";
import type { Stats } from "@/types";

export interface ErroredScrobble {
    id: number;
    user_id: string;
    data: unknown;
    created_at: string;
}

export interface RawScrobble {
    id: string;
    user_id: string;
    listened_at: string;
    data: unknown;
    status: "unprocessed" | "processed";
}

export interface ScanParams {
    force?: boolean;
    artists?: boolean;
    releases?: boolean;
    scrobbles?: boolean;
}

export const useAdminStore = defineStore("admin", {
    state: () => ({
        stats: {} as Stats,
        erroredScrobbles: [] as ErroredScrobble[],
        isScanning: false,
    }),

    actions: {
        async fetchStats() {
            try {
                const response = await apiClient.get("/admin/stats");
                this.stats = response.data;
            } catch (error) {
                console.error("Error fetching stats:", error);
                throw error;
            }
        },

        async removeCoverart(releaseId: string) {
            try {
                const response = await apiClient.patch(
                    `/admin/releases/${releaseId}/remove-coverart`,
                );
                return response.data;
            } catch (error) {
                console.error("Error removing coverart:", error);
                throw error;
            }
        },

        async scanDatabase(params: ScanParams = {}) {
            try {
                this.isScanning = true;
                const queryParams = new URLSearchParams();

                if (params.force !== undefined) {
                    queryParams.append("force", params.force.toString());
                }
                if (params.artists !== undefined) {
                    queryParams.append("artists", params.artists.toString());
                }
                if (params.releases !== undefined) {
                    queryParams.append("releases", params.releases.toString());
                }
                if (params.scrobbles !== undefined) {
                    queryParams.append(
                        "scrobbles",
                        params.scrobbles.toString(),
                    );
                }

                const url = `/admin/scan${queryParams.toString() ? `?${queryParams.toString()}` : ""}`;
                const response = await apiClient.post(url);
                return response.data;
            } catch (error) {
                console.error("Error scanning database:", error);
                throw error;
            } finally {
                this.isScanning = false;
            }
        },

        async fetchErroredScrobbles(page: number = 1, per_page: number = 20) {
            try {
                const queryParams = new URLSearchParams();
                queryParams.append("page", page.toString());
                queryParams.append("per_page", per_page.toString());

                const url = `/admin/scrobbles/dlc?${queryParams.toString()}`;
                const response = await apiClient.get(url);
                this.erroredScrobbles = response.data;
                return response.data;
            } catch (error) {
                console.error("Error fetching errored scrobbles:", error);
                throw error;
            }
        },

        async fetchScrobbleById(scrobbleId: string): Promise<RawScrobble> {
            try {
                const response = await apiClient.get(
                    `/admin/scrobbles/${scrobbleId}`,
                );
                return response.data;
            } catch (error) {
                console.error("Error fetching scrobble by ID:", error);
                throw error;
            }
        },
    },
});
