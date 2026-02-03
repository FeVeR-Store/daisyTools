<template>
  <div class="relative">
    <div
      class="w-full"
      :class="{ 'join flex': $slots.append || $slots.prepend }"
    >
      <slot name="prepend" className="join-item"></slot>
      <label
        class="input join-item w-full px-0 focus-within:outline-none"
        :class="{ 'border-error': invalidated }"
      >
        <div v-if="$slots.prefix" class="pl-4">
          <slot name="prefix"></slot>
        </div>
        <div class="w-full h-full relative">
          <input
            :placeholder="showOptions ? '' : placeholder"
            v-model="inputValue"
            type="text"
            class="input w-full focus:outline-none"
            :class="{
              'rounded-l-sm': !$slots.prepend,
              'rounded-r-sm': !$slots.append,
            }"
            @blur="showOptions = false"
            @focus="showOptions = true"
            @input="(currentSelected = 0), (showOptions = true)"
            @keydown.up.prevent="currentSelected > 0 && currentSelected--"
            @keydown.down.prevent="
              currentSelected < options.length - 1 && currentSelected++
            "
            @keydown.tab.prevent="handleClick(options[currentSelected])"
            @keydown.enter.prevent="
              () => {
                handleClick(options[currentSelected]);
                showOptions = false;
              }
            "
          />
          <span
            v-if="showOptions"
            class="opacity-40 absolute top-[0.62rem] left-[0.8rem]"
            >{{ options[currentSelected] }}</span
          >
        </div>
        <div v-if="$slots.suffix" class="mr-4">
          <slot name="suffix"></slot>
        </div>
      </label>
      <slot name="append" className="join-item"></slot>
    </div>

    <ul
      class="menu flex-row bg-base-300 border-gray-500 border-1 rounded-box w-full absolute z-10 max-h-60 overflow-y-auto"
      v-if="
        showOptions &&
        (options.length > 1 ||
          (options.length === 1 && inputValue !== options[0]))
      "
    >
      <li
        class="w-full"
        v-for="(option, index) in options"
        :key="option"
        v-event-delegation:[option].mousedown="
          /* ? 使用mousedown，事件触发顺序 mousedown > blur > click > mouseup */
          handleClick
        "
      >
        <a
          :class="{
            'bg-base-100 border-[0.5px] border-gray-600':
              currentSelected === index,
          }"
          >{{ option }}</a
        >
      </li>
    </ul>
    <p v-show="strict && invalidated" class="text-sm text-error">
      {{ strict }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { vEventDelegation } from "../../directives/eventDelegation";
const props = withDefaults(
  defineProps<{
    options: string[];
    strict?: boolean | string;
    placeholder?: string;
  }>(),
  { strict: false }
);

const invalidated = computed(() => {
  return props.strict && options.value[0] !== inputValue.value;
});
const inputValue = defineModel<string>();
const currentSelected = ref(0);
const showOptions = ref(false);

const options = computed(() => {
  return props.options
    .filter((option) =>
      option.toLowerCase().startsWith((inputValue.value ?? "").toLowerCase())
    )
    .sort((a, b) => a.localeCompare(b));
});

const handleClick = (option: string) => {
  console.log(option);
  inputValue.value = option;
  showOptions.value = false;
};

defineExpose({
  validate: () => invalidated.value,
});
</script>
