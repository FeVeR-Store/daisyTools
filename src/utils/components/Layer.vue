<template>
  <Teleport to="#layer-container">
    <Transition :name="transition">
      <div
        v-if="show"
        @click="clickToClose && (show = false)"
        class="w-screen h-screen fixed top-12 left-0 transition-all flex transform"
        :class="[
          show ? 'opacity-100' : 'opacity-0 pointer-events-none',
          mask ? 'bg-black/50' : 'pointer-events-none',
        ]"
      >
        <div
          v-on-click-outside="clickOutside"
          @click.stop
          class="pointer-events-auto layer"
          :class="[center ? 'm-auto relative' : 'absolute', $attrs.class]"
          :style="{
            ...size,
          }"
        >
          <slot :close="() => (show = false)"></slot>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script lang="ts" setup>
import { computed, Transition } from "vue";
import { vOnClickOutside } from "@vueuse/components";

const defaultRatio = [2, 1];

const {
  center = false,
  mask = false,
  clickOutsideToClose: clickToClose = false,
  aspectRatio,
  wH: w_h,
  hW: h_w,
  w = "66.6667%",
  h = "auto",
  top = "unset",
  right = "unset",
  bottom = "unset",
  left = "unset",
  fitSize = true,
} = defineProps<{
  transition?:
    | "opacity"
    | "x-slide"
    | "-x-slide"
    | "l-r"
    | "r-l"
    | "y-slide"
    | "-y-slide"
    | "t-b"
    | "b-t";
  clickOutsideToClose?: boolean;
  center?: boolean;
  mask?: boolean;
  aspectRatio?: [width: number, height: number] | string;
  wH?: [width: number, height: number] | string;
  hW?: [width: number, height: number] | string;
  w?: string | number;
  h?: string | number;
  top?: string | number;
  right?: string | number;
  bottom?: string | number;
  left?: string | number;
  fitSize?: boolean;
}>();

const show = defineModel<boolean>({ required: true });

const size = computed(() => {
  const size: {
    width: string | number;
    height: string | number;
    top: string | number;
    right: string | number;
    bottom: string | number;
    left: string | number;
    maxWidth?: string;
    maxHeight?: string;
  } = {
    width: "auto",
    height: "auto",
    top: "unset",
    right: "unset",
    bottom: "unset",
    left: "unset",
  };
  const ratio =
    parseAspectRatio(aspectRatio ?? w_h) ??
    parseAspectRatio(h_w, true) ??
    defaultRatio;
  size.width = parseSize(w);
  size.height = parseSize(h);
  size.top = parseSize(top);
  size.right = parseSize(right);
  size.bottom = parseSize(bottom);
  size.left = parseSize(left);
  if (w === "auto" && size.height) {
    size[fitSize ? "maxWidth" : "width"] = `calc(${size.height} * ${
      ratio[0] / ratio[1]
    })`;
  }
  if (h === "auto" && size.width) {
    size[fitSize ? "maxHeight" : "height"] = `calc(${size.width} * ${
      ratio[1] / ratio[0]
    })`;
  }
  return size;
});
function clickOutside() {
  emit("clickOutside");
}

const emit = defineEmits<{
  clickOutside: [];
}>();

function parseAspectRatio(
  aspectRatio: [number, number] | string | undefined,
  reverse = false
): [number, number] | null {
  if (!aspectRatio) {
    return null;
  }
  let ratio: [number, number];
  if (Array.isArray(aspectRatio)) {
    ratio = aspectRatio;
  } else {
    const [width, height] = aspectRatio.split("/").map(Number);
    ratio = [width, height];
  }
  if (reverse) {
    return [ratio[1], ratio[0]];
  }
  return ratio;
}

function parseSize(
  size: number | string,
  numberFormat: (size: number) => string = (size) => `${size * 0.25}rem`
) {
  if (
    typeof size === "number" ||
    (typeof size === "string" && size.match(/\d+$/))
  ) {
    return numberFormat(+size);
  } else {
    return size;
  }
}
</script>

<style scoped>
:global(.layer > div) {
  width: 100%;
  height: 100%;
}
</style>
