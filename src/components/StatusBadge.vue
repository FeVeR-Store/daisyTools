<template>
  <span :class="`badge ${colorClass} gap-1 px-2`">
    {{ status }}
  </span>
</template>

<script setup lang="ts">
import { computed } from "vue";

const { status } = defineProps<{
  status: "running" | "paused" | "failed";
}>();

const colorClass = computed(() => {
  switch (status) {
    case "running":
      return "badge-success";
    case "paused":
      return "badge-warning";
    case "failed":
      return "badge-error";
    default:
      return "";
  }
});
</script>
