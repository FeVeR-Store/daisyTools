import { DirectiveBinding } from "vue";

/*
z-index: 0 底层
z-index: 1 普通元素
z-index: 5 mask(visible)
z-index: 10 mask(invisible)
*/

let exists = false;
/**
 * onClickOutside
 * @prop {Function} 回调函数
 * @arg {boolean} 是否启用
 */
export function vClickOutside(el: HTMLElement, binding: DirectiveBinding) {
  if (binding.arg) {
    if (exists) return;
    exists = true;
    const mask = document.createElement("div");
    mask.id = "click-outside-mask";
    mask.style.position = "fixed";
    mask.style.top = "4rem";
    mask.style.left = "0";
    mask.style.width = "100%";
    mask.style.height = "100%";
    mask.style.zIndex = "10";

    el.style.zIndex = "11";

    mask.addEventListener("click", () => {
      binding.value();
      mask.remove();
      exists = false;
    });
    document.body.append(mask);
  } else {
    const mask = document.getElementById("click-outside-mask");
    if (mask) {
      mask.remove();
      exists = false;
    }
  }
}
