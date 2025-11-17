<template>
    <div class="scan-form">
        <div class="form-header">
            <h2>Database Scan</h2>
            <p class="form-description">
                Configure and trigger a database scan to update metadata and
                process scrobbles.
            </p>
        </div>

        <form @submit.prevent="handleSubmit" class="scan-form-content">
            <div class="form-section">
                <h3>Scan Options</h3>
                <div class="checkbox-group">
                    <label class="checkbox-item">
                        <input
                            type="checkbox"
                            v-model="scanParams.force"
                            :disabled="isScanning"
                            class="checkbox-input"
                        />
                        <span class="checkbox-label">
                            <strong>Force Scan</strong>
                            <span class="checkbox-description">
                                Re-scan all items regardless of last scan time
                            </span>
                        </span>
                    </label>

                    <label class="checkbox-item">
                        <input
                            type="checkbox"
                            v-model="scanParams.artists"
                            :disabled="isScanning"
                            class="checkbox-input"
                        />
                        <span class="checkbox-label">
                            <strong>Artists</strong>
                            <span class="checkbox-description">
                                Update artist metadata and information
                            </span>
                        </span>
                    </label>

                    <label class="checkbox-item">
                        <input
                            type="checkbox"
                            v-model="scanParams.releases"
                            :disabled="isScanning"
                            class="checkbox-input"
                        />
                        <span class="checkbox-label">
                            <strong>Releases</strong>
                            <span class="checkbox-description">
                                Update album/release metadata and cover art
                            </span>
                        </span>
                    </label>

                    <label class="checkbox-item">
                        <input
                            type="checkbox"
                            v-model="scanParams.scrobbles"
                            :disabled="isScanning"
                            class="checkbox-input"
                        />
                        <span class="checkbox-label">
                            <strong>Scrobbles</strong>
                            <span class="checkbox-description">
                                Process unprocessed scrobble data
                            </span>
                        </span>
                    </label>
                </div>
            </div>

            <div class="form-actions">
                <Button
                    variant="confirm"
                    type="submit"
                    :disabled="isScanning || !hasSelectedOptions"
                >
                    <div v-if="isScanning" class="scanning-content">
                        <div class="spinner"></div>
                        <span>Scanning...</span>
                    </div>
                    <div v-else class="button-content">
                        <svg
                            class="scan-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path d="M3 7v6h6" />
                            <path d="M21 17v-6h-6" />
                            <path d="M10 22l-6-6 6-6" />
                            <path d="M14 2l6 6-6 6" />
                        </svg>
                        <span>Start Scan</span>
                    </div>
                </Button>

                <Button
                    type="button"
                    @click="resetForm"
                    :disabled="isScanning"
                >
                    Reset
                </Button>
            </div>

            <div v-if="scanResult" class="scan-result" :class="scanResult.type">
                <div class="result-icon">
                    <svg
                        v-if="scanResult.type === 'success'"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
                        <polyline points="22,4 12,14.01 9,11.01" />
                    </svg>
                    <svg
                        v-else
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <circle cx="12" cy="12" r="10" />
                        <line x1="15" y1="9" x2="9" y2="15" />
                        <line x1="9" y1="9" x2="15" y2="15" />
                    </svg>
                </div>
                <div class="result-content">
                    <h4>{{ scanResult.title }}</h4>
                    <p>{{ scanResult.message }}</p>
                </div>
            <Button class="result-close" variant="close" @click="clearResult">
				<svg
					class="icon"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M18 6L6 18M6 6l12 12" />
				</svg>
			</Button>
            </div>
        </form>

        <div class="scan-info">
            <h3>What does each option do?</h3>
            <div class="info-grid">
                <div class="info-item">
                    <div class="info-header">
                        <svg
                            class="info-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path
                                d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"
                            />
                            <circle cx="12" cy="7" r="4" />
                        </svg>
                        <strong>Artists</strong>
                    </div>
                    <p>
                        Updates artist information, biographies, and metadata
                        from MusicBrainz
                    </p>
                </div>

                <div class="info-item">
                    <div class="info-header">
                        <svg
                            class="info-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <circle cx="12" cy="12" r="10" />
                            <circle cx="12" cy="12" r="3" />
                        </svg>
                        <strong>Releases</strong>
                    </div>
                    <p>
                        Updates album metadata, cover art, and release
                        information
                    </p>
                </div>

                <div class="info-item">
                    <div class="info-header">
                        <svg
                            class="info-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <polygon points="5,3 19,12 5,21 5,3" />
                        </svg>
                        <strong>Scrobbles</strong>
                    </div>
                    <p>
                        Processes unprocessed listening data and matches it to
                        catalog entries
                    </p>
                </div>

                <div class="info-item">
                    <div class="info-header">
                        <svg
                            class="info-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path d="M3 7v6h6" />
                            <path d="M21 17v-6h-6" />
                        </svg>
                        <strong>Force</strong>
                    </div>
                    <p>
                        Ignores last scan timestamps and re-processes all
                        selected items
                    </p>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import Button from "@/base/Button.vue";
import { type ScanParams, useAdminStore } from "@/stores/adminStore";

const adminStore = useAdminStore();

const scanParams = ref<ScanParams>({
    force: false,
    artists: true,
    releases: true,
    scrobbles: true,
});

interface ScanResult {
    type: "success" | "error";
    title: string;
    message: string;
}

const scanResult = ref<ScanResult | null>(null);

const isScanning = computed(() => adminStore.isScanning);

const hasSelectedOptions = computed(() => {
    return (
        scanParams.value.artists ||
        scanParams.value.releases ||
        scanParams.value.scrobbles
    );
});

const handleSubmit = async () => {
    if (!hasSelectedOptions.value) {
        scanResult.value = {
            type: "error",
            title: "No Options Selected",
            message: "Please select at least one scan option before starting.",
        };
        return;
    }

    try {
        clearResult();

        await adminStore.scanDatabase(scanParams.value);

        scanResult.value = {
            type: "success",
            title: "Scan Started Successfully",
            message: `Database scan initiated with the selected options. The process is running in the background.`,
        };
    } catch (error) {
        console.error("Scan failed:", error);

        scanResult.value = {
            type: "error",
            title: "Scan Failed",
            message:
                error instanceof Error
                    ? error.message
                    : "An unexpected error occurred while starting the scan.",
        };
    }
};

const resetForm = () => {
    scanParams.value = {
        force: false,
        artists: true,
        releases: true,
        scrobbles: true,
    };
    clearResult();
};

const clearResult = () => {
    scanResult.value = null;
};
</script>

<style scoped>
.scan-form {
    max-width: 800px;
    margin: 0 auto;
}

.form-header {
    margin-bottom: 32px;
    text-align: center;
}

.form-header h2 {
    color: var(--text-color);
    margin: 0 0 8px 0;
    font-size: 1.8em;
}

.form-description {
    color: var(--text-secondary);
    font-size: 1.1em;
    margin: 0;
}

.scan-form-content {
    background: var(--card-background);
    border-radius: 8px;
    padding: 24px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
    margin-bottom: 24px;
}

.form-section {
    margin-bottom: 24px;
}

.form-section h3 {
    color: var(--text-color);
    margin: 0 0 16px 0;
    font-size: 1.2em;
}

.checkbox-group {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.checkbox-item {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    cursor: pointer;
    padding: 12px;
    border-radius: 6px;
    transition: background-color 0.2s;
}

.checkbox-item:hover:not(:has(.checkbox-input:disabled)) {
    background: rgba(255, 255, 255, 0.05);
}

.checkbox-input {
    width: 18px;
    height: 18px;
    margin: 0;
    cursor: pointer;
    accent-color: var(--primary-color);
}

.checkbox-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.checkbox-label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
}

.checkbox-label strong {
    color: var(--text-color);
    font-size: 0.95em;
}

.checkbox-description {
    color: var(--text-secondary);
    font-size: 0.85em;
    line-height: 1.4;
}

.form-actions {
    display: flex;
    gap: 12px;
    justify-content: center;
    flex-wrap: wrap;
}

.button-content,
.scanning-content {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
}

.scan-icon {
    width: 20px;
    height: 20px;
}

.spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg);
    }
}

.scan-result {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 16px;
    border-radius: 6px;
    margin-top: 16px;
    position: relative;
}

.scan-result.success {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.3);
}

.scan-result.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
}

.result-icon {
    flex-shrink: 0;
    width: 20px;
    height: 20px;
    margin-top: 2px;
}

.result-icon svg {
    width: 100%;
    height: 100%;
}

.scan-result.success .result-icon {
    color: #22c55e;
}

.scan-result.error .result-icon {
    color: #ef4444;
}

.result-content {
    flex: 1;
}

.result-content h4 {
    margin: 0 0 4px 0;
    color: var(--text-color);
    font-size: 0.95em;
}

.result-content p {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.9em;
    line-height: 1.4;
}

.scan-info {
    background: rgba(255, 255, 255, 0.02);
    border-radius: 8px;
    padding: 24px;
    border: 1px solid var(--border-color);
}

.scan-info h3 {
    color: var(--text-color);
    margin: 0 0 16px 0;
    font-size: 1.1em;
}

.info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 16px;
}

.info-item {
    padding: 16px;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.1);
}

.info-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
}

.info-icon {
    width: 16px;
    height: 16px;
    color: var(--primary-color);
}

.info-header strong {
    color: var(--text-color);
    font-size: 0.9em;
}

.info-item p {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.85em;
    line-height: 1.4;
}

@media (max-width: 600px) {
    .scan-form {
        padding: 0 16px;
    }

    .form-actions {
        flex-direction: column;
        align-items: center;
    }

    .scan-button,
    .reset-button {
        width: 100%;
        max-width: 300px;
    }

    .info-grid {
        grid-template-columns: 1fr;
    }
}
</style>
