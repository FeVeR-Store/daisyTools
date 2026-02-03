<template>
  <!-- <div
    class="size-full"
    ref="draggableContainer"
    @mousedown.stop="mouseDownListener"
    @mouseup="dragging = false"
    @click="emit('click', $event)"
  > -->
  <component :is="defaultSlot" ref="draggableContainer"></component>
  <!-- <slot :dragging> </slot> -->
  <!-- </div> -->
  <Teleport to="body">
    <div v-if="dragging" class="fixed pointer-events-none" :style>
      <slot name="dragging">
        <div v-if="dragging" ref="draggingElement"></div>
      </slot>
    </div>
  </Teleport>
</template>

<script lang="ts">
export function useDraggable(id: string | symbol) {
  const state = useStore(id, { haveDragging: false, data: null as any });
  return state;
}
</script>

<script setup lang="ts">
import {
  computed,
  h,
  onMounted,
  onUnmounted,
  reactive,
  ref,
  useTemplateRef,
  VNode,
  watchEffect,
} from "vue";
import { useStore } from "../../composable/useScopeStore";

const {
  disable = false,
  id = Symbol(),
  data,
} = defineProps<{
  disable?: boolean;
  id?: string | symbol;
  data: unknown;
}>();

const slot = defineSlots<{
  default: (dragging: boolean) => VNode[];
  dragging: () => VNode[];
}>();

const defaultSlot = computed(() => {
  const vnodes = slot.default(dragging.value);
  if (vnodes.length === 0) {
    return vnodes;
  } else if (vnodes.length !== 1) {
    vnodes.unshift(h("div", vnodes));
    vnodes.length = 1;
  }
  vnodes[0].props ??= {};
  vnodes[0].props.onMousedown = (e: MouseEvent) => {
    mouseDownListener(e);
    e.stopPropagation();
  };
  vnodes[0].props.onMouseup = () => {
    dragging.value = false;
  };
  vnodes[0].props.onClick = (e: Event) => {
    emit("click", e);
  };
  return vnodes[0];
});

const dragState = useDraggable(id);

const clicked = ref(false);
const dragging = ref(false);

const draggingElementReady = ref(false);
const draggableContainer = useTemplateRef("draggableContainer");
const draggingElement = useTemplateRef("draggingElement");

const draggingContainerStyle = reactive({ top: 0, left: 0 });
const style = computed(() => {
  const { top, left } = draggingContainerStyle;
  return { top: 0, left: 0, transform: `translate(${left}px, ${top}px)` };
});

const mouseDownListener = (e: MouseEvent) => {
  if (disable || dragState.value?.haveDragging) return;
  const { clientX, clientY } = e;
  draggingContainerStyle.left = clientX;
  draggingContainerStyle.top = clientY;
  clicked.value = true;
};

const mouseMoveListener = (e: MouseEvent) => {
  // 禁用则返回
  if (disable) return;
  // 如果当前有其他组件正在拖拽则返回
  if (!dragging.value && dragState.value?.haveDragging) return;
  // 如果鼠标左键未按下则重置状态并返回
  if (e.buttons === 0) {
    dragging.value = false;
    clicked.value = false;
    return;
  }
  if (clicked.value) {
    dragging.value = true;
  }
  const { clientX, clientY } = e;
  draggingContainerStyle.left = clientX;
  draggingContainerStyle.top = clientY;
};

watchEffect(() => {
  if (disable || !dragging.value) {
    draggingContainerStyle.top = 0;
    draggingContainerStyle.left = 0;
  } else {
    if (!draggingElementReady.value && draggingElement.value) {
      draggingElement.value.append(
        (draggableContainer.value as HTMLElement).cloneNode(true)
      );
      draggingElementReady.value = true;
    }
  }
  if (dragging.value) {
    dragState.value!.data = data;
  }
  dragState.value!.haveDragging = dragging.value;
});

const listenerReady = ref(false);

const addMouseMoveListener = () => {
  if (disable || listenerReady.value) return;
  window.addEventListener("mousemove", mouseMoveListener);
  listenerReady.value = true;
};

watchEffect(addMouseMoveListener);
onMounted(addMouseMoveListener);

onUnmounted(() => {
  if (disable || listenerReady) return;
  window.removeEventListener("mousemove", mouseMoveListener);
});

const emit = defineEmits<{
  click: [any];
}>();
</script>
