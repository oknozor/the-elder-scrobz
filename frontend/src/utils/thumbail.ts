export const loadImage = (thumbnail_url: string | null): string => {
    if (thumbnail_url) {
        if (thumbnail_url.startsWith("http")) {
            return thumbnail_url.replace(/^http:/, "https:");
        }
        return (import.meta.env.VITE_API_BASE_URL || "") + thumbnail_url;
    } else {
        return "";
    }
};
