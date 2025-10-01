import { ref } from "vue";

export function useNotifications() {
    const permission = ref(Notification.permission);

    const requestPermission = async () => {
        if (!("Notification" in window)) return;
        permission.value = await Notification.requestPermission();
    };

    const notify = (title: string, options: NotificationOptions) => {
        if (permission.value === "granted") {
            new Notification(title, options);
        } else {
            console.warn("Notifications permission not granted");
        }
    };

    return { permission, requestPermission, notify };
}
