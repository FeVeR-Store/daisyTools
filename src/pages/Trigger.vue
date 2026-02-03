<script setup lang="ts" ge>
import { useTemplateRef } from "vue";
import { api } from "../invoke";
import { triggers } from "../invoke/triggers.ts";
import CardView from "../views/CardView.vue";

const { showDescriptionView = true } = defineProps<{
  showDescriptionView?: boolean;
  draggableId?: symbol;
}>();

const expose = useTemplateRef("cardView");

defineExpose({ expose });
</script>

<template>
  <CardView
    ref="cardView"
    type="trigger"
    :draggableId
    :cardMetas="triggers"
    :fetchLitCards="api.getLitTrigger"
    :showDescriptionView
  >
  </CardView>
</template>
