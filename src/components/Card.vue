<template>
  <div @click="$emit('click')" class="card w-full p-4 relative group">
    <span
      v-if="title"
      class="text-4xl font-bold p-5 group-hover:text-5xl transition-all"
      >{{ title }}</span
    >
    <slot></slot>
    <Icon
      class="absolute right-4 bottom-4 opacity-45 group-hover:scale-85 transition-all"
      size="150"
      :path="icon"
    ></Icon>
  </div>
</template>

<script setup lang="ts">
import Icon from "./Icon.vue";

defineProps<{
  title?: string;
  icon: string;
}>();

defineEmits<{
  click: [];
}>();
</script>
