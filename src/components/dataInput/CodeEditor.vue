<template>
  <PlugDisplay @remove="$emit('plug-remove')" :plug display-style="code">
    <template #onlyForCodeEditor>
      <CodeMirror
        v-if="!plug"
        :extensions="[basicSetup, javascript(), oneDark]"
        :readonly="!!plug || readonly"
        v-model="(moduleValue as string)"
        :placeholder
        :tab
        :allowMultipleSelections
        :disabled
      ></CodeMirror>
      <CodeMirror
        v-else
        :modelValue="'\n\n\n'"
        :extensions="[basicSetup, oneDark]"
        :readonly="!!plug || readonly"
        disabled
      ></CodeMirror>
    </template>
  </PlugDisplay>
</template>

<script setup lang="ts">
import { javascript } from "@codemirror/lang-javascript";
import { oneDark } from "@codemirror/theme-one-dark";
import { basicSetup } from "codemirror";
import CodeMirror from "vue-codemirror6";
import { Plug, usePlug } from "./PlugDisplay.utils";
import PlugDisplay from "./PlugDisplay.vue";
defineProps<{
  readonly?: boolean;
  placeholder?: string;
  tab?: boolean;
  allowMultipleSelections?: boolean;
  disabled?: boolean;
}>();

const moduleValue = defineModel<string | Plug>();

const { plug } = usePlug(moduleValue);

defineEmits<{
  "plug-remove": [];
}>();
</script>
