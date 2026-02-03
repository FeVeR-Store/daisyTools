<template>
  <svg
    :class="inline ? 'inline' : ''"
    :width="sizeValue"
    :height="sizeValue"
    :viewBox="viewboxValue"
    :style="styles"
  >
    <path :d="path" />
  </svg>
</template>

<script setup lang="ts">
import { computed } from "vue";

// todo : move this into own release
const types = <const>{
  mdi: {
    size: 24,
    viewbox: "0 0 24 24",
  },
  "simple-icons": {
    size: 24,
    viewbox: "0 0 24 24",
  },
  default: {
    size: 0,
    viewbox: "0 0 0 0",
  },
};

const props = withDefaults(
  defineProps<{
    type?: keyof typeof types;
    path: string;
    size?: string | number;
    viewbox?: string;
    flip?: "horizontal" | "vertical" | "both" | "none";
    rotate?: number;
    inline?: boolean;
    color?: string;
  }>(),
  { type: "mdi", inline: false }
);

const styles = computed(() => ({
  "--sx": ["both", "horizontal"].includes(props.flip!) ? "-1" : "1",
  "--sy": ["both", "vertical"].includes(props.flip!) ? "-1" : "1",
  "--r": isNaN(props.rotate!) ? props.rotate : props.rotate + "deg",
  "color": props.color
}));

const defaults = computed(() => types[props.type] || types.default);

const sizeValue = computed(() => props.size || defaults.value.size);

const viewboxValue = computed(() => props.viewbox || defaults.value.viewbox);
</script>

<style scoped>
svg {
  transform: rotate(var(--r, 0deg)) scale(var(--sx, 1), var(--sy, 1));
}

path {
  fill: currentColor;
}
</style>
