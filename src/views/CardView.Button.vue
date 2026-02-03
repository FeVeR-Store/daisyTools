<template>
  <div class="join absolute right-4 bottom-4 shadow-2xl">
    <Switch :value :cases>
      <template #selectAction>
        <button @click="selectAction" class="rounded-4xl btn btn-primary">
          <Icon :path="mdiCheck"></Icon>
          {{ t("action.select") }}
          <Modal v-model="showSelectActionModal" title="123"> <Task /> </Modal>
        </button>
      </template>
      <template #addAction>
        <button @click="selectAction" class="rounded-4xl btn btn-primary">
          <Icon :path="mdiPlus"></Icon>
          {{ t("action.add") }}
          <Modal v-model="showSelectActionModal" title="123">
            <Action selectMode></Action>
          </Modal>
        </button>
      </template>
      <template #runAction>
        <button @click="runAction" class="rounded-4xl btn btn-primary">
          <Icon :path="mdiPlay"></Icon>
          {{ t("action.run.title") }}
        </button>
      </template>
      <template #litCard>
        <div class="join">
          <input
            v-model="name"
            class="input join-item"
            :placeholder="t('lit.placeholder')"
            :class="['rounded-l-4xl pl-5', { 'input-primary': name }]"
          />
          <button
            :disabled="!name"
            @click="litCard(card!.getData())"
            class="btn join-item rounded-r-full"
          >
            {{ t("lit.title") }}
          </button>
        </div>
      </template>
    </Switch>
  </div>
  <div v-if="litCardInfo" class="absolute left-64 bottom-4 shadow-2xl">
    <button
      @click="showDeleteModal = true"
      class="rounded-4xl group btn btn-error"
    >
      <Icon :path="mdiClose"></Icon>
      <span class="transition-all duration-100"> {{ t("delete.title") }} </span>
      <Modal size="sm" v-model="showDeleteModal" title="123">
        <div class="card-body p-8">
          <h2 class="card-title text-2xl">{{ t("delete.warning.ask") }}</h2>
          <p>{{ t("delete.warning.description") }}</p>
          <div class="card-actions justify-end">
            <button @click="removeCard" class="btn btn-error rounded-4xl">
              {{ t("delete.comfirm") }}
            </button>
          </div>
        </div>
      </Modal>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { api } from "../invoke";
import { toast } from "../utils/components/ToastProvider.vue";
import { CardMeta, Data, LitCard } from "../invoke/type";
import { mdiCheck, mdiClose, mdiPlay, mdiPlus } from "@mdi/js";
import Icon from "../components/Icon.vue";
import Modal from "../components/Modal.vue";
import Action from "../pages/Action.vue";
import Switch from "../utils/components/Switch.vue";
import Task from "../components/Task.vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const name = ref<string>("");

const showSelectActionModal = ref(false);
const showDeleteModal = ref(false);

const emit = defineEmits<{
  lit: [id: string];
  remove: [];
}>();

const cases = (<const>[
  "selectAction",
  "addAction",
  "runAction",
  "litCard",
]) satisfies string[];

export type CardButtonType = (typeof cases)[number];

const { cardInfo, litCardInfo, id } = defineProps<{
  cardInfo: CardMeta;
  litCardInfo?: LitCard | null;
  value: CardButtonType;
  id: string;
  card: { getData: () => Data } | null;
}>();

async function litCard(_data: Data | Promise<Data> | false | Promise<false>) {
  const data = await _data;
  if (!data) return;
  const cardName = cardInfo.name;
  try {
    const id = await api[
      cardName.endsWith("_action") ? "registerAction" : "registerTrigger"
    ](cardInfo.name as any, name.value, data as any);
    toast.success(t("lit.success"));
    emit("lit", id);
  } catch (e) {
    console.error(e);
    toast.error(t("lit.failed") + e);
  }
}

async function removeCard() {
  const cardName = cardInfo.name;
  try {
    await api[cardName.endsWith("_action") ? "removeAction" : "removeTrigger"](
      litCardInfo!.id
    );
    toast.success(t("delete.success"));
    showDeleteModal.value = false;
    emit("remove");
  } catch (e) {
    console.error(e);
    toast.error(t("delete.failed"));
    showDeleteModal.value = false;
  }
}
async function runAction() {
  try {
    await api.runActionById(id);
  } catch (e) {
    console.error(e);
    toast.error(t("action.run.failed") + e);
  }
}
function selectAction() {
  // const workflow = new Window("workflow");
  // workflow.once("tauri://window-created", async () => {
  //   const webview = new Webview(workflow, "workflow", {
  //     url: "/task",
  //     x: 0,
  //     y: 0,
  //     height: 800,
  //     width: 600,
  //     devtools: true,
  //   });
  //   webview.once("tauri://error", function (e) {
  //     // an error happened creating the webview
  //     console.log("webview error", e);
  //   });
  // });
  // workflow.once("tauri://error", function (e) {
  //   console.log("FAIL");
  //   console.log(e);
  // });
  return api.openWindow();

  // showSelectActionModal.value = true;
}
</script>

<i18n lang="yaml">
zh-CN:
  action:
    select: 选择该行动
    add: 添加行动
    run:
      title: 执行行动
      failed: 执行行动失败：
  lit:
    title: 点亮
    success: 点亮卡片成功!
    failed: 点亮卡片失败：
    placeholder: 请输入卡片名称
  delete:
    title: 删除该卡片
    warning:
      ask: 确定要删除这个卡片吗
      description: 删除后对应的任务同样将被移除, 请谨慎操作
    comfirm: 没得商量
    success: 删除卡片成功!
    failed: 删除卡片失败：

en:
  action:
    select: Select this action
    add: Add this action
    run:
      title: Run this action
      failed: "Failed to run this action:"
  lit:
    title: Lit this card
    success: Card lit successfully!
    failed: "Failed to lit up the card:"
    placeholder: Please enter the card name
  delete:
    title: Delete This card
    warning:
      ask: Are you sure you want to delete this card?
      description: The corresponding tasks will also be removed after deletion please proceed with caution.
    comfirm: No discussion
    success: Card deleted successfully!
    failed: "Failed to delete the card:"
</i18n>
