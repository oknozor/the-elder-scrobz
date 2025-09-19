<template>
    <table class="tracks-table">
        <thead>
            <tr class="track-header-row">
                <th>#</th>
                <th>Track</th>
                <th>Duration</th>
                <th>Top Plays</th>
                <th>Total Plays</th>
            </tr>
        </thead>
        <tbody>
            <tr
                v-for="track in props.tracks"
                :key="track.mbid"
                class="track-row"
            >
                <td class="track-number-column">
                    {{ track.number }}
                </td>
                <td class="track-name-column">
                    <span class="track-name">{{ track.name }}</span>
                    <span v-if="track.artist_name" class="track-artist">
                        - {{ track.artist_name }}</span
                    >
                </td>
                <td class="track-duration-column">
                    <span>{{
                        track.length ? formatTrackLength(track.length) : "N/A"
                    }}</span>
                </td>
                <td class="track-playcount-column">
                    <div v-if="track.playcount && track.playcount.length">
                        <span
                            v-for="(pc, idx) in track.playcount"
                            :key="idx"
                            class="playcount-item"
                        >
                            {{ pc.username }} ({{ pc.count }})
                            <span v-if="idx < track.playcount.length - 1"
                                >,
                            </span>
                        </span>
                    </div>
                    <span v-else>N/A</span>
                </td>
                <td class="track-total-plays-column">
                    {{ track.total_playcount }}
                </td>
            </tr>
        </tbody>
    </table>
</template>
<script setup lang="ts">
import { AlbumTrack } from "@/types";
import { formatTrackLength } from "@/utils/formatter";

const props = defineProps({
    tracks: {
        type: Array<AlbumTrack>,
        required: true,
    },
});
</script>
<style scoped>
.tracks-table {
    width: 100%;
    background: var(--card-background);
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
    table-layout: fixed;
}

.track-row {
    display: grid;
    grid-template-columns: 60px 1fr 100px 120px;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
    min-height: 48px;
    will-change: background-color;
}

.track-header-row {
    display: flex;
    flex-direction: row;
    font-weight: 600;
    border-bottom: 1px solid var(--border-color);
    justify-content: space-between;
}

.track-number-column {
    color: var(--text-secondary);
    font-size: 0.9em;
    text-align: center;
    font-weight: bold;
}

.track-name-column {
    color: var(--text-color);
    font-size: 0.9em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.track-duration-column {
    color: var(--text-secondary);
    font-size: 0.9em;
    text-align: right;
}

.track-playcount-column {
    color: var(--text-secondary);
    font-size: 0.9em;
    text-align: right;
}
</style>
