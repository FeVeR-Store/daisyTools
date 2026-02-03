<script setup lang="ts">
import { mdiHelp, mdiHelpCircleOutline } from "@mdi/js";
import Icon from "./Icon.vue";
import Layer from "../utils/components/Layer.vue";
import { ref } from "vue";
import Alert from "./Alert.vue";
import Markdown from "./Markdown.vue";
import { useI18n } from "vue-i18n";
const showHelpView = ref(false);
const { triggerType = "default" } = defineProps<{
  title: { [lang: string]: string };
  triggerType?: "icon" | "default";
  markdownContent?: { [lang: string]: string };
}>();
const { t, locale } = useI18n();
const show = () => {
  showHelpView.value = true;
};
</script>

<template>
  <slot :show name="trigger">
    <button
      v-if="triggerType === 'default'"
      @click="show"
      class="btn btn-primary btn-xs rounded-3xl"
    >
      <Icon :size="16" :path="mdiHelp"></Icon>
      {{ t("help") }}
    </button>
    <button
      v-else-if="triggerType === 'icon'"
      class="btn btn-ghost btn-primary btn-circle btn-xs"
      @click="show"
    >
      <Icon size="20" :path="mdiHelpCircleOutline"></Icon>
    </button>
  </slot>
  <Transition name="opacity">
    <Layer
      transition="opacity"
      mask
      clickOutsideToClose
      v-model="showHelpView"
      center
      bottom="10"
      w-h="4/4"
    >
      <div class="card bg-base-100 rounded-lg p-4 size-full">
        <Alert type="info" class="text-base font-medium">
          {{ t("is_about") }}
          <span class="font-bold">{{ title[locale] }}</span>
          {{ t("help_page") }}
        </Alert>
        <article class="overflow-y-auto max-h-[50vh] p-4">
          <slot></slot>
          <Markdown
            v-if="markdownContent"
            :content="markdownContent[locale]"
          ></Markdown>
        </article>
      </div>
    </Layer>
  </Transition>
</template>

<i18n lang="yaml">
zh-CN:
  help: 帮助
  is_about: 这是关于
  help_page: 的帮助页面
en:
  help: Help
  is_about: This is about
  help_page: help page
</i18n>
