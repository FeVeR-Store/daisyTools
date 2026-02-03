<template>
  <Draggable
    :data="{path,type: propertyValue}"
    :disable="!draggable"
    :id="JsonNodeSymbol"
    v-if="!propertyKey.startsWith('\0')"
  >
    <template #dragging>
      <div class="badge badge-primary transition-transform">
        <Icon size="18" :path="mdiPowerPlugOutline"></Icon>
        <Join :list="plugPath">
          <template #separator>
            <Icon size="18" :path="mdiChevronRight"></Icon>
          </template>
        </Join>
      </div>
    </template>
    <li
      class="tooltip:w-full child hover-stop-json-node"
      v-tooltip:[isShadowed]="
        isDeleting
          ? t('tooltip.willDelete')
          : isShadowed
          ? t('tooltip.isShadowed')
          : ''
      "
    >
      <div
        class="flex items-center gap-2 w-full"
        :class="{
          'pr-0': !readonly,
          'line-through bg-error-content': isDeleting || isShadowed,
        }"
      >
        <div class="w-full">
          <div
            class="menu-dropdown-toggle flex items-center gap-2 w-full"
            :class="{ 'menu-dropdown-show': isExpanded }"
            @click="toggleExpand"
          >
            <span
              ref="keyRef"
              :contenteditable="!readonly && !readonlyKey"
              class="key"
              @keydown.enter.prevent="
                updateKeyHandler(($event.target as HTMLDivElement).innerText)
              "
              @input="
                updateEditingKey(($event.target as HTMLDivElement).innerText)
              "
              @click.stop
              @blur="
                updateKeyHandler(($event.target as HTMLDivElement).innerText)
              "
            >
              {{ !allowDelete ? t("member") : propertyKey }}
            </span>
            <div class="text-gray-500">:</div>

            <!-- 如果是基本类型，显示可编辑的值 -->
            <slot
              name="value"
              :property-key="propertyKey"
              :value="propertyValue"
              :display="displayValue"
              :update="updateValue"
            >
              <!-- 输入模式 -->
              <!-- <span
                :contenteditable="!readonly"
                @focus="isEditing = true"
                @blur="(e) => {isEditing = false; updateValue((e.target as HTMLDivElement).innerText)}"
                class="value cursor-pointer hover:bg-base-200 px-2 py-1 rounded w-full"
                :class="{
                  'text-[#ce9178]': typeof props.propertyValue === 'string',
                  'text-[#b5cea8]': typeof props.propertyValue === 'number',
                  'text-[#569cd6]':
                    typeof props.propertyValue === 'boolean' ||
                    props.propertyValue === null ||
                    props.propertyValue === undefined,
                }"
              >
                {{
                  isEditing
                    ? propertyValue === newSymbol
                      ? ""
                      : propertyValue
                    : displayValue
                }}
              </span> -->
            </slot>
            <!-- 如果是对象或数组，显示类型标记 -->
            <Dropdown
              @update:model-value="updateValue"
              :option="createTypeMap()"
              :selected-key="propertyValue"
              :i18n="node.typeMapI18n"
              :disabled="readonly"
              :trigger-class="
                isBasicType
                  ? 'hover:bg-base-200 px-2 py-1 rounded w-full'
                  : [
                      {
                        'badge-info': !isInSocketMode && isObject,
                        'badge-warning': isArray,
                        'badge-success': isTuple,
                        'badge-error': isInSocketMode,
                      },
                      'badge badge-soft rounded-lg text-xs',
                    ]
              "
            >
              <template v-if="!isBasicType" #trigger>
                {{ typeLabel }}
              </template>
            </Dropdown>

            <!-- 操作按钮 -->
            <ActionButtonGroup v-if="!readonly" class="actions" :btns />
          </div>

          <!-- 子节点容器 -->
          <ul
            v-if="isObject || isArray || isTuple"
            class="menu-dropdown pl-4"
            :class="{ 'menu-dropdown-show': isExpanded }"
          >
            <JsonNode
              v-for="(childValue, childKey) in subNodes"
              :key="childKey"
              :property-key="childKey"
              :property-value="childValue"
              :path="[...path, String(childKey)]"
              :readonly
              :readonly-key="isTuple || isArray"
              :format
              :isDeleting
              :isShadowed="currentAddingKey === childKey"
              :allow-delete="!isArray"
              :draggable
              @update:value="updateChildValue(String(childKey), $event)"
              @update:key="updateChildKey(String(childKey), $event)"
              @editing:key="currentAddingKey = $event"
              @delete="deleteChildProperty(String(childKey))"
              @cancel="cancelNewChildKey"
              @comfirm="comfirmNewChildKey"
            >
              <!-- @vue-ignore -->
              <template #add="scope">
                <slot name="add" v-bind="scope"></slot>
              </template>
              <!-- @vue-ignore -->
              <template #value="scope">
                <slot name="value" v-bind="scope"></slot>
              </template>
            </JsonNode>
          </ul>
        </div>
      </div>
    </li>
  </Draggable>
</template>

<script setup lang="ts">
import {
  computed,
  ref,
  useTemplateRef,
  onMounted,
  inject,
  watchEffect,
  nextTick,
} from "vue";
import {
  mdiArrowULeftTop,
  mdiChevronRight,
  mdiDelete,
  mdiPlus,
  mdiPowerPlugOutline,
} from "@mdi/js";
import { vTooltip } from "../directives/tooltip";
import * as node from "./JsonNode.utils";
import { createTypeMap, defaultType } from "./JsonNode.utils";
import Dropdown from "./Dropdown.vue";
import ActionButtonGroup, { ActionButton } from "./ActionButtonGroup.vue";
import { useI18n } from "vue-i18n";
import Draggable from "../utils/components/Draggable.vue";
import { usePlugPath } from "./dataInput/PlugDisplay.utils";
import Icon from "./Icon.vue";
import Join from "../utils/components/Join.vue";

const { t, locale } = useI18n();
const JsonNodeSymbol = Symbol.for("jsonNode");

// 新添加的成员的占位符
const newSymbol = Symbol.for("new");

const props = withDefaults(
  defineProps<{
    propertyKey: string;
    propertyValue: any;
    // 当前成员在树中的路径
    path: string[];
    // 格式化函数
    format?: (value: any, isNewValue: boolean) => string;
    // 父节点是否正在被删除
    isDeleting?: boolean;
    // 当前成员是否被遮蔽
    isShadowed?: boolean;
    // 是否允许删除, 只有数组的[成员]不允许删除
    allowDelete?: boolean;
    // 是否只读
    readonly?: boolean;
    // 键是否只读
    readonlyKey?: boolean | "all";
    // 是否可拖拽
    draggable?: boolean;
  }>(),
  { allowDelete: true }
);

const isShadowed = computed(() => {
  if (isEditingKey.value) return false;
  return props.isShadowed;
});

const emit = defineEmits<{
  (e: "update:value", value: any): void;
  (e: "update:key", key: string): void;
  (e: "delete"): void;
  (e: "cancel"): void;
  (e: "comfirm", key: string): void;
  (e: "editing:key", key: string): void;
}>();

const keyRef = useTemplateRef("keyRef");

const updateWillDelete = inject(
  "update:willDelete",
  (_willDelete: () => void) => {}
);
const doDelete = inject("do:delete", () => {});
const _cancelDelete = inject("cancel:delete", () => {});

onMounted(() => {
  if (props.propertyValue === newSymbol && props.propertyKey === "") {
    keyRef.value?.focus();
  }
});

// 当前节点是否在删除
const isDeleting = ref(false);
const isAdding = ref(false);
const isExpanded = ref(true);
const isEditingKey = ref(false);

const btns = computed(() => {
  return [
    {
      // 立刻删除，在允许删除，并且正在删除的情况下会显示
      show: props.allowDelete && isDeleting.value,
      icon: mdiDelete,
      callback: doDelete,
      tooltip: t("tooltip.doDelete"),
      type: "error",
    },
    {
      // 添加成员，不在删除状态，并且在对象或元组中会显示
      show:
        !isDeleting.value &&
        !props.isDeleting &&
        (isObject.value || isTuple.value),
      icon: mdiPlus,
      callback: addChild,
      tooltip: t("tooltip.addChild"),
      type: "accent",
    },
    {
      // 撤销删除，在允许删除，并且正在删除的情况下会显示
      show: props.allowDelete && isDeleting.value,
      icon: mdiArrowULeftTop,
      callback: cancelDelete,
      tooltip: t("tooltip.cancelDelete"),
      type: "warning",
    },
    {
      // 删除，在允许删除，当前节点不是删除状态下会显示
      show: props.allowDelete && !isDeleting.value,
      icon: mdiDelete,
      callback: deleteNode,
      tooltip: t("tooltip.delete"),
      type: "error",
    },
  ] satisfies ActionButton[];
});

const currentAddingKey = ref<string | null>(null);

watchEffect(() => {
  isAdding.value;
  isExpanded.value = true;
  currentAddingKey.value = null;
});

const subNodes = computed<any[] | Record<string, any>>(() => {
  if (isAdding.value) {
    return { ...props.propertyValue, "": newSymbol };
  } else {
    return props.propertyValue;
  }
});

// 类型判断
const isObject = computed(() => !!node.isObject(props.propertyValue));

const isArray = computed(() => !!node.isArray(props.propertyValue));

const isTuple = computed(() => !!node.isTuple(props.propertyValue));

const isInSocketMode = computed(() => !!props.propertyValue["\0socket"]);

const isBasicType = computed(
  () => !isObject.value && !isArray.value && !isTuple.value
);

// 显示标签
const typeLabel = computed(() => {
  const memberSum = Object.keys(props.propertyValue).length - 1;
  if (isInSocketMode.value)
    return t("label.plug", [
      node.typeMapI18n[locale.value as 'zh-CN'][props.propertyValue["\0type"] as "string"],
    ]);
  if (isObject.value) return t("label.object", [memberSum]);
  if (isArray.value) return t("label.array");
  if (isTuple.value) return t("label.tuple", [memberSum]);
  return typeof props.propertyValue;
});

// 显示值
const displayValue = computed(() => {
  if (props.format) {
    return props.format(props.propertyValue, props.propertyValue === newSymbol);
  }
  if (props.propertyValue === newSymbol) return "unknown";
  if (props.propertyValue === null) return "null";
  if (props.propertyValue === undefined) return "undefined";
  if (typeof props.propertyValue === "string")
    return `"${props.propertyValue}"`;
  return String(props.propertyValue);
});

// 切换展开状态
function toggleExpand() {
  isExpanded.value = !isExpanded.value;
}

function updateKeyHandler(key: string) {
  isEditingKey.value = false;
  if (props.propertyKey === "" && props.propertyValue === newSymbol) {
    if (key.trim() === "") return emit("cancel");
    return emit("comfirm", key);
  }
  emit("update:key", key);
}

// 更新值
function updateValue(value: string) {
  // 尝试将值转换为适当的类型
  let parsedValue: any = value;

  if (typeof props.propertyValue === "number") {
    parsedValue = Number(value);
  } else if (typeof props.propertyValue === "boolean") {
    parsedValue = value.toLowerCase() === "true";
  }

  emit("update:value", parsedValue);
}

// 更新新键，用于判断属性遮蔽
function updateEditingKey(key: string) {
  isEditingKey.value = true;
  emit("editing:key", key);
}

// 添加子属性
function addChild() {
  if (isTuple.value) {
    const newValue = { ...props.propertyValue };
    newValue[`[${Object.keys(newValue).length - 1}]`] = defaultType;
    doDelete();
    emit("update:value", newValue);
    return;
  }
  isAdding.value = true;
}

function cancelNewChildKey() {
  isAdding.value = false;
}

async function comfirmNewChildKey(key: string) {
  const newValue = { ...props.propertyValue };
  newValue[key] = defaultType;
  emit("update:value", newValue);
  isAdding.value = false;
  await nextTick();
  doDelete();
}

// 删除当前节点
function deleteNode() {
  isDeleting.value = true;
  doDelete();
  updateWillDelete(() => {
    emit("delete");
    isDeleting.value = false;
  });
}

function cancelDelete() {
  isDeleting.value = false;
  _cancelDelete();
}

function updateChildKey(key: string, newKey: string) {
  const newData: any = isObject.value ? {} : [];
  Object.entries(props.propertyValue).forEach(([oldKey, value]) => {
    if (oldKey === key) {
      newData[newKey] = value;
    } else {
      newData[oldKey] = value;
    }
  });
  emit("update:value", newData);
  currentAddingKey.value = null;
  doDelete();
}

// 更新子属性的值
function updateChildValue(key: string, value: any) {
  const newValue = { ...props.propertyValue };
  newValue[key] = value;
  emit("update:value", newValue);
}

// 删除子属性
function deleteChildProperty(key: string) {
  const newValue = { ...props.propertyValue };
  delete newValue[key];
  emit("update:value", newValue);
}

// 拖拽
const { plugPath } = usePlugPath(() => props.path, 5);
</script>

<style scoped>
@reference "../main.css";

.menu-dropdown-toggle {
  display: flex;
  align-items: center;
  cursor: pointer;
  gap: 4px;
  border-radius: 0.5rem;
  transition: background-color 0.2s ease;
}

.menu-dropdown-toggle:hover {
  background-color: rgba(0, 0, 0, 0.05);
}

.menu-dropdown {
  display: none;
  margin-top: 0.25rem;
  margin-left: 1rem;
  border-left: 1px dashed rgba(0, 0, 0, 0.2);
  padding-left: 0.5rem;
  transition: all 0.3s ease;
}

.menu-dropdown-show + .menu-dropdown {
  display: block;
}

.menu-dropdown-toggle.menu-dropdown-show {
  background-color: rgba(0, 0, 0, 0.03);
}

.key {
  color: #9cdcfe;
}

[contenteditable]:focus {
  outline: none;
  border-bottom: 1px solid #fff;
}
[contenteditable] {
  @apply px-2 py-1 hover:bg-base-200;
}

.type-indicator {
  font-style: italic;
}

.actions {
  margin-left: auto;
  display: flex;
  gap: 4px;
}

.v-enter-active,
.v-leave-active {
  transition: opacity 0.2s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>

<style>
/* 实现只有当前hover的项突出显示 */
@reference "../main.css";
.hover-stop-json-node:not(:has(.hover-stop-json-node:hover)):hover {
  @apply transition-all;
  background: color-mix(
    in oklab,
    var(--color-base-content) /* var(--color-base-content) */ 10%,
    transparent
  ) !important;
}
.hover-stop-json-node:hover {
  background-color: var(--color-base-300) !important;
}
</style>

<i18n lang="yaml">
zh-CN:
  tooltip:
    willDelete: 此条目将在下次操作或保存时删除
    isShadowed: 此条目将被新条目遮蔽而删除
    doDelete: 立刻删除
    addChild: 添加成员
    cancelDelete: 撤销删除
    delete: 删除
  label:
    object: 对象{'{'}{0}{'}'}
    array: 数组
    tuple: 元组({0})
    plug: 插头 [{0}]
  member: "[成员]"

en:
  tooltip:
    willDelete: This item will be deleted on next operation or save
    isShadowed: This item will be deleted due to being shadowed by new item
    doDelete: Delete immediately
    addChild: Add member
    cancelDelete: Cancel delete
    delete: Delete
  label:
    object: Object{'{'}{0}{'}'}
    array: Array
    tuple: Tuple({0})
    plug: Plug [{0}]
  member: "[Member]"
</i18n>
