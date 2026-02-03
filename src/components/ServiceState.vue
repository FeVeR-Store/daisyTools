<template>
  <Switch
    data-tauri-drag-region
    class="m-auto w-fit h-full scale-65 text-2xl rounded-full px-8 py-1 pointer-events-auto"
    :duration="1000"
    :cases="<const>[...serviceState, 'Unkown']"
    :value="state"
  >
    <template #Unkown>
      <Alert loading>{{ t("Unkown") }}</Alert>
    </template>
    <template #Running>
      <Alert type="success">{{ t("Running") }}</Alert>
    </template>
    <template #Stopped>
      <Alert @click="launch" type="warning">{{ t("Stopped") }}</Alert>
    </template>
    <template #StartPending>
      <Alert type="info" loading>{{ t("StartPending") }}</Alert>
    </template>
    <template #PausePending>
      <Alert type="info" loading>{{ t("PausePending") }}</Alert>
    </template>
    <template #Paused>
      <Alert type="warning">{{ t("Paused") }}</Alert>
    </template>
    <template #ContinuePending>
      <Alert class="bbbb" type="info" loading>{{ t("ContinuePending") }}</Alert>
    </template>
    <template #StopPending>
      <Alert type="info" loading>{{ t("StopPending") }}</Alert>
    </template>
  </Switch>
</template>

<script setup lang="ts">
import { api } from "../invoke";
import { onBeforeMount, onUnmounted, ref } from "vue";
import Switch from "../utils/components/Switch.vue";
import { serviceState } from "../invoke/serviceState";
import Alert from "../components/Alert.vue";
import { watch } from "@tauri-apps/plugin-fs";
import { useI18n } from "vue-i18n";

const { t } = useI18n({});
type ServiceState = Awaited<ReturnType<typeof api.getServiceState>> | "Unkown";

let unwatch: () => void;

const state = ref<ServiceState>("Unkown");

onBeforeMount(async () => {
  state.value = await api.getServiceState();
  const file = await api.getServiceStateFile();
  console.log(`watch ${file}`);
  unwatch = await watch(
    file,
    async () => {
      console.log("watch ");
      state.value = await api.getServiceState();
    },
    { recursive: false }
  );
});

onUnmounted(() => {
  unwatch?.();
});

async function launch() {
  await api.launchService();
  state.value = "StartPending";
}
</script>

<i18n lang="yaml">
zh-CN:
  Unkown: 正在获取服务状态...
  Running: 服务正在运行
  Stopped: 服务已停止
  StartPending: 服务正在启动
  PausePending: 服务正在暂停
  Paused: 服务已暂停
  ContinuePending: 服务正在继续
  StopPending: 服务正在停止

en:
  Unkown: Getting service state...
  Running: Service is running
  Stopped: Service is stopped
  StartPending: Service is starting
  PausePending: Service is pausing
  Paused: Service is paused
  ContinuePending: Service is continuing
  StopPending: Service is stopping
</i18n>
