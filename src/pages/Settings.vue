<template>
  <div class="container mx-auto p-6 max-w-3xl">
    <h1 class="text-2xl font-bold mb-6">{{ t("title") }}</h1>

    <div class="card bg-base-200 shadow-xl mb-6">
      <div class="card-body overflow-y-auto h-[calc(100vh-14rem)]">
        <div class="flex">
          <h2 class="card-title text-xl mb-3">
            {{ t("aiConfig.title") }}
            <Help v-bind="AIConfigHelp" trigger-type="icon"> </Help>
          </h2>
        </div>

        <div class="grid gap-6">
          <!-- API Key -->
          <div class="form-control w-full">
            <label class="label">
              <span class="label-text font-medium">{{
                t("aiConfig.apiKey.title")
              }}</span>
            </label>
            <input
              type="password"
              v-model="config.aiConfig.apiKey"
              :placeholder="t('aiConfig.apiKey.placeholder')"
              class="input input-bordered w-full"
            />
          </div>

          <!-- 模型选择 -->
          <div class="form-control w-full">
            <label class="label">
              <span class="label-text font-medium">{{
                t("aiConfig.model.title")
              }}</span>
            </label>
            <AutoComplete
              v-model="config.aiConfig.model"
              :placeholder="t('aiConfig.model.placeholder')"
              :options="modelList"
              :strict="t('aiConfig.model.strict')"
            >
              <template #suffix>
                <span v-if="providerName" class="badge badge-primary">{{
                  providerName
                }}</span>
                <span
                  v-else
                  @click.stop="addCustomModel"
                  class="badge badge-accent font-bold"
                >
                  <Icon size="16" :path="mdiPlus"></Icon>
                  {{ t("aiConfig.model.custom") }}</span
                >
              </template>
            </AutoComplete>
          </div>

          <!-- 滑动条设置区域 -->
          <div class="divider my-2"></div>

          <!-- Temperature -->
          <div class="form-control w-full">
            <div class="flex justify-between items-center mb-2">
              <span class="font-medium">{{ t("aiConfig.temperature") }}</span>
              <span class="badge badge-primary">{{
                config.aiConfig.temperature
              }}</span>
            </div>
            <input
              type="range"
              min="0"
              max="2"
              step="0.1"
              v-model.number="config.aiConfig.temperature"
              class="range range-primary w-full"
            />
            <div
              class="flex justify-between text-xs px-2 mt-1 text-base-content/70"
            >
              <span>0</span>
              <span>1</span>
              <span>2</span>
            </div>
          </div>

          <!-- Max Tokens -->
          <div class="form-control w-full">
            <div class="flex justify-between items-center mb-2">
              <span class="font-medium">{{
                t("aiConfig.maxTokens.title")
              }}</span>
              <span
                v-if="config.aiConfig.maxTokens === 'Default'"
                class="badge badge-primary"
              >
                {{ t("aiConfig.maxTokens.default") }}
              </span>
            </div>
            <input
              type="range"
              min="100"
              max="8000"
              step="100"
              v-model.number="config.aiConfig.maxTokens"
              class="range range-primary w-full"
            />
            <div
              class="flex justify-between text-xs px-2 mt-1 text-base-content/70"
            >
              <span>100</span>
              <span>4000</span>
              <span>8000</span>
            </div>
          </div>

          <!-- Top P -->
          <div class="form-control w-full">
            <div class="flex justify-between items-center mb-2">
              <span class="font-medium">{{ t("aiConfig.topP") }}</span>
              <span class="badge badge-primary">{{
                config.aiConfig.topP
              }}</span>
            </div>
            <input
              type="range"
              min="0"
              max="1"
              step="0.05"
              v-model.number="config.aiConfig.topP"
              class="range range-primary w-full"
            />
            <div
              class="flex justify-between text-xs px-2 mt-1 text-base-content/70"
            >
              <span>0</span>
              <span>0.5</span>
              <span>1</span>
            </div>
          </div>

          <!-- Frequency Penalty -->
          <div class="form-control w-full">
            <div class="flex justify-between items-center mb-2">
              <span class="font-medium">{{
                t("aiConfig.frequencyPenalty")
              }}</span>
              <span class="badge badge-primary">{{
                config.aiConfig.frequencyPenalty
              }}</span>
            </div>
            <input
              type="range"
              min="-2"
              max="2"
              step="0.1"
              v-model.number="config.aiConfig.frequencyPenalty"
              class="range range-primary w-full"
            />
            <div
              class="flex justify-between text-xs px-2 mt-1 text-base-content/70"
            >
              <span>-2</span>
              <span>0</span>
              <span>2</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="flex justify-end gap-3">
      <button @click="resetConfig" class="btn btn-outline">
        {{ t("reset") }}
      </button>
      <button @click="saveConfig" class="btn btn-primary">
        {{ t("save") }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Help from "../components/Help.vue";
import { mdiPlus } from "@mdi/js";
import Icon from "../components/Icon.vue";
import { getAllModels, provider } from "../ai";
import { AIConfigHelp } from "./Settings.help";
import AutoComplete from "../components/dataInput/AutoComplete.vue";
import { AiConfig } from "../ai/aiInstance";
import { api } from "../invoke";
import { useI18n } from "vue-i18n";

const { t } = useI18n({});

const modelList = getAllModels();
const providerName = computed(() => {
  config.value.aiConfig.model;
  return Object.entries(provider).find(([, models]) => {
    return (models as string[]).includes(config.value.aiConfig.model);
  })?.[0];
});

function addCustomModel() {
  console.log("addCustomModel");
}

interface AppConfig {}

export interface Config {
  aiConfig: AiConfig;
  appConfig: AppConfig;
}

const defaultConfig: Config = {
  aiConfig: {
    apiKey: "",
    model: "gpt-3.5-turbo",
    temperature: 0.7,
    maxTokens: 2000,
    topP: 1,
    frequencyPenalty: 0,
  },
  appConfig: {},
};

const config = ref<Config>({ ...defaultConfig });
const originalConfig = ref<Config>({ ...defaultConfig });

onMounted(async () => {
  try {
    const result = await api.get_config();
    config.value = result;
    originalConfig.value = JSON.parse(JSON.stringify(result));
  } catch (error) {
    console.error("获取配置失败:", error);
  }
});

const saveConfig = async () => {
  try {
    await invoke("save_config", { config: config.value });
    originalConfig.value = JSON.parse(JSON.stringify(config.value));
    alert("配置已保存");
  } catch (error) {
    console.error("保存配置失败:", error);
  }
};

const resetConfig = () => {
  config.value = JSON.parse(JSON.stringify(originalConfig.value));
};
</script>

<i18n lang="json">
{
  "zh-CN": {
    "title": "设置",
    "aiConfig": {
      "title": "AI配置",
      "maxTokens": { "title": "最大Token数", "default": "默认值" },
      "topP": "Top P",
      "frequencyPenalty": "频率惩罚(Frequency Penalty)",
      "temperature": "温度(Temperature)",

      "apiKey": {
        "title": "API Key",
        "placeholder": "输入您的API Key",
        "strict": "需要输入正确的API Key"
      },
      "model": {
        "title": "模型",
        "placeholder": "选择或输入您的模型",
        "custom": "自定义模型",
        "strict": "需要输入正确的模型"
      }
    },
    "appConfig": {
      "title": "应用配置",
      "reset": "重置",
      "save": "保存"
    },
    "reset": "重置",
    "save": "保存"
  },
  "en": {
    "title": "Settings",
    "aiConfig": {
      "title": "AI Config",
      "maxTokens": { "title": "Max Tokens", "default": "Default" },
      "topP": "Top P",
      "apiKey": {
        "title": "API Key",
        "placeholder": "Enter your API Key",
        "strict": "Need to enter the correct API Key"
      },
      "model": {
        "title": "Model",
        "placeholder": "Select or enter your model",
        "custom": "Custom Model",
        "strict": "Need to enter the correct model"
      }
    },
    "appConfig": {
      "title": "App Config",
      "reset": "Reset",
      "save": "Save"
    },
    "reset": "Reset",
    "save": "Save"
  }
}
</i18n>
