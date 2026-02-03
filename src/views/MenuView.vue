<template>
  <div class="grid grid-cols-[auto_1fr] gap-4 h-full">
    <Menu
      v-model:active="active"
      class="overflow-y-auto"
      :draggableId
      :path
      :router-method
      :items
    ></Menu>
    <div class="transform h-[var(--view-height)]">
      <!-- <Transition name="opacity"> -->
      <slot v-if="!hideView"></slot>
      <!-- </Transition> -->
    </div>
  </div>
</template>

<script setup lang="ts" generic="T">
import Menu, { MenuItem } from "../components/Menu.vue";

const active = defineModel<string | null>("active", { default: null });
withDefaults(
  defineProps<{
    items: MenuItem | MenuItem[];
    routerMethod?: "push" | "replace";
    path?: string;
    hideView?: boolean;
    draggableId?: symbol;
  }>(),
  { routerMethod: "replace", hideView: false }
);
</script>
