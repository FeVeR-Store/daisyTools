<template>
  <select v-if="!plug" v-bind="$attrs" v-model="value" class="select w-full">
    <option v-if="!$props.default && placeholder" value="" disabled selected>
      {{ placeholder }}
    </option>
    <option
      v-for="{ label, value } in data"
      :value
      :selected="value === $props.default"
    >
      {{ label }}
    </option>
  </select>
  <PlugDisplay
    v-else
    @remove="$emit('plug-remove')"
    :plug
    display-style="select"
  ></PlugDisplay>
</template>

<script setup lang="ts">
import { usePlug } from "./PlugDisplay.utils";
import PlugDisplay from "./PlugDisplay.vue";

defineProps<{
  placeholder?: string;
  default?: string;
  plug?: string[];
  data: { label: string; value: string }[];
}>();

const value = defineModel();
const { plug } = usePlug(value);

defineEmits<{
  "plug-remove": [];
}>();
</script>
