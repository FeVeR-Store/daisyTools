<template>
  <div
    class="radial-progress text-primary"
    :style="{
      '--value': percentage,
      '--size': '3.5rem',
      '--thickness': '6px'
    }"
    role="progressbar"
  >
    {{ percentage }}%
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{ value: number }>()
const percentage = computed(() => Math.round(props.value * 100))
</script>