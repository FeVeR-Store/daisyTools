<template>
  <div class="compact-stat-container w-full">
    <div class="divide-y divide-base-300">
      <div
        v-for="(stat, index) in processedStats"
        :key="index"
        v-event-delegation:[stat].click="openDetailModal"
        class="p-3 hover:bg-base-50 transition-colors"
      >
        <div class="flex items-start gap-3">
          <div class="flex-shrink-0 mt-1">
            <Icon
              :path="
                stat.type === 'Code'
                  ? mdiCodeTags
                  : stat._plug
                  ? mdiPowerPlugOutline
                  : mdiTextBox
              "
              size="16"
              class="text-primary"
            />
          </div>

          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-1">
              <h3 class="font-medium text-sm">{{ stat.title }}</h3>
              <!--        <div :class="['badge', 'badge-xs']">
                {{ stat.priority }}
              </div> -->
            </div>

            <div class="text-xs text-base-content/60 mb-2">
              {{ stat.description }}
            </div>

            <Switch
              :cases="[...argType, 'plug']"
              :value="stat._plug ? 'plug' : stat.type!"
            >
              <template #plug>
                <div
                  @click="jumpToPlugProvider(stat.value)"
                  class="inline-flex items-center gap-1 text-primary cursor-pointer hover:underline"
                >
                  <Icon :path="mdiPowerPlugOutline" size="12" />
                  <span class="text-lg font-mono">{{ stat.value[0] }}</span>
                </div>
              </template>
              <template #Code>
                <div
                  class="text-lg font-mono text-base-content/70 bg-base-200 rounded px-2 py-1 max-h-20 overflow-y-auto"
                >
                  {{ String(stat.value).slice(0, 100)
                  }}{{ String(stat.value).length > 100 ? "..." : "" }}
                </div>
              </template>
              <template #Text>
                <div class="text-lg font-mono text-base-content/70">
                  {{ stat.value }}
                </div>
              </template>
              <template #Int>
                <div class="text-lg font-mono text-base-content/70">
                  {{ stat.value }}
                </div>
              </template>
              <template #String>
                <div class="text-lg font-mono text-base-content/70">
                  {{ stat.value }}
                </div>
              </template>
              <template #Bool>
                <div class="text-lg font-mono text-base-content/70">
                  {{ stat.value }}
                </div>
              </template>
              <template #default>
                <div class="stat-value">{{ stat.value }}</div>
              </template>
            </Switch>
          </div>

          <div class="flex-shrink-0 text-right">
            <div
              :class="[
                'badge',
                'badge-xs',
                stat.type === 'Code'
                  ? 'badge-info'
                  : stat._plug
                  ? 'badge-primary'
                  : 'badge-outline',
              ]"
            >
              {{ stat._plug ? "plug" : stat.type }}
            </div>
            <div class="text-xs text-base-content/50 mt-1">
              {{ stat.width === 2 ? "Wide" : "Normal" }}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 详情模态框 -->
    <Modal
      v-model="showModal"
      v-if="selectedStat"
      title="数据"
      center
      click-outside-to-close
      w="130"
      w-h="1/1"
      fit-size
      #="{ close }"
    >
      <div class="relative bg-base-100 rounded-2xl overflow-y-auto">
        <div class="p-6">
          <div class="flex items-center justify-between mb-4">
            <h2 class="text-xl font-bold flex items-center gap-2">
              <Icon :path="mdiAt" size="20" />
              {{ selectedStat.title }}
            </h2>
            <button @click="close" class="btn btn-sm btn-circle btn-ghost">
              <Icon :path="mdiClose" size="16" />
            </button>
          </div>

          <div class="space-y-4">
            <div class="flex items-center gap-4">
              <div
                :class="[
                  'badge',
                  selectedStat.type === 'Code'
                    ? 'badge-info'
                    : selectedStat._plug
                    ? 'badge-primary'
                    : 'badge-outline',
                ]"
              >
                {{ selectedStat._plug ? "plug" : selectedStat.type }}
              </div>
              <!--               <div
                :class="[
                  'badge',
                  selectedStat.priority === 'high'
                    ? 'badge-error'
                    : selectedStat.priority === 'medium'
                    ? 'badge-warning'
                    : 'badge-success',
                ]"
              >
                {{ selectedStat.priority }}
              </div> -->
              <div class="badge badge-outline">
                {{ selectedStat.width === 2 ? "宽屏" : "标准" }}
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2">描述</h3>
              <p class="text-sm text-base-content/70">
                {{ selectedStat.description }}
              </p>
            </div>

            <div>
              <h3 class="font-semibold mb-2">值</h3>
              <Switch
                :cases="[...argType, 'plug']"
                :value="selectedStat._plug ? 'plug' : selectedStat.type!"
              >
                <template #plug>
                  <div
                    @click="jumpToPlugProvider(selectedStat.value)"
                    class="bg-primary/10 border border-primary/20 rounded-lg p-4 cursor-pointer hover:bg-primary/20 transition-colors"
                  >
                    <div class="flex items-center gap-2">
                      <Icon
                        :path="mdiPowerPlugOutline"
                        size="24"
                        class="text-primary"
                      />
                      <Join
                        :list="selectedStat.value"
                        class="text-primary font-mono"
                      >
                        <template #separator>
                          <Icon :path="mdiChevronRight" size="20" />
                        </template>
                      </Join>
                    </div>
                  </div>
                </template>
                <template #Code>
                  <div class="bg-base-200 rounded-lg overflow-hidden">
                    <CodeEditor :model-value="selectedStat.value" readonly />
                  </div>
                </template>
                <template #Text>
                  <div class="bg-base-200 p-4 rounded-lg text-center">
                    <span class="font-mono text-lg">{{
                      selectedStat.value
                    }}</span>
                  </div>
                </template>
                <template #default>
                  <div class="stat-value">{{ selectedStat.value }}</div>
                </template>
              </Switch>
            </div>

            <div class="flex gap-2">
              <button class="btn btn-primary btn-sm flex-1">
                <Icon :path="mdiContentCopy" size="14" />
                复制
              </button>
              <button class="btn btn-outline btn-sm flex-1">
                <Icon :path="mdiInformation" size="14" />
                详情
              </button>
            </div>
          </div>
        </div>
      </div>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import { computed, inject, ref } from "vue";
import {
  mdiAt,
  mdiChevronRight,
  mdiCodeTags,
  mdiTextBox,
  mdiPowerPlugOutline,
  mdiContentCopy,
  mdiInformation,
  mdiClose,
} from "@mdi/js";
import Icon from "./Icon.vue";
import Switch from "../utils/components/Switch.vue";
import CodeEditor from "./dataInput/CodeEditor.vue";
import Join from "../utils/components/Join.vue";
import { parseData } from "../utils/type";
import { isPlug } from "./dataInput/PlugDisplay.utils";
import Modal from "./Modal.vue";
import { vEventDelegation } from "../directives/eventDelegation";
import { argType } from "../invoke/type";
import { StatProps } from "./Stat.vue";

const props = withDefaults(
  defineProps<{
    stat: StatProps[];
    displayMode?: "mobile" | "compact" | "grid";
  }>(),
  {
    displayMode: "compact",
  }
);
// 注入的跳转函数
const _jumpToPlugProvider = inject<(name: string) => void>(
  "jumpToPlugProvider",
  () => {}
);

const jumpToPlugProvider = ([name]: string[]) => {
  _jumpToPlugProvider(name.slice(1, -1));
};

// 状态管理
const selectedStat = ref<StatProps | null>(null);
const showModal = ref(false);

// 处理统计数据
const processedStats = computed(() => {
  return props.stat.map((stat) => {
    let { width = 1 } = stat;

    const value = parseData(stat.value);
    const currentStat: StatProps = {
      ...stat,
      width,
      value,
      _plug: isPlug(value),
    };
    return currentStat;
  });
});

const openDetailModal = (stat: StatProps) => {
  selectedStat.value = stat;
  showModal.value = true;
};
</script>

<style scoped>
/* 文本截断 */
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* 触摸友好的尺寸 */
.btn,
.card,
[role="button"] {
  min-height: 44px;
}

/* 模态框动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

/* 移动端优化 */
@media (max-width: 768px) {
  .grid-cols-2 {
    grid-template-columns: 1fr;
  }

  .col-span-2 {
    grid-column: span 1;
  }
}
</style>
