import { useSseStore } from "@/stores";
import type { ScrobzEvent } from "@/types/event";

let eventSource: EventSource | null = null;
let isUnloading = false;

if (typeof window !== "undefined") {
    window.addEventListener("beforeunload", () => {
        isUnloading = true;
        stopSse();
    });
}

export const startSse = async () => {
    const sseStore = useSseStore();

    if (eventSource) return;

    const url = `/api/v1/events`;
    eventSource = new EventSource(url);

    eventSource.onmessage = (event) => {
        try {
            const parsedEvent: ScrobzEvent = JSON.parse(event.data);
            sseStore.addMessage(parsedEvent);
        } catch (error) {
            console.error("Failed to parse SSE event:", error);
        }
    };

    eventSource.onerror = (_) => {
        // prevent sse error to be logged twice during page reload
        if (isUnloading) {
            return;
        }

        console.log("SSE error, attempting to reconnect...");
        if (eventSource && eventSource.readyState === EventSource.CLOSED) {
            eventSource = null;
        }
    };
};

export const stopSse = () => {
    eventSource?.close();
    eventSource = null;
};
