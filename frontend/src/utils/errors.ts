export const handleImageError = (
    event: Event,
    opacity?: string,
    transform?: string,
) => {
    const img = event.target as HTMLImageElement;
    img.src = "/img/photo-off.svg";
    if (opacity) {
        img.style.opacity = opacity;
    }
    if (transform) {
        img.style.transform = transform;
    }
};
