<template>
  <div class="h-12">
    <slot name="left"></slot>
  </div>
  <div class="join">
    <div
      @click="handleClick"
      v-for="([handleClick, path], i) in btns"
      class="btn btn-ghost h-12 join-item"
      :class="i === 2 && 'btn-error'"
    >
      <Icon :path class="scale-80"></Icon>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  mdiWindowClose,
  mdiWindowMaximize,
  mdiWindowMinimize,
  mdiWindowRestore,
} from "@mdi/js";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { computed, ref, toValue } from "vue";
import Icon from "./Icon.vue";

const currentWindow = getCurrentWindow();

// 图标
const icons = {
  mdiWindowClose,
  mdiWindowMaximize,
  mdiWindowMinimize,
  mdiWindowRestore,
};

// 当前窗口的状态
const restoreOrMaximize = ref(
  (await currentWindow.isMaximized()) ? "Restore" : "Maximize"
);

// 监听窗口大小变化，以判断当前窗口是否最大化
currentWindow.onResized(async () => {
  restoreOrMaximize.value = (await currentWindow.isMaximized())
    ? "Restore"
    : "Maximize";
});

// 按钮
const btns = computed(() =>
  ["Minimize", restoreOrMaximize, "Close"].map(
    (state) =>
      [
        () => changeWindowState(toValue(state)), // 行为
        icons[("mdiWindow" + toValue(state)) as keyof typeof icons], // 图标
      ] as const
  )
);

// 改变窗口状态
async function changeWindowState(state: string) {
  switch (state) {
    case "Maximize":
    case "Minimize":
    // @ts-ignore switch 语句中的 Fallthrough 情况。ts-plugin(7029)
    case "Close":
      currentWindow[state.toLowerCase() as "close"]();
    case "Maximize":
      restoreOrMaximize.value = "Restore";
      break;

    case "Restore": {
      currentWindow.unmaximize();
      restoreOrMaximize.value = "Maximize";
    }
  }
}
</script>
