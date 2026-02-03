<template>
  <div
    @mousedown.stop
    @click.stop
    :class="['dropdown dropdown-center', { 'pointer-events-none': disabled }]"
  >
    <div
      @mousedown="!disabled && focus($event)"
      tabindex="0"
      role="button"
      class="hover:bg-base-300 px-2 py-1 rounded w-full"
      :class="[triggerClass]"
    >
      <slot name="trigger"> {{ display }} </slot>
    </div>
    <Teleport to="body">
      <ul
        v-click-outside:[open]="closeDropdown"
        v-show="open"
        ref="list"
        @click="closeDropdown"
        tabindex="0"
        class="dropdown-content menu border-1 border-gray-500 w-max fixed!"
        :class="[
          listClass,
          layout && 'block',
          'p-2 shadow-sm bg-base-100 rounded-box',
        ]"
      >
        <li
          v-event-delegation:[{item,key}].click="onClick"
          :class="nodeClass"
          v-for="(item, key) in option"
          :key="key"
        >
          <a>{{ t(key) }}</a>
        </li>
      </ul>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {
  computed,
  nextTick,
  onUnmounted,
  reactive,
  ref,
  useTemplateRef,
  watchEffect,
} from "vue";
import { vEventDelegation } from "../directives/eventDelegation";
import { vClickOutside } from "../directives/clickOutside";
import { I18n, useI18n } from "vue-i18n";
import { createI18nWithUtils } from "../i18n/utils";

const { locale } = useI18n();

const props = defineProps<{
  triggerClass?: any;
  listClass?: any;
  nodeClass?: any;
  option: { [key: string]: any };
  i18n?: { [lang: string]: any };
  disabled?: boolean;
}>();

const currentI18n = ref<I18n<any>>();

watchEffect(() => {
  if (props.i18n) {
    currentI18n.value?.dispose();
    const i18n = createI18nWithUtils({
      messages: props.i18n,
      locale: locale.value,
    });
    currentI18n.value = i18n;
  }
});

onUnmounted(() => {
  currentI18n.value?.dispose();
});

const t = computed(() => currentI18n.value?.global.t ?? ((key: any) => key));

const modelValue = defineModel();
const modelValueKey = defineModel("selectedKey");

const display = computed(() => {
  try {
    return t.value(modelValueKey.value ?? Object.keys(props.option)[0]);
  } catch (e) {
    modelValueKey.value = Object.keys(props.option)[0];
    return t.value(modelValueKey.value);
  }
});

function onClick({ item, key }: { item: any; key: string }) {
  modelValue.value = item;
  modelValueKey.value = key;
}

const layout = ref(false);
const open = ref(false);

const list = useTemplateRef("list");
const _listOffset = reactive({ x: 0, y: 0 });
const listOffset = computed<{ x: string; y: string }>(() => ({
  x: _listOffset.x + "px",
  y: _listOffset.y + "px",
}));

const listSize = ref<{ width: number; height: number } | null>(null);
watchEffect(() => {
  props;
  listSize.value = null;
});

async function focus(e: MouseEvent) {
  // 聚焦之后，需要获取触发器和下拉列表的位置，然后计算需要移动的距离
  const { clientX, clientY } = e;

  open.value = !open.value;
  layout.value = true;
  if (!listSize.value) {
    await nextTick();
    const { height, width } = list.value!.getBoundingClientRect();
    listSize.value = { height, width };
  }
  const { height, width } = listSize.value;
  // console.log(bottom, height, innerHeight, triggerHeight);
  layout.value = false;
  _listOffset.y = clientY + 15;
  _listOffset.x = clientX + 15;
  if (clientY + height + 15 > innerHeight) {
    // 保证纵向不遮挡
    _listOffset.y += innerHeight - clientY - height * 1.1;
    // 横向，因为纵向不遮挡时，会挡住触发器
    // 先尝试右侧
    if (clientX + width <= innerWidth) {
      _listOffset.x += width / 4;
    } else {
      _listOffset.x -= (width * 4) / 3;
    }
  }
  return false;
}

function closeDropdown() {
  open.value = false;
}
</script>

<style scoped>
ul {
  top: v-bind("listOffset.y");
  left: v-bind("listOffset.x");
}
</style>
