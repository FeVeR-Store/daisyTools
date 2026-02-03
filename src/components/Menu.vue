<template>
  <ul class="menu bg-primary rounded-box w-56 h-full [flex-flow:column]">
    <SubMenu
      :path="[key]"
      :draggableId
      v-for="(item, key) in items"
      :item
      :active-menu
    ></SubMenu>
  </ul>
</template>

<script setup lang="ts" generic="T">
import { join } from "path-browserify";
import {
  Component,
  computed,
  defineComponent,
  h,
  inject,
  InjectionKey,
  provide,
  Ref,
} from "vue";
import { useRouter } from "vue-router";
import Draggable from "../utils/components/Draggable.vue";

export type MenuItem = {
  id?: string;
  label: string;
  icon?: string;
  url?: string;
  loading?: boolean;
  onClick?: (item: MenuItem) => void;
  children?: MenuItem[];
  payload?: any;
};

const {
  items: _items,
  routerMethod = "none",
  path: basePath = "/",
  draggableId = null,
} = defineProps<{
  items: MenuItem | MenuItem[];
  routerMethod?: "push" | "replace" | "none";
  draggableId?: symbol;
  path?: string;
}>();

const activeMenu = defineModel<string | null>("active", { default: null });
const active_key = Symbol() as InjectionKey<{
  active: (menuItem: string) => void;
  activeMenu: Ref<string | null>;
}>;
provide(active_key, {
  active: (item) => (activeMenu.value = item),
  activeMenu,
});

const items = computed(() => {
  if (Array.isArray(_items)) {
    return _items;
  } else {
    return [_items];
  }
});

const router = useRouter();

const SubMenu: Component<{
  item: MenuItem;
  draggableId: symbol | null;
  path: number[];
}> = defineComponent(
  (props) => {
    const { active, activeMenu } = inject(active_key)!;
    return () => {
      const { item, draggableId, path } = props;
      return h(
        "li",
        item.children
          ? item.loading
            ? h("a", { class: "justify-between" }, [
                props.item.label,
                h("span", {
                  class:
                    "loading loading-spinner loading-xs scale-95 translate-x-[0.3rem]",
                }),
              ])
            : h("details", { open: true }, [
                h("summary", props.item.label),
                h(
                  "ul",
                  props.item.children!.map((item, key) =>
                    h(SubMenu, { item, draggableId, path: [...path, key] })
                  )
                ),
              ])
          : h(
              Draggable,
              {
                id: draggableId ?? Symbol(),
                data: item.payload,
                disable: !draggableId,
                onClick: () => {
                  active(item.id!);
                  item.onClick
                    ? item.onClick(item)
                    : routerMethod === "none"
                    ? null
                    : item.url &&
                      router[routerMethod](join(basePath, item.url));
                },
              },
              {
                default: () =>
                  h(
                    "a",
                    {
                      class: activeMenu.value === item.id ? "menu-active" : "",
                    },
                    item.label
                  ),
                dragging: () =>
                  h(
                    "div",
                    {
                      class: "badge opacity-20 badge-primary min-w-15",
                    },
                    item.label
                  ),
              }
            )
      );
    };
  },
  { props: ["item", "draggableId", "path"] }
);
</script>

<i18n lang="yaml">
zh-CN:
  loading: 加载中...
en:
  loading: Loading...
</i18n>
