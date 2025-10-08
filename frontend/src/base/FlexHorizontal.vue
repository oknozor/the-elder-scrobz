<template>
<div class="flex-horizontal">
<slot/>
</div>
</template>

<style scoped lang="scss">
.flex-horizontal {
    background: var(--card-background);
    border-radius: 8px;
    padding: 24px;
    border: 1px solid var(--border-color);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    display: flex;
    align-items: center;
    gap: 16px;
    transition: all 0.2s;
}

.flex-horizontal:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
}

@media screen and (max-width: 500px) {
    .flex-horizontal {
        padding: 8px;
    }
}

</style>
