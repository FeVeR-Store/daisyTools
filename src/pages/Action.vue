<script setup lang="ts" generic="T">
import { provide, useTemplateRef } from "vue";
import { api } from "../invoke";

import CardView from "../views/CardView.vue";
import { actions } from "../invoke/actions.ts";

const { selectMode, showDescriptionView = true } = defineProps<{
  selectMode?: boolean;
  showDescriptionView?: boolean;
  draggableId?: symbol;
}>();
provide("selectAction", selectMode);

const expose = useTemplateRef("cardView");

defineExpose({ expose });
</script>

<template>
  <CardView
    ref="cardView"
    type="action"
    :draggableId
    :cardMetas="actions"
    :fetchLitCards="api.getLitAction"
    :showDescriptionView
  >
  </CardView>
</template>
