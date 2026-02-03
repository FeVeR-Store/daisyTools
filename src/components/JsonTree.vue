<template>
  <div class="json-tree-viewer">
    <ul class="menu menu-sm bg-base-200 rounded-box w-full">
      <div v-if="!readonly" class="navbar min-h-0 h-10 bg-base-100 shadow-sm">
        <div class="w-full">
          <ul class="menu menu-horizontal px-1">
            <!-- 类型选项栏 -->
            <div class="m-auto mr-4">{{ t("title") }}</div>
            <Dropdown
              :disabled="readonly"
              v-model:selected-key="type"
              @update:selected-key="updateType"
              :option="typeMap"
              :i18n="typeMapI18n"
              trigger-class="hover:bg-gray-700"
            ></Dropdown>
          </ul>
        </div>
        <!-- 对象/元组的成员添加按钮 -->
        <div
          v-if="!readonly"
          v-show="type === 'object' || type === 'tuple'"
          class="flex-none"
        >
          <button
            v-tooltip="t('add')"
            @click="addMember"
            class="btn btn-xs btn-soft btn-accent btn-circle mr-1"
          >
            <Icon size="15" :path="mdiPlus"></Icon>
          </button>
        </div>
      </div>
      <!-- ? 处理复杂嵌套组件和动态添加/删除条目时，如果需要频繁切换显示状态，使用 v-show 通常比 v-if 更稳定。 -->
      <div
        class="overflow-y-auto overflow-x-hidden"
        :style="`max-height: calc(${maxHeight} - 10rem)`"
        v-show="['object', 'array', 'tuple'].includes(type)"
      >
        <json-node
          v-for="(value, key) in treeData"
          :key="key"
          :property-key="key"
          :property-value="value"
          :path="[key]"
          :draggable
          :format
          :readonly
          :readonly-key="type === 'tuple' || type === 'array'"
          :is-shadowed="currentEditingingKey === key"
          :allow-delete="type !== 'array'"
          @update:value="updateValue([key], $event)"
          @update:key="updateKey([key], $event)"
          @editing:key="currentEditingingKey = $event"
          @delete="deleteProperty([key])"
          @cancel="isAdding = false"
          @comfirm="comfirmAdding"
        >
          <template #add="scope">
            <slot name="add" v-bind="scope"></slot>
          </template>
          <template #value="scope">
            <slot name="value" v-bind="scope"></slot>
          </template>
        </json-node>
      </div>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watchEffect } from "vue";
import JsonNode from "./JsonNode.vue";
import { provide } from "vue";
import { mdiPlus } from "@mdi/js";
import Icon from "./Icon.vue";
import {
  defaultType,
  isArray,
  isObject,
  isTuple,
  createTypeMap,
  typeMapI18n,
  isJsonNode,
} from "./JsonNode.utils";
import { deepClone } from "../utils/object";
import { useI18n } from "vue-i18n";
import Dropdown from "./Dropdown.vue";
import { vTooltip } from "../directives/tooltip";
const typeMap = createTypeMap();

// i18n
const { t } = useI18n();

defineProps<{
  // 格式化函数，将值格式化为字符串用于展示，isNewValue 为 true 时，表示新添加的成员
  format?: (value: any, isNewValue: boolean) => string;
  readonly?: boolean;
  maxHeight?: string;
  draggable?: boolean;
}>();

const modelValue = defineModel<Record<string, any>>({ default: {} });

// 新添加的成员的占位符
const newSymbol = Symbol.for("new");

// 记录当前类型
const type = ref<keyof typeof typeMap>("object");

onMounted(() => {
  // 如果modelValue已经是JsonNode，从中读取类型
  if (isJsonNode(modelValue.value)) {
    type.value = modelValue.value["\0type"];
  } else {
    // 如果modelValue有内容但不是JsonNode，保留内容并添加type标记
    if (Object.keys(modelValue.value).length > 0) {
      // 检测数据类型并添加相应的type标记
      const hasArrayIndexKeys = Object.keys(modelValue.value).every(key => /^\[\d+\]$/.test(key));
      if (hasArrayIndexKeys && Object.keys(modelValue.value).length > 0) {
        // 看起来像元组
        modelValue.value = { "\0type": "tuple", ...modelValue.value };
        type.value = "tuple";
      } else if (modelValue.value["[member]"] !== undefined) {
        // 看起来像数组
        modelValue.value = { "\0type": "array", ...modelValue.value };
        type.value = "array";
      } else {
        // 普通对象
        modelValue.value = { "\0type": "object", ...modelValue.value };
        type.value = "object";
      }
    } else {
      // 空对象，初始化为object类型
      modelValue.value = { "\0type": "object" };
      type.value = "object";
    }
  }
});

function updateType(key: keyof typeof typeMap) {
  modelValue.value = { "\0type": key };
  type.value = key;
}

// 分别储存数组，元组，对象，防止切换类型后导致丢失，造成不好的体验
const arrayStore = ref(typeMap.array);
const tupleStore = ref(typeMap.tuple);
const objectStore = ref(typeMap.object);

watchEffect(() => {
  // 如果modelValue有type标记，使用该标记并更新对应的store
  if (isJsonNode(modelValue.value)) {
    const nodeType = modelValue.value["\0type"];
    if (type.value !== nodeType) {
      type.value = nodeType;
    }
    
    // 更新对应的store以保持数据同步
    if (nodeType === "array") {
      // @ts-ignore 类型检查已通过isJsonNode确认
      arrayStore.value = modelValue.value;
    } else if (nodeType === "object") {
      // @ts-ignore 类型检查已通过isJsonNode确认
      objectStore.value = modelValue.value;
    } else if (nodeType === "tuple") {
      // @ts-ignore 类型检查已通过isJsonNode确认
      tupleStore.value = modelValue.value;
    }
    return;
  }

  // 如果modelValue不是JsonNode但有内容，尝试自动转换
  if (Object.keys(modelValue.value).length > 0) {
    const hasArrayIndexKeys = Object.keys(modelValue.value).every(key => /^\[\d+\]$/.test(key));
    if (hasArrayIndexKeys) {
      // 转换为元组
      modelValue.value = { "\0type": "tuple", ...modelValue.value };
      type.value = "tuple";
    } else if (modelValue.value["[member]"] !== undefined) {
      // 转换为数组
      modelValue.value = { "\0type": "array", ...modelValue.value };
      type.value = "array";
    } else {
      // 转换为对象
      modelValue.value = { "\0type": "object", ...modelValue.value };
      type.value = "object";
    }
    return;
  }

  // 检测当前modelValue的实际类型并同步（兼容旧逻辑）
  let detectedType: keyof typeof typeMap | null = null;
  
  if (isArray(modelValue.value)) {
    detectedType = "array";
    // @ts-ignore 类型检查已通过isArray确认
    arrayStore.value = modelValue.value;
  } else if (isObject(modelValue.value)) {
    detectedType = "object";
    // @ts-ignore 类型检查已通过isObject确认
    objectStore.value = modelValue.value;
  } else if (isTuple(modelValue.value)) {
    detectedType = "tuple";
    // @ts-ignore 类型检查已通过isTuple确认
    tupleStore.value = modelValue.value;
  }

  // 如果检测到类型，更新type.value
  if (detectedType && type.value !== detectedType) {
    type.value = detectedType;
  }
});

//#region 添加/修改成员

// 是否正在添加成员
const isAdding = ref(false);

// 计算树状结构
const treeData = computed({
  get: () => {
    isAdding.value; // 触发依赖收集
    // 根据当前类型返回对应的树状结构
    if (type.value === "array") {
      // 数组只包含一个成员，用于表示该数组的成员类型，不能添加新成员
      return arrayStore.value;
    } else if (type.value === "tuple") {
      // 元组可以添加新成员，但其成员名不可修改，其添加成员的逻辑在addMember中
      return tupleStore.value;
    }
    // 对象可以添加新成员，其成员名可以修改
    return isAdding.value
      ? { ...objectStore.value, [""]: newSymbol }
      : objectStore.value;
  },
  set: (value: any) => {
    // 三种情况的操作大致相同，存储至对应的存储器中 > 更新 modelValue
    // 但元组需要将成员名转换为 [index] 的形式
    const storeMap = {
      array: arrayStore,
      tuple: tupleStore,
      object: objectStore,
    };
    if (type.value === "tuple") {
      // 将元组转换为 [index] 的形式
      const result: Record<string, any> = {};
      let index = 0;
      for (const [key, val] of Object.entries(value)) {
        if (key.startsWith("\0")) {
          result[key] = val;
        } else {
          result[`[${index++}]`] = val;
        }
      }
      value = result;
    }
    // 不可能是基本类型，因为仅有引用类型才会显示json-node，基本类型不可能触发set
    storeMap[type.value as "object" | "array" | "tuple"].value = value;
    modelValue.value = value;
  },
});

function addMember() {
  // 元组添加成员的逻辑
  if (type.value === "tuple") {
    /* ? 为什么要这么做? */
    /*
    新增成员的逻辑是 isAdding == true >> treeData 产生新的未确定成员(成员名为"")
    >> 自动focus到该成员名 >> blur时判断是否为空，为空则取消，否则确定

    然而元组成员名不可修改，因此不能focus，自然也不会blur，所以不会触发comfirm事件，
    而isAdding在comfirm事件中被重置为false，因此isAdding会保持为true，
    此时进行修改时，treeData的触发set，其value中包含新成员，
    而isAdding仍为true，其get会返回包含未确定成员的treeData，
    最终导致每进行一次修改，都会添加一个新成员，故不能采用操作isAdding的方案。

    又因为元组成员名不能修改，所以也不会触发cancel，因此便可直接将新成员加入其中
     */
    tupleStore.value[`[${Object.keys(tupleStore.value).length - 1}]`] =
      newSymbol;
    doDelete();
  } else {
    isAdding.value = true;
  }
}

// 确认添加成员
function comfirmAdding(key: string) {
  // 先重置isAdding，这样treeData的值就不再包含未确定的新成员了
  isAdding.value = false;
  // 创建新数据，避免直接修改treeData
  const newData: Record<string, any> = { ...treeData.value };
  // 添加新成员并更新
  newData[key] = defaultType;
  treeData.value = newData;
  doDelete();
}

// 更新成员名
function updateKey(path: string[], newKey: string) {
  // 创建新数据，避免引用类型导致数据被修改
  const newData = deepClone<any>(treeData.value);

  // 遍历到倒数第二层, 并储存倒数第三层，用于后续的更新
  let current = newData; // 储存倒数第二层
  let lastThird = null; // 储存倒数第三层
  // 通过path来遍历到倒数第二层
  for (let i = 0; i < path.length - 1; i++) {
    if (i === path.length - 2) {
      // 储存倒数第三层
      lastThird = current;
    }
    // 遍历到倒数第二层
    current = current[path[i]];
  }
  // 因为在不能直接修改对象的值，所以需要创建一个新的容器
  const newContainer: Record<string, any> = {};
  // Object.entries的遍历顺序为元素的插入顺序
  // 只要在遍历到要修改键名的元素时，将该元素的值储存在newContainer中，并用新键名作键
  // 便可以只修改键名，而保证原有的对象结构不变
  const targetKey = path.at(-1);
  Object.entries(current).forEach(([key, value]) => {
    // 如果遍历到的键名与目标键名相同，则将该元素的值储存在newContainer中，并用新键名作键
    if (key === targetKey) {
      newContainer[newKey] = value;
    } else {
      // 否则，正常存储即可
      newContainer[key] = value;
    }
  });
  // 如果倒数第三层存在，则将newContainer储存在倒数第三层中
  // 也就是修改的键在嵌套结构中
  /*
  treeData = {
    "a": {
      "b": {
        "c": value // 修改c的键名
      }
    }
  }
  需要获取b的值(倒数第二层)，以及a的值(倒数第三层)
  修改b的值为newContainer，并储存在a中
  */
  if (lastThird && path.at(-2)) {
    // 如果倒数第三层存在，则将newContainer储存在倒数第三层中
    lastThird[path.at(-2)!] = newContainer;
    treeData.value = newData;
  } else {
    // 否则，直接修改treeData
    /*
    treeData = {
      "a": {
        "b": value // 修改b的键名
      }
    }
    需要获取a的值(倒数第二层)，修改a的值为newContainer，并储存在treeData中
    */
    treeData.value = newContainer;
  }
}

// 更新成员的值
function updateValue(path: string[], newValue: any) {
  // 与更新成员名类似
  const newData = deepClone<any>(treeData.value);
  let current = newData;

  // 遍历到倒数第二层
  for (let i = 0; i < path.length - 1; i++) {
    current = current[path[i]];
  }

  // 更新最后一层的值
  current[path.at(-1)!] = newValue;
  treeData.value = newData;
}

function deleteProperty(path: string[]) {
  // 与更新成员名类似
  const newData = deepClone<any>(treeData.value);
  let current = newData;

  // 遍历到倒数第二层
  for (let i = 0; i < path.length - 1; i++) {
    current = current[path[i]];
  }

  // 删除属性
  delete current[path[path.length - 1]];
  treeData.value = newData;
}
//#endregion

//#region 遮蔽
// 当新成员名与已经存在的成员名相同时，新成员会替代旧成员
// 为避免意外重名而导致成员被替代，产生不好的体验
// 因此需要判断是否遮蔽，并给出提示

// 正在修改的成员名，用于判断是否遮蔽
const currentEditingingKey = ref<string | null>(null);

// 监听isAdding和treeData，并重置currentEditingingKey
watchEffect(() => {
  isAdding.value;
  treeData.value;
  currentEditingingKey.value = null;
});

//#endregion

//#region 延迟删除
// ? 执行删除操作之后不会立即删除，而是会延迟到下一个有效操作(如添加，修改，删除)之后执行
// ? 在删除之前可以撤销

// 存储删除操作
const willDelete = ref<() => void>();

function doDelete() {
  willDelete.value?.();
  willDelete.value = undefined;
}

function cancelDelete() {
  willDelete.value = undefined;
}

// 提供更新删除操作的函数
provide("update:willDelete", (fn: () => void) => {
  willDelete.value = fn;
});

// 执行删除操作，执行后重置
provide("do:delete", doDelete);

// 撤销删除操作
provide("cancel:delete", cancelDelete);

//#endregion

//#region 杂项

//#endregion
</script>

<style scoped>
@reference "../main.css";

.json-tree-viewer {
  font-family: var(
    --default-mono-font-family,
    ui-monospace,
    SFMono-Regular,
    Menlo,
    Monaco,
    Consolas,
    "Liberation Mono",
    "Courier New",
    monospace
  );
}
[contenteditable]:focus {
  outline: none;
  border-bottom: 1px solid #fff;
}
[contenteditable] {
  @apply px-2 py-1 hover:bg-base-200;
}

.toolbar {
  margin-bottom: 8px;
}
</style>

<i18n lang="yaml">
zh-CN:
  title: 类型：
  add: 添加成员
en:
  title: "Type:"
  add: "Add member"
</i18n>
