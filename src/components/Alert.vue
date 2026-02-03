<!-- include: alert-info | alert-success | alert-warning | alert-error -->
<!-- include: alert-soft | alert-outline | alert-dash -->
 
<template>
  <div
    :class="[
      'alert',
      [type, style].map((tag) => (tag == 'default' ? '' : `alert-${tag}`)),
    ]"
  >
    <Icon v-if="!loading" :path="icon ?? icons[type]"></Icon>
    <span v-else class="loading loading-spinner loading-md"></span>
    <span><slot></slot></span>
    <slot name="action"></slot>
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

const { type = "default", style = "default" } = defineProps<{
  type?: AlertType;
  style?: AlertStyle;
  icon?: string;
  loading?: boolean;
}>();
</script>
