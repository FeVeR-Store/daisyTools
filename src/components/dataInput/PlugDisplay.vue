<script setup lang="ts">
import { mdiChevronRight, mdiClose, mdiPowerPlugOutline } from "@mdi/js";
import Join from "../../utils/components/Join.vue";
import Icon from "../Icon.vue";
import { computed } from "vue";
import { usePlugPath } from "./PlugDisplay.utils";
const { displayStyle, plug } = defineProps<{
  plug?: string[] | any;
  displayStyle?: "range" | "input" | "select" | "toggle" | "code";
}>();

const { plugPath } = usePlugPath(() => plug, 3);

const displayClass = computed(() => {
  switch (displayStyle) {
    case "range":
      return "range-style range";
    case "input":
    case "select":
      return "input";
    case "toggle":
      return "h-[calc(var(--size-selector)*6)]";
    case "code":
      return "code-style";
  }
});

const emit = defineEmits<{
  remove: [];
}>();
</script>

<template>
  <div v-bind="$attrs" class="flex m-auto flex-col w-full" :class="displayClass" v-if="plug">
    <div
      class="divider w-full h-18 my-auto transition-all flex hover-show-action"
    >
      <div class="badge badge-primary transition-transform">
        <Icon size="18" :path="mdiPowerPlugOutline"></Icon>
        <Join :list="plugPath">
          <template #separator>
            <Icon size="18" :path="mdiChevronRight"></Icon>
          </template>
        </Join>
      </div>
      <button
        @click="emit('remove')"
        class="badge badge-primary px-1 transition-all"
      >
        <Icon size="18" :path="mdiClose"></Icon>
      </button>
    </div>
  </div>
  <div
    v-if="displayStyle === 'code'"
    :class="[{ 'blur-[4px]': plug }, 'transition-all']"
  >
    <slot name="onlyForCodeEditor"></slot>
  </div>
</template>

<style scoped>
.code-style {
  position: relative;
  & > div.divider {
    position: absolute;
    z-index: 1;
    top: 0;
    left: 0;
  }
}
.range-style {
  & > div.divider::before {
    height: 0.75rem;
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }
  & > div.divider::after {
    height: 0.75rem;
    border-top-right-radius: 0.5rem;
    border-bottom-right-radius: 0.5rem;
  }
}

.hover-show-action {
  --translate-x: 0;
  --action-display: none;
  --action-scale: 0;
  & > div.badge {
    transform: translateX(var(--translate-x));
  }
  & > button.badge {
    display: var(--action-display);
    transform: scale(var(--action-scale));
  }
  &:hover {
    animation: show-action 0.2s forwards;
  }
  &:not(:hover) {
    animation: hide-action 0.2s forwards;
  }
}
@keyframes show-action {
  0% {
    --translate-x: 0;
    --action-scale: 0;
  }
  65% {
    --translate-x: -0.4rem;
  }
  66% {
    --translate-x: -0.4rem;
    --action-display: inline-flex;
    --action-scale: 0;
  }
  100% {
    --translate-x: 0;
    --action-display: inline-flex;
    --action-scale: 1;
  }
}
@keyframes hide-action {
  0% {
    --translate-x: 0;
    --action-scale: 1;
    --action-display: inline-flex;
  }
  34% {
    --translate-x: -0.4rem;
    --action-scale: 0;
    --action-display: inline-flex;
  }
  35% {
    --translate-x: -0.4rem;
  }
  100% {
    --translate-x: 0;
  }
}
</style>
