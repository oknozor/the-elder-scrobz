<template>
    <div class="json-node">
        <div v-if="isObject || isArray" class="json-expandable">
            <button
                @click="toggle"
                class="json-toggle"
                :class="{ expanded: isExpanded }"
            >
                <svg class="json-arrow" viewBox="0 0 20 20" fill="currentColor">
                    <path
                        fill-rule="evenodd"
                        d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                        clip-rule="evenodd"
                    />
                </svg>
            </button>

            <span class="json-bracket" @click="toggle">{{ openBracket }}</span>

            <span v-if="!isExpanded" class="json-collapsed" @click="toggle">
                {{ collapsedText }}
            </span>

            <div v-if="isExpanded" class="json-expanded">
                <div v-if="isObject" class="json-content">
                    <div
                        v-for="(val, key) in value as Record<string, any>"
                        :key="key"
                        class="json-property"
                    >
                        <span class="json-key">"{{ key }}"</span>
                        <span class="json-colon">:</span>
                        <JsonNode
                            :value="val"
                            :path="[...path, String(key)]"
                            :expanded="expanded"
                            @toggle="$emit('toggle', $event)"
                        />
                        <span v-if="!isLastProperty(key)" class="json-comma"
                            >,</span
                        >
                    </div>
                </div>

                <div v-if="isArray" class="json-content">
                    <div
                        v-for="(item, index) in value as any[]"
                        :key="index"
                        class="json-property"
                    >
                        <JsonNode
                            :value="item"
                            :path="[...path, String(index)]"
                            :expanded="expanded"
                            @toggle="$emit('toggle', $event)"
                        />
                        <span v-if="index < value.length - 1" class="json-comma"
                            >,</span
                        >
                    </div>
                </div>
            </div>

            <span v-if="isExpanded" class="json-bracket">{{
                closeBracket
            }}</span>
        </div>

        <span v-else class="json-primitive" :class="primitiveType">
            {{ formattedValue }}
        </span>
    </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

interface Props {
    value: any;
    path: string[];
    expanded: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
    toggle: [path: string[]];
}>();

const isObject = computed(
    () =>
        typeof props.value === "object" &&
        props.value !== null &&
        !Array.isArray(props.value),
);

const isArray = computed(() => Array.isArray(props.value));

const isExpanded = computed(() => {
    return props.expanded;
});

const openBracket = computed(() => (isArray.value ? "[" : "{"));
const closeBracket = computed(() => (isArray.value ? "]" : "}"));

const collapsedText = computed(() => {
    if (isArray.value) {
        const length = props.value.length;
        return length === 0 ? "" : `${length} item${length === 1 ? "" : "s"}`;
    } else if (isObject.value) {
        const keys = Object.keys(props.value);
        const length = keys.length;
        return length === 0
            ? ""
            : `${length} propert${length === 1 ? "y" : "ies"}`;
    }
    return "";
});

const primitiveType = computed(() => {
    if (props.value === null) return "json-null";
    if (typeof props.value === "boolean") return "json-boolean";
    if (typeof props.value === "number") return "json-number";
    if (typeof props.value === "string") return "json-string";
    return "";
});

const formattedValue = computed(() => {
    if (props.value === null) return "null";
    if (typeof props.value === "string") return `"${props.value}"`;
    return String(props.value);
});

const isLastProperty = (key: string) => {
    if (!isObject.value) return false;
    const keys = Object.keys(props.value);
    return keys[keys.length - 1] === key;
};

const toggle = () => {
    emit("toggle", props.path);
};
</script>

<style scoped>
.json-node {
    display: inline;
}

.json-expandable {
    display: inline;
}

.json-toggle {
    background: none;
    border: none;
    padding: 2px;
    cursor: pointer;
    color: var(--json-arrow, #888);
    transition:
        transform 0.2s ease,
        color 0.2s ease;
    vertical-align: top;
    margin-top: 1px;
}

.json-toggle:hover {
    color: var(--json-arrow-hover, #ccc);
}

.json-toggle.expanded .json-arrow {
    transform: rotate(90deg);
}

.json-arrow {
    width: 12px;
    height: 12px;
    transition: transform 0.2s ease;
}

.json-bracket {
    color: var(--json-bracket, #888);
    font-weight: bold;
    cursor: pointer;
}

.json-bracket:hover {
    color: var(--json-bracket-hover, #ccc);
}

.json-collapsed {
    color: var(--json-collapsed, #666);
    font-style: italic;
    cursor: pointer;
    margin-left: 4px;
}

.json-collapsed:hover {
    color: var(--json-collapsed-hover, #999);
}

.json-expanded {
    display: block;
    margin-left: 16px;
}

.json-content {
    display: block;
}

.json-property {
    display: block;
    margin: 2px 0;
}

.json-key {
    color: var(--json-key, #3b82f6);
    font-weight: 500;
}

.json-colon {
    color: var(--json-punctuation, #888);
    margin: 0 4px;
}

.json-comma {
    color: var(--json-punctuation, #888);
}

.json-primitive.json-string {
    color: var(--json-string, #10b981);
}

.json-primitive.json-number {
    color: var(--json-number, #f59e0b);
}

.json-primitive.json-boolean {
    color: var(--json-boolean, #8b5cf6);
    font-weight: 500;
}

.json-primitive.json-null {
    color: var(--json-null, #ef4444);
    font-weight: 500;
}

/* Dark theme adjustments */
@media (prefers-color-scheme: dark) {
    .json-toggle {
        --json-arrow: #999;
        --json-arrow-hover: #ddd;
    }

    .json-bracket {
        --json-bracket: #999;
        --json-bracket-hover: #ddd;
    }

    .json-collapsed {
        --json-collapsed: #777;
        --json-collapsed-hover: #aaa;
    }

    .json-key {
        --json-key: #60a5fa;
    }

    .json-primitive {
        --json-punctuation: #999;
        --json-string: #34d399;
        --json-number: #fbbf24;
        --json-boolean: #a78bfa;
        --json-null: #f87171;
    }
}
</style>
