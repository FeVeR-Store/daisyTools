import { Directive, DirectiveBinding } from "vue";

const map = new WeakMap<HTMLElement, EventListenerOrEventListenerObject[]>();

export const vOnHover: Directive<any, (arg: { isHover: boolean }) => void> = {
  mounted: (el: HTMLElement, binding: DirectiveBinding) => {
    const { arg = "includeChild", value } = binding;
    const events =
      arg === "includeChild"
        ? <const>["mouseover", "mouseout"]
        : <const>["mouseenter", "mouseleave"];
    // 鼠标移入
    const mouseEnterHandler = (e: Event) => {
      // 鼠标按键按下的时候hover元素，比如拖拽，效果会很奇怪吧
      if ((e as MouseEvent).buttons !== 0) return;
      value({ isHover: true });
    };
    // 鼠标移出
    const mouseLeaveHandler = () => {
      value({ isHover: false });
    };
    el.addEventListener(events[0], mouseEnterHandler);
    el.addEventListener(events[1], mouseLeaveHandler);
    map.set(el, [mouseEnterHandler, mouseLeaveHandler]);
  },
  unmounted(el: HTMLElement, binding: DirectiveBinding) {
    const listener = map.get(el);
    if (!listener) return;
    const events =
      binding.arg === "includeChild"
        ? <const>["mouseover", "mouseout"]
        : <const>["mouseenter", "mouseleave"];
    el.removeEventListener(events[0], listener[0]);
    el.removeEventListener(events[1], listener[1]);
  },
};
