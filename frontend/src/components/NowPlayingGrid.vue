<template>
    <Transition name="now-playing" mode="out-in">
        <div v-if="nowPlayingEvents.length > 0" class="now-playing-display">
            <div class="section-header">
                <svg
                    class="title-icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <circle cx="12" cy="12" r="10" />
                    <polygon points="10,8 16,12 10,16 10,8" />
                </svg>
                <h2>Now Playing</h2>
                <Button
                    @click="notificationsEnabled = !notificationsEnabled"
                    :class="[
                        'notification-toggle',
                    ]"
                    :active="notificationsEnabled"
                    :title="
                        notificationsEnabled
                            ? 'Disable notifications'
                            : 'Enable notifications'
                    "
                >
                    <svg
                        v-if="notificationsEnabled"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
                        <path d="M13.73 21a2 2 0 0 1-3.46 0" />
                    </svg>
                    <svg
                        v-else
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path d="M6.16 6.16a6 6 0 1 0 8.68 8.68" />
                        <path d="M10 10v.01" />
                        <path d="M13 13v.01" />
                        <path d="M16 16v.01" />
                        <path d="m1 1 22 22" />
                        <path d="M6.5 6.5C3 10 3 17 3 17h14" />
                    </svg>
                </Button>
            </div>
            <TransitionGroup name="card" tag="div" class="now-playing-cards">
                <PlayingNowCard
                    v-for="(event, index) in nowPlayingEvents"
                    :key="`${event.user}-${event.track_name}-${index}`"
                    :playing-now="event"
                    class="now-playing-item"
                />
            </TransitionGroup>
        </div>
    </Transition>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import Button from "@/base/Button.vue";
import { useNotifications } from "@/composables/useNotifications";
import { useSseStore } from "@/stores";
import { PlayingNow } from "@/types";
import PlayingNowCard from "./NowPlayingCard.vue";

const sseStore = useSseStore();
const { permission, requestPermission, notify } = useNotifications();

const previousEventKeys = ref(new Set<string>());
const notificationsEnabled = ref(true);
const lastNotificationTime = ref(0);
const NOTIFICATION_COOLDOWN = 1000; // 1 second between notifications

const sendNotification = async (event: PlayingNow): Promise<void> => {
    if (!notificationsEnabled.value) return;

    // Prevent notification spam
    const now = Date.now();
    if (now - lastNotificationTime.value < NOTIFICATION_COOLDOWN) return;
    lastNotificationTime.value = now;

    // Request permission if not yet granted
    if (permission.value === "default") {
        await requestPermission();
    }

    // Fire notification if permission granted
    if (permission.value === "granted") {
        const title = `🎵 ${event.user} is now playing`;
        const body = `${event.track_name}\nby ${event.artist}`;

        notify(title, {
            body,
            icon: event.cover_art_url || "/music-icon.svg",
            tag: `now-playing-${event.user}`, // Replace previous notification from same user
            requireInteraction: false,
            silent: false,
        });
    }
};

const nowPlayingEvents = computed(() => {
    return sseStore.messages
        .filter((event) => event.type === "PlayingNow")
        .slice(0, 8);
});

// Auto-request notification permission on component mount
onMounted(async () => {
    if (permission.value === "default") {
        await requestPermission();
    }
});

// Watch for new now playing events and send notifications
watch(
    nowPlayingEvents,
    (newEvents, oldEvents = []) => {
        // Only process truly new events by comparing with previous state
        const oldEventKeys = new Set(
            oldEvents.map(
                (event) => `${event.user}-${event.track_name}-${event.artist}`,
            ),
        );

        newEvents.forEach((event) => {
            const eventKey = `${event.user}-${event.track_name}-${event.artist}`;

            // Check if this is a new event we haven't seen in the previous state
            if (
                !oldEventKeys.has(eventKey) &&
                !previousEventKeys.value.has(eventKey)
            ) {
                previousEventKeys.value.add(eventKey);
                sendNotification(event);
            }
        });

        // Clean up old event keys to prevent memory leaks
        // Keep only keys for current events
        const currentKeys = new Set(
            newEvents.map(
                (event) => `${event.user}-${event.track_name}-${event.artist}`,
            ),
        );
        previousEventKeys.value = currentKeys;
    },
    { immediate: false },
);
</script>

<style scoped>
.now-playing-display {
    margin-bottom: var(--app-padding);
}

.section-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;
    padding-bottom: 8px;
    border-bottom: 2px solid var(--border-color);
    justify-content: flex-start;
}

.title-icon {
    width: 24px;
    height: 24px;
    color: var(--primary-color);
}

.section-header h2 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-color);
    flex: 1;
}

.notification-toggle {
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
}

.notification-toggle:hover {
    border-color: var(--primary-color);
    color: var(--primary-color);
}

.notification-toggle.active {
    color: var(--primary-color);
    border-color: var(--primary-color);
    background: rgba(59, 130, 246, 0.1);
}

.notification-toggle svg {
    width: 18px;
    height: 18px;
}

.now-playing-cards {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    align-items: flex-start;
    width: 100%;
}

.now-playing-item {
    min-width: 0;
    flex: 0 1 calc((100% - 24px) / 3);
}

/* Now Playing section transitions */
.now-playing-enter-active {
    transition: all 0.5s ease;
}

.now-playing-leave-active {
    transition: all 0.3s ease;
}

.now-playing-enter-from {
    opacity: 0;
    transform: translateY(-20px);
    max-height: 0;
    margin-bottom: 0;
}

.now-playing-leave-to {
    opacity: 0;
    transform: translateY(-10px);
    max-height: 0;
    margin-bottom: 0;
}

/* Individual card transitions */
.card-enter-active {
    transition: all 0.4s ease;
}

.card-leave-active {
    transition: all 0.3s ease;
}

.card-enter-from {
    opacity: 0;
    transform: translateX(-30px) scale(0.95);
}

.card-leave-to {
    opacity: 0;
    transform: translateX(30px) scale(0.95);
}

.card-move {
    transition: transform 0.3s ease;
}

@media screen and (max-width: 768px) {
    .section-header h2 {
        font-size: 1.3rem;
    }

    .title-icon {
        width: 20px;
        height: 20px;
    }

    .notification-toggle svg {
        width: 16px;
        height: 16px;
    }

    .notification-toggle {
        padding: 5px;
    }

    .now-playing-cards {
        gap: 8px;
    }

    .now-playing-item {
        flex: 0 1 100%;
    }
}

@media screen and (max-width: 1024px) {
    .now-playing-item {
        flex: 0 1 calc((100% - 24px) / 3);
    }
}

@media screen and (max-width: 900px) {
    .now-playing-item {
        flex: 0 1 calc((100% - 12px) / 2);
    }
}
</style>
