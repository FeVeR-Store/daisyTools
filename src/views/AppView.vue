<template>
    <ToastProvider ref="toastProvider"></ToastProvider>
    <div class="h-screen">
        <div data-tauri-drag-region class="mockup-window h-fit relative">
            <div class="window-decorations"></div>
            <div
                class="absolute top-0 left-0 w-full h-full flex pointer-events-none"
            >
                <ServiceState></ServiceState>
            </div>
            <div
                v-if="$route.name != 'home'"
                class="right-btn top-0 h-full items-center flex"
            >
                <button
                    @click="(backFunc ?? $router.back)(back)"
                    class="btn btn-circle btn-sm btn-primary appear"
                >
                    <Icon :path="mdiChevronLeft"></Icon>
                </button>
            </div>
            <div id="title-bar" class="absolute right-0 top-0 flex">
                <slot name="title-bar"></slot>
            </div>
        </div>
        <div id="layer-container" class="h-[calc(100%-3rem)] p-5 pt-0">
            <slot></slot>
        </div>
    </div>
</template>

<script setup lang="ts">
import { mdiChevronLeft } from "@mdi/js";
import Icon from "../components/Icon.vue";
import ToastProvider from "../utils/components/ToastProvider.vue";
import ServiceState from "../components/ServiceState.vue";
import { inject, InjectionKey, provide, ref } from "vue";
import { useRouter } from "vue-router";
import { useRouteNavType } from "../pages/pages";

const router = useRouter();

const backFunc = ref<((back: () => void) => void) | null>(null);

provide(backKey, (onBack: (back: () => void) => void) => {
    backFunc.value = onBack;
});
const back = () => {
    backFunc.value = null;
    setTimeout(() => {
        router.back();
    });
};

const forwardFunc = ref<((forward: () => void) => void) | null>(null);

provide(forwardKey, (onforward: (forward: () => void) => void) => {
    forwardFunc.value = onforward;
});
const forward = () => {
    forwardFunc.value = null;
    setTimeout(() => {
        router.forward();
    });
};

const navType = useRouteNavType();

router.beforeEach(() => {
    const source = navType.value;

    if (source === "back" && backFunc.value) {
        backFunc.value(back);
        return backFunc.value === null;
    }
    if (source === "forward" && forwardFunc.value) {
        forwardFunc.value(forward);
        return forwardFunc.value === null;
    }
    return true;
});
</script>

<script lang="ts">
const backKey = Symbol() as InjectionKey<
    (callback: (back: () => void) => void) => void
>;
const forwardKey = Symbol() as InjectionKey<
    (callback: (forward: () => void) => void) => void
>;
export function onBack(callback: (back: () => void) => void) {
    inject(backKey)?.(callback);
}
export function onForward(callback: (forward: () => void) => void) {
    inject(forwardKey)?.(callback);
}
</script>

<style>
:root {
    --view-height: calc(100vh - 4rem);
}
</style>

<style scoped>
.right-btn {
    @apply absolute left-[1.4rem];
}
.window-decorations {
    margin-bottom: calc(0.25rem * 4);
    display: block;
    aspect-ratio: 1 / 1;
    height: calc(0.25rem * 3);
    flex-shrink: 0;
    align-self: flex-start;
    border-radius: calc(infinity * 1px);
    opacity: 30%;
    box-shadow:
        1.4em 0,
        2.8em 0,
        4.2em 0;
    transition: all 300ms;
}
.mockup-window {
    &:has(.right-btn) {
        .window-decorations {
            transform: translateX(20px);
        }
    }
    &::before {
        display: none;
    }
}
.appear {
    animation: appear 100ms forwards;
}
@keyframes appear {
    from {
        width: calc(0.3rem * 3);
        height: calc(0.3rem * 3);
    }
}
</style>
