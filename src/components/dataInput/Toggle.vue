<template>
  <input v-if="!plug" class="toggle" type="checkbox" v-model="value" />

  <PlugDisplay
    v-else
    @remove="$emit('plugRemove')"
    :plug="plug"
    display-style="toggle"
  ></PlugDisplay>
</template>

<script setup lang="ts">
import { usePlug } from "./PlugDisplay.utils";
import PlugDisplay from "./PlugDisplay.vue";

defineProps<{
  plug?: string[];
}>();
const value = defineModel();
const { plug } = usePlug(value);

defineEmits<{
  plugRemove: [];
}>();
</script>
