<template>
  <div class="markdown-body">
    <Alert v-if="isError" type="error"> Markdown解析出现错误 </Alert>
    <div v-html="html"></div>
  </div>
</template>

<style>
@reference "../main.css";
.markdown-body {
  h1 {
    @apply text-2xl font-bold;
  }
  h2 {
    @apply text-xl font-bold;
  }
  h3 {
    @apply text-lg font-bold;
  }
  li::marker {
    unicode-bidi: isolate;
    font-variant-numeric: tabular-nums;
    text-transform: none;
    text-indent: 0px !important;
    text-align: start !important;
    text-align-last: auto !important;
  }
}
</style>

<script setup lang="ts">
import { computed, ref } from "vue";
import markdownit from "markdown-it";
import Alert from "./Alert.vue";
const md = markdownit({
  breaks:true,
  html: true,
});
const { content = "" } = defineProps<{
  content: string;
}>();
const isError = ref(false);

const html = computed(() => {
  try {
    return md.render(content.replace(/\n\n/g, "\n<br>\n\r"));
  } catch (error) {
    isError.value = true;
    console.error(error);
    return error;
  }
});
</script>
