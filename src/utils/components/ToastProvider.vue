<template>
  <Teleport to="body">
    <TransitionGroup
      name="toast"
      tag="div"
      class="toast toast-top toast-center z-1000"
    >
      <div class="" v-for="toast in toastList" :key="toast.id">
        <Alert :type="toast.type" :style="toast.style"
          >{{ toast.message }}
        </Alert>
      </div>
    </TransitionGroup>
  </Teleport>
</template>

<script setup lang="ts">
import { bus } from "../../bus";
import { ref } from "vue";
import Alert, { AlertStyle, AlertType } from "../../components/Alert.vue";

type Toast = {
  type: AlertType;
  style: AlertStyle;
  duration: number;
  message: string;
  id?: string;
};
const toastList = ref<Toast[]>([]);

const showToast = (message: string, option: ToastOption = {}) => {
  const toastItem: Toast = {
    message,
    type: "default",
    style: "default",
    duration: 3000,
    id: Math.random().toString(16),
    ...option,
  };
  toastList.value.push(toastItem);
  const toastId = toastItem.id;
  setTimeout(() => {
    toastList.value = toastList.value.filter((toast) => toast.id !== toastId);
  }, toastItem.duration);
};

bus.on("toast:show", ([message, option]) => {
  console.log(message);
  showToast(message, option);
});
</script>

<script lang="ts">
export type ToastOption = {
  type?: AlertType;
  style?: AlertStyle;
  duration?: number;
};

export type ToastTools = {
  [type in AlertType]: (message: string, option?: ToastOption) => void;
} & {
  (message: string, option?: ToastOption): void;
};

// @ts-ignore
export const toast: ToastTools = (message: string, option: ToastOption) => {
  bus.emit("toast:show", [message, option]);
};

(<const>["default", "info", "success", "warning", "error"]).forEach((type) => {
  toast[type] = (message, option) => {
    toast(message, { ...option, type });
  };
});
</script>

<style>
.toast-move,
.toast-enter-active,
.toast-leave-active {
  transition: all 0.5s ease;
}
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
}
.toast-leave-active {
  position: absolute;
}
</style>
