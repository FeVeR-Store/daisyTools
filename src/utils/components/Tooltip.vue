<template>
  <Transition name="opacity">
    <div
      v-show="force || show"
      class="fixed"
      :style="{ top: y + 'px', left: x + 'px' }"
      ref="wrapperRef"
    >
      <slot></slot>
      <div
        class="tooltip tooltip-wrapper"
        :style="tooltipStyle"
        role="tooltip"
        ref="tooltipRef"
      >
        <div class="tooltip-arrow" :style="arrowStyle"></div>
        <div class="tooltip-inner">{{ content }}</div>
      </div>
    </div>
  </Transition>
</template>

<script lang="ts" setup>
import { ref, reactive } from "vue";

export interface TooltipProps {
  x: number;
  y: number;
  content: string;
  position: "top" | "bottom" | "left" | "right";
  force: boolean;
  show: boolean;
}

defineProps<TooltipProps>();

const wrapperRef = ref<HTMLElement | null>(null);
const tooltipRef = ref<HTMLElement | null>(null);

const tooltipStyle = reactive<any>({
  position: "absolute",
  top: "0px",
  left: "0px",
  inset: "auto auto auto auto",
  boxSizing: "border-box",
  "--arrow-x": "50%",
  "--arrow-y": "100%",
});

const arrowStyle = reactive<any>({});
</script>

<style scoped> 
.tooltip-wrapper {
  transform: translate(-50%, -120%);
}

.tooltip {
  background: rgba(0, 0, 0, 0.75);
  color: white;
  padding: 1px 2px;
  border-radius: 4px;
  font-size: 13px;
  position: absolute;
  z-index: 1000;
  white-space: nowrap;
  box-sizing: border-box;
}

.tooltip-arrow {
  width: 8px;
  height: 8px;
  background: rgba(0, 0, 0, 0.75);
  position: absolute;
  bottom: -4px;
  transform: rotate(45deg);
  left: 50%;
  margin-left: -4px;
  left: 50%;
}

.tooltip-inner {
  padding: 4px 8px;
}
</style>
