<template>
  <Teleport to="body">
    <Layer
      transition="opacity"
      v-model="show"
      :clickOutsideToClose
      center
      class="size-full"
      mask
      :wH
      :hW
      :aspectRatio
      :w
      :h
      :fitSize
      #="bind"
    >
      <div @click.stop class="card bg-base-100 modal-view" :class="{ scale }">
        <slot v-bind="bind"> </slot>
      </div>
    </Layer>
  </Teleport>
</template>

<script lang="ts">
export let modalNumber = 0;
</script>

<script setup lang="ts">
import { computed, onBeforeMount, ref, watch } from "vue";
import Layer from "../utils/components/Layer.vue";

const depth = ref(modalNumber);

const { clickOutsideToClose = true, size = "lg" } = defineProps<{
  aspectRatio?: [width: number, height: number] | string;
  wH?: [width: number, height: number] | string;
  hW?: [width: number, height: number] | string;
  "aspectRatio:fixed"?: [width: number, height: number] | string;
  "wH:fixed"?: [width: number, height: number] | string;
  "hW:fixed"?: [width: number, height: number] | string;
  w?: string | number;
  h?: string | number; 
  clickOutsideToClose?: boolean;
  size?: "sm" | "md" | "lg";
  fitSize?: boolean;
}>();

const show = defineModel<boolean>({ default: false });
const scale = ref(false);
const scaleValue = computed(() => {
  switch (size) {
    case "sm":
      return 0.4;
    case "md":
      return 0.6;
    case "lg":
      return 0.8;
  }
});

const heightScale = computed(() => scaleValue.value ** depth.value);

onBeforeMount(() => {
  depth.value++;
  modalNumber++;
});

watch(
  show,
  (val) => {
    if (!val) {
      depth.value--;
      modalNumber--;
    } else {
      depth.value++;
      modalNumber++;
    }
  },
  { immediate: true }
);
</script>

<style scoped>
:global(.modal-view) {
  --view-height: calc(99vh * v-bind(heightScale));
}
</style>
