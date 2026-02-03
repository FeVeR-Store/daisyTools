<template>
  <div
    :class="[
      'badge',
      { 'cursor-pointer': $attrs.onClick },
      [type, style].map((tag) => (tag == 'default' ? '' : `badge-${tag}`)),
    ]"
  >
    <template v-if="icon">
      <Icon v-if="!loading" :path="icon === true ? icons[type] : icon"></Icon>
      <span v-else class="loading loading-spinner loading-md"></span>
    </template>
    <span><slot></slot></span>
  </div>
</template>

<script setup lang="ts">
import {
  mdiAlertCircleOutline,
  mdiCheckCircleOutline,
  mdiCloseCircleOutline,
  mdiCommentProcessingOutline,
  mdiInformationSlabCircleOutline,
} from "@mdi/js";
import Icon from "./Icon.vue";

export type AlertType = "default" | "info" | "success" | "warning" | "error";
export type AlertStyle = "default" | "soft" | "outline" | "dash";

const icons: { [type in AlertType]: string } = {
  default: mdiCommentProcessingOutline,
  info: mdiInformationSlabCircleOutline,
  success: mdiCheckCircleOutline,
  warning: mdiAlertCircleOutline,
  error: mdiCloseCircleOutline,
};

const { type = "default", Style: style = "default" } = defineProps<{
  type?: AlertType;
  Style?: AlertStyle;
  icon?: string | true;
  loading?: boolean;
}>();
</script>
