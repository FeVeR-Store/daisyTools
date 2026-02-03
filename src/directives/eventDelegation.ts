import { DirectiveBinding } from "vue";

const map = new WeakMap();
export function vEventDelegation(el: HTMLElement, binding: DirectiveBinding) {
  const { value, modifiers, arg } = binding;
  map.set(el, arg);
  if (
    el.parentElement &&
    !el.parentElement.classList.contains("event-delegation")
  ) {
    Object.entries(modifiers).forEach(([event, enable]) => {
      if (enable) {
        el.parentElement?.addEventListener(event, (e) => {
          let current = e.target as HTMLElement;
          if (!current) return;
          for (;;) {
            current = current.parentElement!;
            if (map.has(current)) {
              break;
            } else if (current === el.parentElement || !current.parentElement) {
              return;
            }
          }
          if (map.has(current)) {
            value(map.get(current));
          }
        });
      }
    });
    el.parentElement.classList.add("event-delegation");
  }
}
