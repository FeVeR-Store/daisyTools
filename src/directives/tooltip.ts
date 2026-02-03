import {
  createApp,
  defineComponent,
  DirectiveBinding,
  h,
  Ref,
  ref,
  Directive,
  MaybeRefOrGetter,
  toValue,
} from "vue";
import Tooltip, { TooltipProps } from "../utils/components/Tooltip.vue";

const map = new WeakMap<HTMLDivElement, Ref<Map<number, TooltipProps>>>();

function useTooltip() {
  const container = document.querySelector<HTMLDivElement>(
    "body > div.tooltip-container"
  );
  if (container) return map.get(container)!;

  const tooltipContainer = document.createElement("div");
  tooltipContainer.classList.add("tooltip-container");
  document.body.appendChild(tooltipContainer);
  const tooltips = ref<Map<number, TooltipProps>>(new Map());
  const app = createApp(
    defineComponent(
      ({
        tooltips,
      }: {
        tooltips: MaybeRefOrGetter<Map<number, TooltipProps>>;
      }) => {
        return () =>
          Array.from(toValue(tooltips).values()).map((tooltip) =>
            h(Tooltip, tooltip)
          );
      },
      { props: ["tooltips"] }
    ),
    { tooltips }
  );
  app.mount(tooltipContainer);
  map.set(tooltipContainer, tooltips);
  return tooltips;
}

const tooltipMap = new WeakMap<HTMLElement, [number, (() => void)[]]>();

const handler = (
  el: HTMLElement,
  binding: DirectiveBinding<
    string | undefined,
    "top" | "bottom" | "left" | "right",
    // @ts-ignore arg 可传递boolean
    boolean
  >
) => {
  const position = (Object.keys(binding.modifiers)?.[0] as "top") ?? "top";
  const content = binding.value ?? "";
  const force = !!binding.arg;
  const tooltips = useTooltip();
  if (!tooltipMap.has(el)) {
    tooltipMap.set(el, [Math.random(), []]);
  }

  const mouseEnterHandler = () => {
    const { top, left, width } = el.getBoundingClientRect();
    const [id] = tooltipMap.get(el)!;
    if (tooltips.value.get(id)?.show) return;
    tooltips.value.set(id, {
      x: left + width / 2,
      y: top,
      content,
      position,
      force,
      show: true,
    });
  };

  if (force) {
    mouseEnterHandler();
    return;
  }
  {
    const [id, handlers] = tooltipMap.get(el)!;

    const [mouseEnterHandler = () => {}, mouseLeaveHandler = () => {}] =
      handlers;
    el.removeEventListener("mouseenter", mouseEnterHandler);
    el.removeEventListener("mouseleave", mouseLeaveHandler);
    el.classList.remove("cursor-help");
    if (!content) {
      tooltips.value.delete(id);
      return;
    }
  }
  el.classList.add("cursor-help");
  const mouseLeaveHandler = () => {
    const [id] = tooltipMap.get(el)!;
    tooltips.value.get(id)!.show = false;
  };
  el.addEventListener("mouseenter", mouseEnterHandler);
  el.addEventListener("mouseleave", mouseLeaveHandler);
  tooltipMap.get(el)![1][0] = mouseEnterHandler;
  tooltipMap.get(el)![1][1] = mouseLeaveHandler;
};

/**
 * Tooltip 指令
 * @Arg {boolean} [force] - 是否强制显示
 * @Modifier {string = "top"} ["top" | "bottom" | "left" | "right"] - Tooltip 的位置
 * @prop {string} [content] - Tooltip 的内容
 * @example 鼠标悬停时显示 tooltip
 * ```vue
 * <div v-tooltip="content"></div>
 * ```
 * @example 强制在左侧显示 tooltip
 * ```vue
 * <div v-tooltip[force].left="content"></div>
 * ```
 */
export const vTooltip: Directive<
  HTMLElement,
  string | undefined,
  "top" | "bottom" | "left" | "right",
  // @ts-ignore arg 可传递boolean
  boolean
> = {
  mounted: handler,
  updated: handler,
  unmounted: (el) => {
    const tooltips = useTooltip();
    const [id, handlers] = tooltipMap.get(el)!;
    tooltips.value.delete(id);
    const [mouseEnterHandler = () => {}, mouseLeaveHandler = () => {}] =
      handlers;
    el.removeEventListener("mouseenter", mouseEnterHandler);
    el.removeEventListener("mouseleave", mouseLeaveHandler);
  },
};
