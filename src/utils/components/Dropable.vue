<template>
  <div
    @mouseenter="onMouseEnter"
    @mouseup="onMouseUp"
    @mouseleave="onMouseLeave"
    @mousemove="onMouseMove"
    :class="[
      'w-full',
      'h-full',
      isDraggover && dragoverClass,
      'relative',
      haveDragging && dropableClass,
    ]"
  >
    <slot :haveDragging> </slot>
    <Transition>
      <div
        v-if="isDraggover && $slots.enter"
        ref="enterElement"
        class="z-0 absolute top-0 left-0 w-full h-full"
      >
        <slot name="enter"></slot>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
/**
 * @prop {string | symbol} id - 唯一标识符，与`Draggable`组件配合使用
 * @prop {string} [dragoverClass] - 拖拽经过时的类名
 * @prop {string} [dropableClass] - 可放置区域的类名
 * @prop {function} [filter] - 过滤函数，用于判断拖拽数据是否可放置
 * ```
 */
import { computed, ref, useTemplateRef } from "vue";
import { useDraggable } from "./Draggable.vue";

export type Position = { x: number; y: number };
export type EnterEvent<T = any> = { data: T; position: Position };
export type MoveEvent<T = any> = { data: T; position: Position };
export type DropEvent<T = any> = { data: T; position: Position };

const { id, filter } = defineProps<{
  id: string | symbol;
  dragoverClass?: any;
  dropableClass?: any;
  filter?: (data: any) => boolean;
}>();

const enterElement = useTemplateRef("enterElement");
const dragState = useDraggable(id);

const isDraggover = ref<boolean>(false);

let position = ref({ x: 0, y: 0 });

const haveDragging = computed(() => {
  return (
    dragState.value?.haveDragging &&
    (filter ? filter(dragState.value?.data) : true)
  );
});

function onMouseEnter(e: MouseEvent) {
  if (!haveDragging.value) return;
  if (e.relatedTarget === enterElement.value) return;
  isDraggover.value = true;
  const rect = (e.target as Element).getBoundingClientRect();
  const x = e.clientX - rect.left;
  const y = e.clientY - rect.top;
  position.value = { x, y };
  emit("enter", { data: dragState.value?.data, position: position.value });
}
function onMouseUp() {
  if (!isDraggover.value) return;
  isDraggover.value = false;
  if (dragState.value!.data) {
    emit("drop", { data: dragState.value?.data, position: position.value });
  }
}
function onMouseMove(e: MouseEvent) {
  if (!isDraggover.value) return;
  position.value.x += e.movementX;
  position.value.y += e.movementY;
  emit("move", { data: dragState.value?.data, position: position.value });
}
function onMouseLeave(_e: MouseEvent) {
  if (isDraggover.value) {
    isDraggover.value = false;
    emit("leave");
  }
}

const emit = defineEmits<{
  drop: [DropEvent];
  enter: [EnterEvent];
  move: [MoveEvent];
  leave: [];
}>();
</script>

<style scoped></style>
