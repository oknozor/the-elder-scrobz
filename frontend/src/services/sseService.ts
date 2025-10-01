import { useSseStore } from "@/stores";
import type { ScrobzEvent } from "@/types/event";

let eventSource: EventSource | null = null;

export const startSse = () => {
  const sseStore = useSseStore();

  if (eventSource) return;

  eventSource = new EventSource("/api/v1/events");

  eventSource.onmessage = (event) => {
    try {
      const parsedEvent: ScrobzEvent = JSON.parse(event.data);
      sseStore.addMessage(parsedEvent);
    } catch (error) {
      console.error("Failed to parse SSE event:", error);
    }
  };

  eventSource.onerror = (err) => {
    console.error("SSE error:", err);
    eventSource?.close();
    eventSource = null;
  };
};

export const stopSse = () => {
  eventSource?.close();
  eventSource = null;
};
