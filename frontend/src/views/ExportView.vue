<template>
    <div class="import-page">
        <div class="import-section">
            <h1>Export Listening History</h1>
            <p class="description">
                Export your listen history in jsonl Musicbrainz format.
            </p>

            <div class="download-section">
                <button class="download-btn" @click="handleDownload">
                    <svg
                        class="download-icon"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                        <polyline points="7 10 12 15 17 10" />
                        <path d="M12 15V3" />
                    </svg>
                    Download Listening History
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import apiClient from "@/services/api";

const handleDownload = async () => {
    try {
        const response = await apiClient.get("/users/export", {
            responseType: "blob",
        });

        const blob = new Blob([response.data], {
            type: "application/x-ndjson",
        });
        const url = window.URL.createObjectURL(blob);
        const link = document.createElement("a");
        link.href = url;
        link.download = "listens.jsonl";
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        window.URL.revokeObjectURL(url);
    } catch (error) {
        console.error("Download failed:", error);
    }
};
</script>

<style scoped>
.import-page {
    padding: 20px;
    max-width: 1200px;
    margin: 0 auto;
}

.import-section {
    max-width: 600px;
    margin: 0 auto;
    padding: 40px;
    background: var(--card-background);
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
}

h1 {
    color: var(--text-color);
    margin: 0 0 8px 0;
    font-size: 1.8em;
}

.description {
    color: var(--text-secondary);
    margin: 0 0 32px 0;
    font-size: 1.1em;
}

.download-section {
    text-align: center;
}

.download-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 12px 24px;
    background: var(--primary-color);
    color: var(--background-color);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 1em;
    font-weight: 500;
    transition: all 0.2s;
}

.download-btn:hover {
    opacity: 0.9;
    transform: translateY(-1px);
}

.download-icon {
    width: 20px;
    height: 20px;
}
</style>
