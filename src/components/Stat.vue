<!-- include: grid-cols-1 grid-cols-2 -->

<template>
  <div>
    <div
      ref="gridContainer"
      class="stats shadow w-full grid layout transition-all"
    >
      <div
        v-for="(stat, i) in blocks"
        :key="i"
        :ref="(el) => setStatRef(el, i)"
        :style="{ gridArea: `stat-${i}` }"
        class="stat w-full stat-item"
        :class="[stat.type && `${stat.type}-style`, 'pl-0']"
      >
        <div class="stat-title text-lg [line-height:2]">
          <Icon inline size="18" :path="mdiAt"></Icon> {{ stat.title }}
        </div>
        <Switch
          :cases="[...argType, 'plug']"
          :value="stat._plug ? 'plug' : stat.type!"
        >
          <template #Code>
            <CodeEditor
              :model-value="`console.log('hello world')\n\n`"
              readonly
            >
            </CodeEditor>
          </template>
          <template #Text>
            <div class="stat-value">{{ stat.value }}</div>
          </template>
          <template #plug>
            <div
              @click="jumpToPlugProvider(stat.value)"
              v-show="activeStat === i || stat.width === maxWidth"
              class="stat-figure text-primary border-3 border-dashed rounded-4xl p-2"
            >
              <Icon size="40" :path="mdiPowerPlugOutline"></Icon>
            </div>
            <div
              v-tooltip="t('tooltip.isPlug', stat.value)"
              @click="handleStatClick(i)"
              class="stat-value whitespace-nowrap inline-flex text-primary"
              :class="{ 'hover-show-icon': activeStat !== i }"
            >
              <div
                class="whitespace-nowrap overflow-hidden text-ellipsis plug flex flex-wrap"
              >
                <template v-if="activeStat !== i">
                  {{ stat.value[0] }}
                </template>
                <template v-else>
                  <Join :list="[...stat.value, ...stat.value]">
                    <template #separator>
                      <Icon size="48" :path="mdiChevronRight"></Icon>
                    </template>
                  </Join>
                </template>
              </div>

              <div v-show="activeStat !== i" class="icon inline-flex">
                <Icon size="48" :path="mdiChevronRight"></Icon>
                ...
              </div>
            </div>
          </template>
          <template #default>
            <div class="stat-value">{{ stat.value }}</div>
          </template>
        </Switch>
        <div class="stat-desc">{{ stat.description }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, inject, nextTick, ref } from "vue";
import CodeEditor from "./dataInput/CodeEditor.vue";
import Switch from "../utils/components/Switch.vue";
import Icon from "./Icon.vue";
import { mdiAt, mdiChevronRight, mdiPowerPlugOutline } from "@mdi/js";
import { parseData } from "../utils/type";
import { isPlug } from "./dataInput/PlugDisplay.utils";
import Join from "../utils/components/Join.vue";
import { vTooltip } from "../directives/tooltip";
import { useI18n } from "vue-i18n";
import { ArgType, argType } from "../invoke/type";

const { t } = useI18n();

const _jumpToPlugProvider = inject<(name: string) => void>(
  "jumpToPlugProvider",
  () => {}
);

const jumpToPlugProvider = ([name]: string[]) => {
  _jumpToPlugProvider(name.slice(1, -1));
};

export interface StatProps {
  type?: ArgType;
  title: string;
  value: string | any;
  description: string;
  width?: 1 | 2;
  /** @internal */
  _plug?: boolean;
  /** @internal */
  _pos?: [number, number];
}

const maxWidth = 2;
const widthMap: { [key: string]: 1 | 2 } = {
  Code: 2,
  Text: 2,
};

const activeStat = ref<number>();
const gridContainer = ref<HTMLElement>();
const statRefs = ref<(HTMLElement | null)[]>([]);
const isAnimating = ref(false);

// 存储元素位置信息
const statPositions = ref<Map<number, DOMRect>>(new Map());

const setStatRef = (el: any, index: number) => {
  if (el && el instanceof HTMLElement) {
    statRefs.value[index] = el;
  }
};

const { stat } = defineProps<{
  stat: StatProps[];
}>();

const blocks = computed(() => {
  return stat.map((stat) => {
    let { width = 1, type } = stat;
    if (!width) {
      width = (type && widthMap[type]) || 1;
    }

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

const layout = computed(() => {
  const gridTemplateAreas: string[] = [];
  let currentLine: string[] = [];
  function addLineToGrid() {
    gridTemplateAreas.push(
      `"${[
        ...currentLine,
        ...new Array(maxWidth - currentLine.length).fill("."),
      ].join(" ")}"`
    );
    currentLine = [];
  }
  blocks.value.forEach((block, i) => {
    const width = i === activeStat.value ? maxWidth : block.width!;
    if (currentLine.length === maxWidth) {
      addLineToGrid();
    }
    if (currentLine.length + width > maxWidth) {
      addLineToGrid();
    }
    currentLine.push(...new Array(width).fill(`stat-${i}`));
  });
  if (currentLine.length > 0) {
    addLineToGrid();
  }
  return gridTemplateAreas.join(" ");
});

// FLIP 动画函数
const recordPositions = () => {
  statPositions.value.clear();
  statRefs.value.forEach((el, index) => {
    if (el) {
      statPositions.value.set(index, el.getBoundingClientRect());
    }
  });
};

const animateToNewPositions = async () => {
  if (!gridContainer.value || isAnimating.value) return;

  isAnimating.value = true;

  // 等待 DOM 更新完成
  await nextTick();

  const newPositions = new Map<number, DOMRect>();
  statRefs.value.forEach((el, index) => {
    if (el) {
      newPositions.set(index, el.getBoundingClientRect());
    }
  });

  // 计算并应用初始 transform
  statRefs.value.forEach((el, index) => {
    if (!el) return;

    const oldPos = statPositions.value.get(index);
    const newPos = newPositions.get(index);

    if (oldPos && newPos) {
      const deltaX = oldPos.left - newPos.left;
      const deltaY = oldPos.top - newPos.top;
      const scaleX = oldPos.width / newPos.width;
      const scaleY = oldPos.height / newPos.height;

      // 计算缩放程度，用于模糊和透明度效果
      const scaleDiff = Math.abs(1 - Math.max(scaleX, scaleY));
      const blurAmount = Math.min(scaleDiff * 3, 2); // 最大模糊2px
      const opacity = Math.max(0.3, 1 - scaleDiff * 0.75); // 最低透明度0.7

      // 设置初始状态（变化前的位置和大小）
      el.style.transform = `translate(${deltaX}px, ${deltaY}px) scale(${scaleX}, ${scaleY})`;
      el.style.filter = `blur(${blurAmount}px)`;
      el.style.opacity = opacity.toString();
      el.style.transition = "none";
    }
  });

  // 强制重排
  gridContainer.value.offsetHeight;

  // 应用过渡并移除 transform
  statRefs.value.forEach((el) => {
    if (el) {
      el.style.transition =
        "transform 0.3s cubic-bezier(0.4, 0, 0.2, 1), filter 0.3s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1)";
      el.style.transform = "";
      el.style.filter = "";
      el.style.opacity = "";
    }
  });

  // 清理过渡样式
  setTimeout(() => {
    statRefs.value.forEach((el) => {
      if (el) {
        el.style.transition = "";
        el.style.transform = "";
        el.style.filter = "";
        el.style.opacity = "";
      }
    });
    isAnimating.value = false;
  }, 300);
};

const handleStatClick = async (index: number) => {
  // 记录当前位置
  recordPositions();

  // 更新状态
  activeStat.value = activeStat.value === index ? undefined : index;

  // 执行动画
  await animateToNewPositions();
};
</script>

<style scoped>
.layout {
  grid-template-areas: v-bind(layout);
  grid-template-columns: repeat(2, 1fr);
}

.stat-item {
  /* 确保变换原点在左上角 */
  transform-origin: top left;
  border: none;
}

.code-style {
  & > .stat-desc {
    margin-top: 0.5rem;
  }
}

.hover-show-icon {
  & > .icon {
    opacity: 0;
    pointer-events: none;
    transform: translateX(20px);
    transition: all 0.3s;
  }
  & .plug {
    width: 80%;
  }
  &:hover > .icon {
    animation: show-icon 0.2s forwards;
  }
  &:hover .plug {
    width: fit-content;
    max-width: 50%;
  }
}

@keyframes show-icon {
  0% {
    opacity: 0;
    transform: translateX(-10px);
  }
  100% {
    opacity: 1;
    transform: translateX(0);
  }
}
</style>

<i18n lang="yaml">
zh-CN:
  tooltip:
    isPlug: 这是{0}的插头
en:
  tooltip:
    isPlug: This is the plug of {0}
</i18n>
