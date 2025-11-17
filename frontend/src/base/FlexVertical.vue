<template>
<div class="flex-vertical">
<slot class="container-content"/>
</div>
</template>

<style scoped lang="scss">
.flex-vertical {
    background: var(--card-background);
    border-radius: 8px;
    padding: 16px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    flex: 1;
    gap: 8px;
    transition: all 0.2s;
}

.flex-vertical:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
}

@media screen and (max-width: 500px) {
    .flex-vertical {
        padding: 8px;
    }
}

</style>
