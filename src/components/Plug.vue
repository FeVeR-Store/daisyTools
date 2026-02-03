<template>
  <Teleport to="#app">
    <div
      @click="showPlugView = true"
      class="bg-primary rounded-l-4xl h-10 flex w-14 fixed right-0 top-20 transition-all hover:scale-110 animate-pulse hover:animate-none"
    >
      <Icon class="m-auto" :path="mdiPowerPlugOutline"></Icon>
    </div>
    <Layer mask :clickOutsideToClose="true" v-model="showPlugView" transition="opacity">
      <article
        class="fixed top-20 right-0 transition-all"
        :class="showPlugView ? 'translate-x-0' : 'translate-x-120'"
      >
        <div
          class="card bg-base-100 text-primary-content w-115 max-h-[calc(100vh-6rem)]"
        >
          <div class="card-body">
            <Alert :icon="mdiPowerPlugOutline" class="card-title bg-primary">
              {{ t('plug.title') }}
              <template #action>
                <Help v-bind="PlugHelp" trigger-type="icon"></Help>
              </template>
            </Alert>
            <SelfOwned :defaultValue="t('plug.define')">
              <template #="value">
                <TabView
                  :tabs="[t('plug.define'), t('plug.getFromExisting')]"
                  v-bind="value"
                >
                  <template #[t('plug.define')]>
                    <Alert type="info" :style="'dash'">
                      {{ t('plug.typeQuestion') }}
                      <template #action>
                        <Help v-bind="TypeHelp"> </Help>
                      </template>
                    </Alert>
                    <JsonTree
                      max-height="calc(100vh - 15rem)"
                      v-model="plugType"
                    >
                    </JsonTree>
                    <button class="btn btn-primary" @click="updatePlug">
                      {{ t('plug.update') }}
                    </button>
                  </template>
                  <template #[t('plug.getFromExisting')]> </template>
                </TabView>
              </template>
            </SelfOwned>
          </div>
        </div>
      </article>
    </Layer>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watchEffect } from "vue";
import Icon from "./Icon.vue";
import { mdiPowerPlugOutline } from "@mdi/js";
import TabView from "../views/TabView.vue";
import SelfOwned from "../utils/components/SelfOwned.vue";
import Alert from "./Alert.vue";
import Layer from "../utils/components/Layer.vue";
import Help from "./Help.vue";
import JsonTree from "./JsonTree.vue";
import { PlugHelp, TypeHelp } from "./Plug.help";
import { toast } from "../utils/components/ToastProvider.vue";
import { api } from "../invoke";
import { defineAiEntry } from "../ai/component";
import { useI18n } from "vue-i18n";
import { LitCard } from "../invoke/type";

const { t } = useI18n();

const { cardId, plug } = defineProps<{
  cardId: string;
  plug?: LitCard["plug"];
}>();

const plugType = ref({});

watchEffect(() => {
  plugType.value = plug ?? {};
});

const showPlugView = ref(false);


function updatePlug() {
  console.log(cardId,plugType.value);
  api.updateActionPlug(cardId, plugType.value);
  toast.success(t('plug.updateSuccess'));
}

defineAiEntry({
  // doc: {
  //   type: "resource",
  //   description: "插头文档",
  //   // value: ,
  // },
  editPlug: {
    type: "method",
    description: [
      "@name 修改插头",
      "@description 接收一个参数，表示插头的类型",
      "@param {Card['plug']} plug 插头的类型",
      "@effect 更新插头",
    ],
    value: (plug: LitCard["plug"]) => {
      plugType.value = plug;
    },
  },
});
</script>

<i18n lang="yaml">
zh-CN:
  plug:
    title: 为卡片定义插头
    define: 定义插头
    getFromExisting: 从已有卡片中获取插头
    typeQuestion: 我能使用什么类型？
    update: 更新插头
    updateSuccess: 更新插头成功

en:
  plug:
    title: Define plugs for cards
    define: Define plug
    getFromExisting: Get plug from existing cards
    typeQuestion: What types can I use?
    update: Update plug
    updateSuccess: Plug updated successfully
</i18n>
