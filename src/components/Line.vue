<template>
  <ul class="timeline">
    <li v-for="(item, i) in items" class="w-[50%]">
      <div class="timeline-middle">
        <Icon :path="getIcon(i)"></Icon>
      </div>
      <div class="timeline-end" >{{ item }}</div>
      <hr v-if="i < items.length - 1" style="width: 200%;" class="bg-primary" />
    </li>
  </ul>
</template>

<script setup lang="ts">
import {
  mdiCheckCircle,
  mdiNumeric1Circle,
  mdiNumeric2Circle,
  mdiNumeric3Circle,
  mdiNumeric4Circle,
  mdiNumeric5Circle,
  mdiNumeric6Circle,
  mdiNumeric7Circle,
  mdiNumeric8Circle,
  mdiNumeric9Circle,
} from "@mdi/js";
import Icon from "./Icon.vue";

function getIcon(i: number) {
  switch (i) {
    case 0:
      return mdiNumeric1Circle;
    case 1:
      return mdiNumeric2Circle;
    case 2:
      return mdiNumeric3Circle;
    case 3:
      return mdiNumeric4Circle;
    case 4:
      return mdiNumeric5Circle;
    case 5:
      return mdiNumeric6Circle;
    case 6:
      return mdiNumeric7Circle;
    case 7:
      return mdiNumeric8Circle;
    case 8:
      return mdiNumeric9Circle;
    default:
      return mdiCheckCircle;
  }
}

defineProps<{
  items: string[];
}>();
</script>
