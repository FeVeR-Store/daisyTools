/// <reference types="vite/client" />
/// <reference path="../node_modules/reflect-metadata/standalone.d.ts" />
declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

namespace NodeJS {
  interface ProcessEnv {
    target: "android" | "windows" | "linux" | "macos" | "ios";
    mode: "production" | "development";
  }
}

declare module "vue3-smooth-dnd" {
  import Vue from "vue";

  type Payload = any;

  interface DropResult {
    removedIndex: number;
    addedIndex: number;
    payload: Payload;
    element: Element;
  }

  interface DragEvent {
    isSource: boolean;
    payload: Payload;
    willAcceptDrop: boolean;
  }

  interface NodeDescription {
    value: string;
    props: Vue.VNodeData;
  }

  interface ContainerProps {
    orientation?: string;
    behaviour?: string;
    tag?: string | NodeDescription;
    groupName?: string;
    lockAxis?: string;
    dragHandleSelector?: string;
    nonDragAreaSelector?: string;
    dragBeginDelay?: number;
    animationDuration?: number;
    autoScrollEnabled?: boolean;
    dragClass?: string;
    dropClass?: string;
    removeOnDropOut?: boolean;
    getChildPayload?: (index: number) => Payload;
    shouldAnimateDrop?: (
      sourceContainerOptions: ContainerProps,
      payload: Payload
    ) => boolean;
    shouldAcceptDrop?: (
      sourceContainerOptions: ContainerProps,
      payload: Payload
    ) => boolean;
    getGhostParent: () => Element;
    onDragStart?: (dragEvent: DragEvent) => void;
    onDragEnd?: (dragEvent: DragEvent) => void;
    onDrop?: (dropResult: DropResult) => void;
    onDragEnter?: () => void;
    onDragLeave?: () => void;
    onDropReady?: (dropResult: DropResult) => void;
  }

  class Container extends Vue {
    props: ContainerProps;
  }

  class Draggable extends Vue {
    props: {
      tag?: string | NodeDescription;
    };
  }
}
