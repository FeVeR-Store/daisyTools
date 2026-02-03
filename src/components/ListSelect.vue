<template>
  <ul class="menu bg-primary rounded-box w-max min-w-50 menu-md relative">
    <li
      v-for="item in items"
      :key="item.id"
      class="bg-primary rounded"
      v-event-delegation:[item].click="onSelect"
      :class="[
        !active || item.id === active ? 'show' : 'hide',
        'transition-all duration-700',
      ]"
    >
      <div class="inline-flex">
        <button
          class="btn btn-circle btn-ghost btn-primary btn-xs transition-transform"
        >
          <Icon
            size="16"
            :path="!active ? mdiPowerPlug : mdiArrowULeftTop"
          ></Icon>
        </button>
        <a>{{ item.label }}</a>
      </div>
    </li>
  </ul>
</template>

<script setup lang="ts">
import { vEventDelegation } from "../directives/eventDelegation";
import Icon from "./Icon.vue";
import { mdiArrowULeftTop, mdiPowerPlug } from "@mdi/js";

defineProps<{
  items: { id: string; label: string }[];
}>();

const active = defineModel<string | null>();

const onSelect = async ({ id }: { id: string; label: string }) => {
  active.value = !active.value ? id : null;
};
</script>

<style scoped>
.hide {
  position: absolute;
  top: 0;
  left: 0;
  opacity: 0;
  z-index: -1;
}

.show {
  opacity: 1;
}
</style>
