<template>
  <slot :value></slot>
</template>

<script setup lang="ts" generic="T">
defineProps<{
  value: T;
}>();

defineSlots<{
  default: (value: { value: T }) => any;
}>();
</script>
