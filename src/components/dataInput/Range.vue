<template>
  <template v-if="!plug">
    <input
      v-bind="$attrs"
      type="range"
      :min
      :max
      :step
      v-model="value"
      class="range w-full"
    />
  </template>
  <PlugDisplay
    @remove="$emit('plug-remove')"
    :plug
    display-style="range"
  ></PlugDisplay>
  <div v-if="measure !== 0" class="flex justify-between px-2.5 mt-2 text-xs">
    <span v-for="measure in measures">{{ measure }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import PlugDisplay from "./PlugDisplay.vue";
import { usePlug } from "./PlugDisplay.utils";

const {
  measure = 2,
  max = 100,
  min = 0,
  step = 1,
} = defineProps<{
  max: number;
  min: number;
  step: number;
  measure?: number;
  plug?: string[];
}>();

const value = defineModel();

const { plug } = usePlug(value);

const measures = computed(() => {
  const distance = Math.round((max - min) / (measure - 1));
  // 当只有一个标尺时，转化为两个
  return new Array(measure === 1 ? 2 : measure).fill(0).map((_, i) => {
    // 防止精度问题导致最大值出错
    if (i === measure - 1) return max;
    return min + distance * i;
  });
});

defineEmits<{
  "plug-remove": [];
}>();
</script>
