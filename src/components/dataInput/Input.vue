<template>
  <input
    v-if="!plug"
    :type
    class="input w-full"
    :placeholder
    v-bind="$attrs"
    v-model="value"
  />
  <PlugDisplay
    @remove="$emit('plug-remove')"
    :plug
    display-style="input"
  ></PlugDisplay>
</template>

<script setup lang="ts">
import { usePlug } from "./PlugDisplay.utils";
import PlugDisplay from "./PlugDisplay.vue";

const { type = "text" } = defineProps<{
  type?: "text" | "number";
  placeholder?: string;
  plug?: string[];
}>();

const value = defineModel();
const { plug } = usePlug(value);

defineEmits<{
  "plug-remove": [];
}>();
</script>
