import { defineStore } from "pinia";
import { ref } from "vue";
import type { ScrobzEvent } from "@/types";

export const useSseStore = defineStore("sse", () => {
    const messages = ref<ScrobzEvent[]>([]);

    const addMessage = (msg: ScrobzEvent) => {
        messages.value.push(msg);
    };

    return { messages, addMessage };
});
