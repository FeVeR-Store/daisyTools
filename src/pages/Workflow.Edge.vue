<script setup lang="ts">
import { computed, inject, ref, StyleValue } from "vue";
import {
  TransitionPresets,
  useDebounceFn,
  useTransition,
  watchDebounced,
} from "@vueuse/core";
import { getSmoothStepPath, Node, Position, useVueFlow } from "@vue-flow/core";
import { watch } from "vue";
import { transitionKey } from "./Workflow.utils";

const props = defineProps<{
  id: string;
  source: string;
  target: string;
  sourceX: number;
  sourceY: number;
  targetX: number;
  targetY: number;
  sourcePosition: Position;
  targetPosition: Position;
  data: object;
  markerEnd?: string;
  style?: StyleValue;
}>();

const curve = ref();

const dot = ref();

const transform = ref({ x: 0, y: 0 });

const showDot = ref(false);

const { onNodeDoubleClick, fitBounds, fitView } = useVueFlow();

const path = computed(() =>
  getSmoothStepPath({
    sourceX: props.sourceX,
    sourceY: props.sourceY,
    sourcePosition: props.sourcePosition,
    targetX: props.targetX,
    targetY: props.targetY,
    targetPosition: props.targetPosition,
  })
);

const debouncedFitBounds = useDebounceFn(fitBounds, 1, { maxWait: 1 });

const transition = ({ node }: { node: Node }) => {
  const isSource = props.source === node.id;

  if (isSource) {
    showDot.value = true;
    let totalLength = curve.value.getTotalLength();
    if (totalLength <= 1) {
      showDot.value = false;
      fitView({
        nodes: [isSource ? props.target : props.source],
        duration: 500,
      });
      return;
    }
    const initialPos = ref(isSource ? 0 : totalLength);
    let stopHandle: () => void;

    const output = useTransition(initialPos, {
      duration: (totalLength / 200) * 1000,
      transition: TransitionPresets.linear,
      onFinished: () => {
        stopHandle?.();
        showDot.value = false;
        fitView({
          nodes: [isSource ? props.target : props.source],
          duration: 500,
        });
        finishTransition();
      },
    });

    transform.value = curve.value.getPointAtLength(output.value);

    debouncedFitBounds(
      {
        width: 100,
        height: 200,
        x: transform.value.x - 100,
        y: transform.value.y - 100,
      },
      { duration: 500 }
    );

    setTimeout(() => {
      initialPos.value = isSource ? totalLength : 0;

      stopHandle = watchDebounced(
        output,
        (next) => {
          if (!showDot.value) {
            return;
          }
          transform.value = curve.value.getPointAtLength(next);
          debouncedFitBounds({
            width: 100,
            height: 200,
            x: transform.value.x - 100,
            y: transform.value.y - 100,
          });
        },
        { debounce: 1 }
      );
    }, 500);
  }
};

const { currentNode, targetNode, finishTransition } = inject(transitionKey)!;

watch(currentNode, (node) => {
  if (node) {
    if (targetNode.value?.id === props.target) {
      transition({ node });
    }
  }
});

onNodeDoubleClick(transition);
defineOptions({
  inheritAttrs: false,
});

// 暴露transition方法供父组件调用
defineExpose({
  transition,
});
</script>

<template>
  <!-- <path
    :id
    ref="curve"
    :style
    class="vue-flow__edge-path"
    :d="path[0]"
    :marker-end="markerEnd"
  /> -->
  <defs>
    <!-- 渐变定义 -->
    <linearGradient
      id="grad"
      x1="0%"
      y1="0%"
      x2="100%"
      y2="0%"
      :opacity="'10%'"
    >
      <stop offset="0%" stop-color="var(--color-accent)" />
    </linearGradient>

    <!-- 阴影滤镜 -->
    <filter id="shadow" x="-10%" y="-10%" width="120%" height="120%">
      <feDropShadow dx="1" dy="1" stdDeviation="1.2" flood-color="#888" />
    </filter>
  </defs>

  <path
    :id
    ref="curve"
    :style="{
      fill: 'none',
      'stroke-width': 1.5,
      'stroke-linecap': 'round',
      filter: 'url(#shadow)',
    }"
    class="vue-flow__edge-path"
    :d="path[0]"
    :marker-end="markerEnd"
  />
  <path
    :id
    ref="curve"
    :style="{
      opacity: '60%',
      stroke: 'var(--color-accent)',
      'stroke-width': 0.8,
      'stroke-linecap': 'round',
      filter: 'url(#shadow)',
    }"
    class="vue-flow__edge-path animated"
    :d="path[0]"
    :marker-end="markerEnd"
  />
  <Transition name="fade">
    <circle
      v-if="showDot"
      ref="dot"
      r="5"
      cy="0"
      cx="0"
      :transform="`translate(${transform.x}, ${transform.y})`"
      style="fill: #fdd023"
    />
  </Transition>
</template>
