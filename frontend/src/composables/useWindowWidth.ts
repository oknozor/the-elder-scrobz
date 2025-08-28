import { onBeforeUnmount, onMounted, ref } from "vue";

export function useWindowWidth() {
    const windowWidth = ref(window.innerWidth);

    const updateWidth = () => {
        windowWidth.value = window.innerWidth;
    };

    onMounted(() => {
        window.addEventListener("resize", updateWidth);
    });

    onBeforeUnmount(() => {
        window.removeEventListener("resize", updateWidth);
    });

    return windowWidth;
}
