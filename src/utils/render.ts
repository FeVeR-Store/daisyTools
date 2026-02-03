import {
  Component,
  defineComponent,
  h,
  isVNode,
  RenderFunction,
  SetupContext,
  VNode,
} from "vue";
import { process } from "./render.ext";

/**
 * 可渲染的内容类型
 * @template T - 组件 props 类型
 */
export type Renderable<T = any> =
  | void
  | ((props: T, ctx: SetupContext) => RenderFunction | Promise<RenderFunction>)
  | VNode
  | VNode[]
  | Component<T>
  | string;

export function normalizeRenderable<T = object>(
  renderable: Renderable<T> | any,
  props: T
): VNode | VNode[] | Component {
  props ??= {} as any;
  {
    const result = process(renderable, props);
    if (result) return result;
  }
  if (!renderable) return h("div", "");
  if (typeof renderable == "function") {
    // ! 仅支持setup
    // ? 传入的是setup函数，需要使用defineComponent包裹
    return h(
      defineComponent(
        renderable as (
          props: any,
          ctx: SetupContext
        ) => RenderFunction | Promise<RenderFunction>,
        { props: Object.keys(props!) }
      ),
      props
    );
  } else if (typeof renderable == "string") {
    // ? 如果传入的是string，那么需要使用div包裹
    // ? 使用span会导致样式问题
    return h("div", {}, renderable);
  } else if (Array.isArray(renderable)) {
    return renderable.map((renderable) =>
      normalizeRenderable(renderable, props)
    );
  } else if (typeof renderable == "object") {
    // ? 如果传入的是VNode, Component，那么直接返回
    if (isVNode(renderable)) {
      return renderable;
    }
    return renderable;
  }
  return renderable;
}
