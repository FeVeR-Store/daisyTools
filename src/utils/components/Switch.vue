<template>
  <component
    v-for="node in currentNodes"
    :class="[{ zoom: value != currentCase }, 'enlarge']"
    :is="node"
    :key="currentCase"
  ></component>
</template>

<script setup lang="ts" generic="const V extends string, const T extends V[]">
import { cloneVNode, computed, h, ref, useAttrs, VNode, watch } from "vue";
import { mdiAlertCircleOutline } from "@mdi/js";
import { useI18n } from "vue-i18n";
import Alert from "../../components/Alert.vue";
import { inDevMode } from "../mode";
defineOptions({
  inheritAttrs: false,
});
const { value, duration, cases } = defineProps<{
  value: V;
  cases: T;
  duration?: number;
}>();

const { t } = useI18n();

const currentCase = ref<V>(value);

// 将Switch组件接收的attributes透传
const slots =
  defineSlots<{ [name in "default" | (T[number] & string)]: any }>();
const attrs = useAttrs();

const currentNodes = computed(() => {
  let matchedCase = cases.includes(value) ? value : "default";
  if (!(matchedCase in slots)) {
    if (!("default" in slots)) {
      return unmatchedWarning();
    } else {
      matchedCase = "default";
    }
  }
  const vnodes: VNode[] = (slots as any)[matchedCase]?.();
  if (inDevMode() && vnodes.length === 1) {
    if (vnodes[0].type === "template") {
      return individualDefaultWarning(vnodes);
    }
  }
  vnodes[0].props = { ...attrs, ...vnodes[0].props };
  return vnodes.map((vnode) => h(vnode));
});

watch(
  () => value,
  (val) => {
    if (duration) {
      setTimeout(() => {
        currentCase.value = val;
      }, duration);
    } else {
      currentCase.value = val;
    }
  }
);

function unmatchedWarning() {
  return [
    h(Alert, { type: "error", icon: mdiAlertCircleOutline }, [
      h(
        "div",
        t("unmatch.warning", [cases.join(","), String(currentCase.value)])
      ),
      h("div", t("unmatch.tip")),
    ]),
  ];
}

function individualDefaultWarning(template: VNode[]) {
  return [
    h(
      Alert,
      {
        type: "warning",
        icon: mdiAlertCircleOutline,
      },
      [
        t("unmatch.invisible.warning"),
        h(
          "div",
          { class: ["p-4", "my-1", "border-1", "border-dashed", "rounded-xl"] },
          h(template)
        ),
        t("unmatch.invisible.maybe"),
        h(
          "div",
          { class: ["p-4", "my-1", "border-1", "border-dashed", "rounded-xl"] },
          h({ ...cloneVNode(template[0]), type: "div" })
        ),
        t("unmatch.invisible.tip", [
          "<template>...</template>",
          "<template #default>...</template>、<template #>...</template>",
        ]),
      ]
    ),
  ];
}

const delay = computed(() => (duration ?? 300) - 750 + "ms");
</script>

<style>
.zoom {
  opacity: 0;
  transition: all 0.5s v-bind(delay);
}
.enlarge {
  animation: enlarge ease 0.5s forwards;
}
@keyframes enlarge {
  from {
    opacity: 0;
  }
  to {
    opacity: 100%;
  }
}
</style>

<i18n lang="yaml">
zh-CN:
  unmatch:
    warning: 没有匹配的分支, 可能的分支有：{0}, 接收到的是：{1}
    tip: "如果存在不确定的分支，或许可以添加default插槽"
    invisible:
      warning: "接收到的值不与任何分支匹配，但默认插槽返回的是一个template元素：渲染结果可能不可见:"
      maybe: "或许您想渲染的内容是："
      tip: "可以尝试将类似{0}改为{1}或者不使用template包裹"

en:
  unmatch:
    warning: "No matching branch, possible branches are: {0}, received is: {1}"
    tip: "If there are uncertain branches, perhaps add a default slot"
    invisible:
      warning: "The received value does not match any branch, but the default slot returns a template element: the rendered result may be invisible."
      maybe: "Perhaps what you intended to render is:"
      tip: "You can try changing {0} to {1}, or avoid wrapping with a template element."
</i18n>
