<template>
  <div role="tablist" class="tabs tabs-border">
    <a
      @click="$emit('update:modelValue', tab)"
      role="tab"
      class="tab"
      v-for="tab in tabs"
      :key="tab"
      :class="{ 'tab-active': modelValue === tab }"
      >{{ tab }}</a
    >
  </div>
  <template v-for="tab in tabs" :key="tab">
    <slot v-if="modelValue === tab" :name="tab"></slot>
  </template>
</template>

<script setup lang="ts" generic="const T extends string[]">
defineProps<{
  tabs: T;
}>();

defineSlots<{
  [tab in T[number]]: () => any;
}>();
const modelValue = defineModel<T[number]>({ required: true });
</script>
