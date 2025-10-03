import { defineStore } from "pinia";
import { ref } from "vue";
import type { ScrobzEvent } from "@/types";

export const useSseStore = defineStore("sse", () => {
    const messages = ref<ScrobzEvent[]>([]);

    const addMessage = (msg: ScrobzEvent) => {
        if (msg.type === "PlayingNow") {
            messages.value = messages.value.filter(
                (existingMsg) =>
                    !(
                        existingMsg.type === "PlayingNow" &&
                        existingMsg.user === msg.user
                    ),
            );
        }

        messages.value.push(msg);

        if (msg.track_duration) {
            setTimeout(() => {
                const index = messages.value.indexOf(msg);
                if (index > -1) {
                    messages.value.splice(index, 1);
                }
            }, msg.track_duration);
        }
    };

    return { messages, addMessage };
});
