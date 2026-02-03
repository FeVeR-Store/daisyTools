<template>
  <template v-for="(ele, i) in list" :key="i">
    <slot :element="ele" :index="i">
      {{ ele }}
    </slot>
    <slot v-if="i !== list.length - 1" name="separator">
      {{ separator }}
    </slot>
  </template>
</template>

<script setup lang="ts" generic="T extends unknown[]">
defineProps<{
  list: T;
  separator?: string;
}>();
</script>
