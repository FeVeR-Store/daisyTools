<!-- include: btn-warning | btn-error | btn-accent | btn-info | btn-success | btn-secondary -->
<!-- include: btn-xs | btn-lg | btn-md -->
<!-- include: btn-outline | btn-dash | btn-soft -->
<template>
  <div
    @click.stop
    :class="[btns.length > 1 ? 'action-button-group' : 'single-button-group']"
  >
    <SelfOwned
      v-for="(
        {
          tooltip,
          class: btnClass,
          type,
          callback,
          style = 'soft',
          icon,
          disable = false,
          shortcutKey,
          flash,
        },
        index
      ) in btns"
      :key="index"
      :default-value="false"
      #="{ modelValue: hover, update }"
    >
      <Alias
        :value="toValue(shortcutKey) && (hover || matchKeys(shortcutKey!))"
        #="{ value: showShortcut }"
      >
        <button
          v-element-hover="update"
          :disabled="toValue(disable)"
          v-tooltip:[hover]="hover ? toValue(tooltip) : ''"
          class="action-btn btn btn-circle"
          :class="[
            toValue(btnClass),
            'btn-' + toValue(type),
            'btn-' + size[0],
            toValue(style) !== 'solid' ? 'btn-' + toValue(style) : '',
            { hover: showShortcut },
            { 'animate-flash-once': toValue(flash) },
          ]"
          v-event-delegation:[callback].click="
        (callback:any) => (isRef(callback) ? toValue(callback) : callback)()
      "
        >
          <Badge
            -style="outline"
            :icon="mdiKeyboardVariant"
            class="shortcut transition-all"
          >
            <div class="m-auto font-[400]">
              <Join :list="toValue(shortcutKey)!">
                <template #="{ element }">
                  <span
                    :class="[
                      'transition-all',
                      {
                        'active-key': current.has(element.toLowerCase()),
                      },
                    ]"
                  >
                    {{ element }}
                  </span>
                </template>
                <template #separator> + </template>
              </Join>
            </div>
          </Badge>
          <Icon :size="size[1]" :path="toValue(icon)"></Icon>
        </button>
      </Alias>
    </SelfOwned>
  </div>
</template>

<script setup lang="ts">
import {
  computed,
  isRef,
  MaybeRefOrGetter,
  ref,
  Ref,
  toValue,
  watchEffect,
} from "vue";
import { vTooltip } from "../directives/tooltip";
import Icon from "./Icon.vue";
import { ShortcutKeyType, useShortcutKeys } from "../composable/useKeys";
import { mdiKeyboardVariant } from "@mdi/js";
import Badge from "./Badge.vue";
import { vElementHover } from "@vueuse/components";

import { vEventDelegation } from "../directives/eventDelegation";
import SelfOwned from "../utils/components/SelfOwned.vue";
import Alias from "../utils/components/Alias.vue";
import Join from "../utils/components/Join.vue";
const { on, stop, current } = useShortcutKeys();

const { btns: _btns, size: _size = "sm" } = defineProps<{
  btns: ActionButton[];
  size?: "sm" | "md" | "lg";
}>();

type _ActionButton = ActionButton & { flash: Ref<boolean> };

const btns = computed<_ActionButton[]>(() => {
  return (_btns as _ActionButton[])
    .filter((btn) => btn.show ?? true)
    .map((btn) => ((btn.flash = ref(false)), btn));
});

watchEffect(() => {
  stop();
  btns.value.forEach((btn) => {
    const { shortcutKey, callback } = btn;
    const keys = toValue(shortcutKey);
    if (keys) {
      let timer: NodeJS.Timeout | null;
      on(keys, () => {
        (isRef(callback) ? callback.value : callback)();
        btn.flash.value = true;
        if (!timer) {
          timer = setTimeout(() => {
            btn.flash.value = false;
            timer = null;
          }, 500);
        }
      });
    }
  });
});

const size = computed(() => {
  return { sm: ["xs", 15, 1], md: ["sm", 16, 1.5], lg: ["md", 18, 2] }[_size];
});

function matchKeys(shortcutKey: MaybeRefOrGetter<ShortcutKeyType>) {
  const keys = toValue(shortcutKey);
  if (current.size === 0) return false;
  let match = true;
  toValue(current).forEach((key) => {
    if (keys.findIndex((k) => k.toLowerCase() === key) === -1) {
      match = false;
    }
  });
  return match;
}
</script>

<style scoped>
@reference '../main.css';

.action-button-group {
  --padding-y: v-bind("size[2]");
  @apply rounded-4xl py-[calc(0.25rem*var(--padding-y))] px-[calc(0.5rem*var(--padding-y))] mr-1 bg-base-200 gap-2 flex;
}
.single-button-group {
  @apply pr-3;
}
.action-btn {
  @apply flex transition-all px-0;
  --shortcut-display: none;
  --shortcut-opacity: 0;
  --delay: 0;
  & {
    transition: all 0.3s var(--delay), color 0.3s, border 0.3s,
      background-color 0.3s;
  }
  &.hover {
    @apply px-4 w-fit;
    --delay: 1s;
  }
  & .shortcut {
    transition: opacity 0.1s;
    margin-right: 0.25rem;
    opacity: var(--shortcut-opacity);
    display: var(--shortcut-display);
  }
  &.hover {
    animation: show-shortcut 0.5s 1s forwards;
  }
}
.active-key {
  @apply font-black underline;
}
@keyframes show-shortcut {
  0% {
    --shortcut-opacity: 0;
    --shortcut-display: flex;
  }
  70% {
    --shortcut-opacity: 0;
    --shortcut-display: flex;
  }
  100% {
    --shortcut-opacity: 1;
    --shortcut-display: flex;
  }
}
</style>

<style></style>

<script lang="ts">
export type ActionButton = {
  show?: MaybeRefOrGetter<boolean>;
  disable?: MaybeRefOrGetter<boolean>;
  icon: MaybeRefOrGetter<string>;
  callback: Ref<() => void> | (() => void);
  tooltip?: MaybeRefOrGetter<string>;
  class?: MaybeRefOrGetter<string>;
  style?: MaybeRefOrGetter<"outline" | "soft" | "dash" | "solid">;
  shortcutKey?: MaybeRefOrGetter<ShortcutKeyType>;
  type: MaybeRefOrGetter<
    | "warning"
    | "error"
    | "accent"
    | "info"
    | "success"
    | "secondary"
    | "primary"
  >;
};
</script>
