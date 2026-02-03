<template>
  <form class="filter" :class="{ 'gap-5': !modelValue }">
    <div v-if="modelValue" style="line-height: 40px">
      <slot :value="modelValue"> </slot>
    </div>
    <input
      @click="modelValue = ''"
      class="btn btn-square"
      type="reset"
      value="×"
    />
    <input
      v-for="option in options"
      @click="modelValue = option"
      :key="option"
      class="btn btn-primary"
      :class="{ 'btn-soft': modelValue !== option }"
      type="radio"
      :name="option"
      :aria-label="option"
      :checked="modelValue === option"
    />
    <div>
      <!-- 占位用 -->
    </div>
  </form>
</template>

<script setup lang="ts">
defineProps<{
  options: string[];
}>();
const modelValue = defineModel<string>({ required: true });
</script>
