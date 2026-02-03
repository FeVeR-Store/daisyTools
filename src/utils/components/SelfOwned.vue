<template>
  <!-- @vue-ignore -->
  <slot :[name]="value" :[`onUpdate:${name}`]="onUpdate" :update="onUpdate">
  </slot>
</template>

<script setup lang="ts" generic="T, const R extends string = 'modelValue'">
/**
 * 创建一个自持有的model，用于不关注某个组件的model值，但是组件需要使用model值时\
 * 只需使用v-bind绑定插槽作用域导出的值即可
 *
 * @example
 * ```html
 * <SelfOwned defaultValue="tab1" #="value">
 *   <!-- 实际上，我们并不在意当前的tab值 -->
 *   <TabView :tabs="['tab1', 'tab2']" v-bind="value">
 *   <!-- 相当于：<TabView ... v-model="xxx"> -->
 *     ...
 *   </TabView>
 * </SelfOwned>
 * ```
 * @example
 * ```html
 * <!-- 有时候，我们可能想手动控制值的更新 -->
 * <SelfOwned defaultValue="tab1" #="{ modelValue: value, update }">
 *   <p>count: {{ value }} </p>
 *   <button v-on:click="update(value + 1)">value + 1</button>
 *   <button v-on:click="update(value - 1)">value - 1</button>
 * </SelfOwned>
 * ```
 *
 */
import { ref } from "vue";
defineOptions({
  inheritAttrs: false,
});
const { defaultValue, name = "modelValue" } = defineProps<{
  defaultValue: T;
  name?: R;
}>();

const value = ref<T>(defaultValue);
const onUpdate = (val: T) => {
  value.value = val;
};

defineSlots<{
  default(props: {
    [key in R | "update" | `onUpdate:${R}`]: key extends
      | "update"
      | `onUpdate:${R}`
      ? typeof onUpdate
      : T;
  }): any;
}>();
</script>
