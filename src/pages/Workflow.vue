<template>
    <div
        class="transition-all grid grid-cols-[var(--grid-cols)] w-full"
        :style="cardGrid"
    >
        <Transition name="x-slide">
            <div
                v-if="show"
                class="transition-all grid gap-4 h-[var(--view-height)] grid-rows-[var(--grid-rows)]"
            >
                <div
                    @click="activeViewName = 'action'"
                    class="w-full h-full relative"
                    :style="{
                        '--view-height':
                            activeViewName == 'action'
                                ? '600px'
                                : 'calc(100vh - 4rem - 600px - 1rem)',
                    }"
                >
                    <Action
                        :draggableId
                        :show-description-view="activeViewName != 'workflow'"
                        ref="action"
                    ></Action>
                </div>
                <div
                    @click="activeViewName = 'trigger'"
                    class="w-full h-full relative"
                    :style="{
                        '--view-height':
                            activeViewName == 'trigger'
                                ? '600px'
                                : 'calc(100vh - 4rem - 600px - 1rem)',
                    }"
                >
                    <Trigger
                        itemDraggable
                        :draggableId
                        :show-description-view="activeViewName != 'workflow'"
                        ref="trigger"
                    ></Trigger>
                </div>
            </div>
        </Transition>
        <Transition name="-x-slide">
            <div v-if="show" @click="activeViewName = 'workflow'">
                <Dropable
                    @enter="onEnterVueFlow"
                    @move="onMoveOnVueFlow"
                    @drop="onDropOnVueFlow"
                    @leave="onLeaveVueFlow"
                    class="relative"
                    :filter="
                        (data: DraggableCardData) =>
                            !data.card.parent.startsWith('trigger.')
                    "
                    :id="draggableId"
                >
                    <VueFlow
                        v-model:nodes="nodes"
                        v-model:edges="edges"
                        :max-zoom="1"
                        :min-zoom="1"
                        @node-drag-stop="commit()"
                        @node-click="onNodeClick"
                        @pane-click="currentFocusedNode = null"
                    >
                        <Background />
                        <template #edge-transition="props">
                            <WorkflowEdge v-bind="props"></WorkflowEdge>
                        </template>
                        <template #node-trigger="{ id, data }">
                            <Alias
                                :value="data.length !== 0"
                                #="{ value: haveTrigger }"
                            >
                                <Dropable
                                    :id="draggableId"
                                    @drop="onDropOnTrigger"
                                    :filter="
                                        (data: DraggableCardData) =>
                                            data.card.parent.startsWith(
                                                'trigger.',
                                            )
                                    "
                                    class="card border-dashed border-[0.1rem] border-accent min-w-22 min-h-25 items-center p-2 gap-2 transform"
                                    #="{ haveDragging }"
                                >
                                    <template v-if="haveTrigger">
                                        <div
                                            v-for="{ card } in data"
                                            class="badge badge-accent min-w-15"
                                            :class="
                                                haveDragging
                                                    ? 'opacity-10'
                                                    : 'opacity-100'
                                            "
                                        >
                                            {{ card.name }}
                                        </div>
                                    </template>
                                    <Transition name="opacity">
                                        <div
                                            v-show="
                                                !haveDragging || haveTrigger
                                            "
                                            v-tooltip="
                                                !haveTrigger
                                                    ? t('trigger.no')
                                                    : t('trigger.tip')
                                            "
                                            class="absolute left-0 flex"
                                            :class="
                                                !haveTrigger
                                                    ? 'size-full top-0'
                                                    : 'w-full -top-8'
                                            "
                                        >
                                            <Icon
                                                class="m-auto"
                                                :size="!haveTrigger ? 35 : 25"
                                                color="#b1b1b7"
                                                :path="
                                                    !haveTrigger
                                                        ? mdiFlashAlertOutline
                                                        : mdiFlashOutline
                                                "
                                            ></Icon>
                                        </div>
                                    </Transition>
                                    <Transition name="opacity">
                                        <div
                                            v-show="haveDragging"
                                            class="absolute left-0 flex size-full top-0"
                                        >
                                            <Icon
                                                class="m-auto"
                                                size="35"
                                                color="#b1b1b7"
                                                :path="mdiPlus"
                                            ></Icon>
                                        </div>
                                    </Transition>
                                    <Handle
                                        :id
                                        type="source"
                                        :position="Position.Right"
                                    />
                                </Dropable>
                            </Alias>
                        </template>
                        <template #node-dragging-placeholder="{ data }">
                            <div>
                                <Handle
                                    v-for="{ position, branch, id, type } in (
                                        data as DraggableCardData
                                    ).card.branches"
                                    v-tooltip="
                                        dbg(
                                            (data as DraggableCardData).card
                                                .i18n[locale],
                                        )?.[branch]
                                    "
                                    :position
                                    :id="`${data.data?.id ?? 'inline'}:${id}_${branch}`"
                                    :type="
                                        type === 'source' ? 'target' : 'source'
                                    "
                                    class="border-0! bg-transparent! size-4! flex justify-center items-center"
                                    :class="[
                                        { 'top-[30%]': position === 'top' },
                                        {
                                            'bottom-[30%]':
                                                position === 'bottom',
                                        },
                                        { 'left-[30%]': position === 'left' },
                                        { 'right-[30%]': position === 'right' },
                                    ]"
                                >
                                    <div
                                        class="vue-flow__handle size-1.5!"
                                        :class="[
                                            { 'top-[14%]': position === 'top' },
                                            {
                                                'bottom-[14%]':
                                                    position === 'bottom',
                                            },
                                            {
                                                'left-[14%]':
                                                    position === 'left',
                                            },
                                            {
                                                'right-[14%]':
                                                    position === 'right',
                                            },
                                        ]"
                                    ></div>
                                </Handle>
                                <div
                                    class="badge badge-accent min-w-15 relative"
                                >
                                    {{ (data as DraggableCardData).card.name }}
                                </div>
                            </div>
                        </template>
                        <template #node-workflow="{ data, id: wid }">
                            <Handle
                                v-for="{ position, branch, id, type } in (
                                    data as DraggableCardData
                                ).card.branches"
                                v-tooltip="
                                    (data as DraggableCardData).card.i18n[
                                        locale
                                    ][branch]
                                "
                                :position
                                :id="`${data.data?.id ?? 'inline'}:${id}_${branch}`"
                                :type="type === 'source' ? 'target' : 'source'"
                                class="border-0! bg-transparent! size-4! flex justify-center items-center"
                                :class="[
                                    { 'top-[30%]': position === 'top' },
                                    { 'bottom-[30%]': position === 'bottom' },
                                    { 'left-[30%]': position === 'left' },
                                    { 'right-[30%]': position === 'right' },
                                ]"
                            >
                                <div
                                    class="vue-flow__handle size-1.5!"
                                    :class="[
                                        { 'top-[14%]': position === 'top' },
                                        {
                                            'bottom-[14%]':
                                                position === 'bottom',
                                        },
                                        { 'left-[14%]': position === 'left' },
                                        { 'right-[14%]': position === 'right' },
                                    ]"
                                ></div>
                            </Handle>
                            <SelfOwned
                                v-if="data.lit || inlineNodeData[wid]"
                                :default-value="false"
                                #="{ modelValue, update }"
                            >
                                <div
                                    class="badge badge-accent min-w-15 relative"
                                    v-on-click-outside="() => update(false)"
                                >
                                    <Transition
                                        name="opacity"
                                        v-for="{
                                            plug,
                                            branch,
                                            id: cardResultId,
                                        } in (data as DraggableCardData).card
                                            .branches"
                                    >
                                        <template
                                            v-if="
                                                currentFocusedNode?.data.lit ===
                                                    false &&
                                                data.branch ===
                                                    `${
                                                        data.data?.id ??
                                                        'inline'
                                                    }:${cardResultId}_${branch}`
                                            "
                                        >
                                            <Alias value>
                                                <div
                                                    @click.stop="
                                                        update(!modelValue)
                                                    "
                                                    class="cursor-pointer rounded-full bg-primary size-6 absolute -top-8 flex z-10 transition-transform"
                                                    :class="{
                                                        'scale-135': modelValue,
                                                    }"
                                                >
                                                    <Icon
                                                        class="m-auto"
                                                        :path="
                                                            mdiPowerPlugOutline
                                                        "
                                                        size="18"
                                                        color="white"
                                                    ></Icon>
                                                </div>

                                                <Transition name="scale">
                                                    <Socket
                                                        v-if="modelValue"
                                                        class="w-80 absolute bottom-10 origin-bottom"
                                                        :lit-card="{
                                                            label:
                                                                data.data
                                                                    ?.label ??
                                                                'inline',
                                                            plug,
                                                        }"
                                                    ></Socket>
                                                </Transition>
                                            </Alias>
                                        </template>
                                    </Transition>
                                    {{ (data as DraggableCardData).card.name }}
                                </div></SelfOwned
                            >
                            <div v-else>
                                <div
                                    class="badge badge-accent min-w-15 relative"
                                >
                                    <Transition name="opacity">
                                        <template
                                            v-if="
                                                currentFocusedNode?.id ===
                                                    data.id &&
                                                inlineNodeData[data.id]
                                            "
                                        >
                                            <Alias value>
                                                <div
                                                    v-tooltip="
                                                        t('inlineNode.require')
                                                    "
                                                    class="cursor-pointer rounded-full bg-warning size-6 absolute -top-8 flex z-10 transition-transform"
                                                >
                                                    <Icon
                                                        class="m-auto"
                                                        :path="mdiPencilOutline"
                                                        size="16"
                                                        color="white"
                                                    ></Icon>
                                                </div>
                                            </Alias>
                                        </template>
                                    </Transition>
                                    {{ (data as DraggableCardData).card.name }}
                                </div>
                            </div>
                        </template>
                    </VueFlow>
                    <ActionButtonGroup
                        class="absolute bottom-5 left-0 w-full"
                        size="lg"
                        :btns
                    ></ActionButtonGroup>
                </Dropable>
            </div>
        </Transition>
        <Alias
            :value="currentFocusedNode?.data as DraggableCardData"
            #="{ value: data }"
        >
            <SelfOwned :default-value="false" #="{ update, modelValue: foucs }">
                <Layer
                    @click-outside="update(false)"
                    transition="-x-slide"
                    w="90"
                    top="20"
                    right="0"
                    w-h="3/2"
                    v-model="showCardInfo"
                    :class="{ 'translate-x-35': !foucs && hideLayer }"
                    #="{ close }"
                >
                    <div class="w-full max-w-sm mx-auto group">
                        <div
                            class="card bg-base-100 shadow-lg rounded-xl border-gray-400 border-1"
                        >
                            <div
                                class="card-header rounded-t-xl bg-primary text-primary-content p-4 relative"
                            >
                                <button
                                    @click="close"
                                    class="group-hover:translate-x-0 btn btn-circle btn-primary btn-xs btn-ghost absolute right-4 top-4 translate-x-10 transition-all"
                                >
                                    <Icon size="20" :path="mdiClose"></Icon>
                                </button>
                                <template v-if="data.card?.i18n?.[locale]">
                                    <h1
                                        v-if="data.lit"
                                        class="text-xl font-black"
                                    >
                                        {{ data.data.label }}
                                        <Badge
                                            class="mx-2"
                                            @click="selectCardMate(data)"
                                            -style="dash"
                                        >
                                            {{ data.card?.i18n[locale].title }}
                                        </Badge>
                                    </h1>
                                    <h1 v-else class="text-xl font-bold">
                                        {{ data.card?.i18n[locale].title }}
                                    </h1>
                                    <p class="font-medium text-sm">
                                        {{
                                            data.card?.i18n[locale].description
                                        }}
                                    </p>
                                </template>
                                <template v-else>
                                    <h1 class="text-xl font-bold">
                                        {{ data.card?.name }}
                                        <div
                                            @click="selectCardMate(data)"
                                            class="badge badge-warning"
                                        >
                                            <Icon
                                                :path="mdiUnfoldMoreVertical"
                                            ></Icon>
                                            {{ t("developing.title") }}
                                            {{ t("developing.tip") }}
                                        </div>
                                    </h1>
                                </template>
                            </div>
                            <CompactStat
                                v-if="
                                    data.lit ||
                                    inlineNodeData[currentFocusedNode!.id]
                                "
                                v-bind="statBind"
                            ></CompactStat>
                            <SelfOwned
                                :name="'valid'"
                                :default-value="false"
                                #="bind"
                                v-else
                            >
                                <component
                                    ref="form"
                                    v-bind="bind"
                                    class="rounded-xl bg-base-100! border-none!"
                                    :is="currentCardForm"
                                ></component>
                                <button
                                    @click="activateInlineNode"
                                    :disabled="!bind.valid"
                                    class="btn btn-success rounded-t-none rounded-b-xl!"
                                >
                                    <Icon :path="mdiCheck" />
                                </button>
                            </SelfOwned>
                        </div>
                    </div>
                </Layer>
            </SelfOwned>
        </Alias>
    </div>
</template>

<script setup lang="ts">
import {
    Edge,
    GraphNode,
    Handle,
    Node,
    NodeMouseEvent,
    Position,
    useVueFlow,
    VueFlow,
} from "@vue-flow/core";
import {
    computed,
    nextTick,
    provide,
    reactive,
    Ref,
    ref,
    toRaw,
    Transition,
    useTemplateRef,
    watch,
    watchEffect,
} from "vue";
import Icon from "../components/Icon.vue";
import Alias from "../utils/components/Alias.vue";
import { useManualRefHistory } from "@vueuse/core";
import { deepClone } from "../utils/object";
import Layer from "../utils/components/Layer.vue";
import CompactStat from "../components/CompactStat.vue";
import { normalizeStatProps } from "../components/Stat.utils";
import { createI18nWithUtils } from "../i18n/utils";
import { parseData } from "../utils/type";
import { Data, litCardViewProps, TaskMap } from "../invoke/type";
import SelfOwned from "../utils/components/SelfOwned.vue";
import { normalizeRenderable } from "../utils/render";
import createScopeI18n from "../composable/useScopeI18n";
import Badge from "../components/Badge.vue";
import Socket from "../components/Socket.vue";
import { vOnClickOutside } from "@vueuse/components";
import { useI18n } from "vue-i18n";
import { Background } from "@vue-flow/background";
import { vTooltip } from "../directives/tooltip";

const { locale, t } = useI18n();
const { viewport, fitView, updateNode } = useVueFlow();

const DEFAULT_EDGE_TYPE = "smoothstep";

//#region connection
type LinkItem = {
    name: string;
    id: string;
    next?: { [handle: string]: [Edge, LinkItem] };
    previous?: { [handle: string]: [Edge, LinkItem] };
};
const linkMap: LinkItem = { name: "0", id: "trigger" };
const indexMap: { [i: (typeof linkMap)["name"]]: LinkItem } = {
    trigger: linkMap,
};

const flow = ref<string[]>([]);
//#endregion

//#region grid layout
import Action from "./Action.vue";
import Trigger from "./Trigger.vue";
const actionView = useTemplateRef("action");
const triggerView = useTemplateRef("trigger");

const activeViewName = ref<"action" | "trigger" | "workflow">("workflow");
const activeViewRef = computed(() =>
    activeViewName.value == "action"
        ? actionView.value
        : activeViewName.value === "trigger"
          ? triggerView.value
          : null,
);
const cardGrid = computed(() => {
    const width = 800,
        height = 600;

    // 默认值为不在活动状态的卡片高度
    let gridTemplateRows = [
        `calc(var(--view-height) - ${height}px - 1rem)`,
        `calc(var(--view-height) - ${height}px - 1rem)`,
    ];

    let gridTemplateCols = ["15rem", "auto"];
    switch (activeViewName.value) {
        case "action":
            gridTemplateRows[0] = `${height}px`;
            gridTemplateCols[0] = `${width}px`;
            triggerView.value?.expose?.showEmptyView();
            break;
        case "trigger":
            gridTemplateRows[1] = `${height}px`;
            gridTemplateCols[0] = `${width}px`;
            actionView.value?.expose?.showEmptyView();
            break;
        case "workflow":
            // 默认情况下激活工作流，需要指定两个view的高度
            gridTemplateRows = cardGrid.value
                ? cardGrid.value["--grid-rows"].split(" ")
                : [`${height}px`, gridTemplateRows[1]];
            gridTemplateCols[1] = `calc(100vw - 17.5rem)`;
            break;
    }
    return {
        // 转化为css变量
        "--grid-rows": gridTemplateRows.join(" "),
        "--grid-cols": gridTemplateCols.join(" "),
    };
});
provide("viewport", viewport);
//#endregion

//#region draggable
import Dropable, {
    DropEvent,
    EnterEvent,
    MoveEvent,
} from "../utils/components/Dropable.vue";

import { throttleWithCache } from "../utils/timing";

import WorkflowEdge from "./Workflow.Edge.vue";
import { inDevMode } from "../utils/mode";
import { DraggableCardData } from "../views/CardView.vue";

const draggableId = Symbol("workflow");

// 定义节点
const triggerNodes = reactive<DraggableCardData[]>([]);
const nodes = ref<Node[]>([
    {
        type: "trigger",
        id: "trigger",
        position: { x: 250, y: 5 },
        data: triggerNodes,
    },
]);

function onDropOnTrigger(e: DropEvent<DraggableCardData>) {
    const id = nodes.value.length.toString();
    currentNodeId.value = id;
    triggerNodes.push(e.data);
}

const edges = ref<Edge[]>([]);

const currentEdgeId = ref("");
const currentNodeId = ref("");

function onEnterVueFlow(e: EnterEvent<DraggableCardData>) {
    const id = nodes.value.length.toString();
    currentNodeId.value = id;
    nodes.value.push({
        id,
        type: "dragging-placeholder",
        position: {
            x: e.position.x / viewport.value.zoom,
            y: e.position.y / viewport.value.zoom,
        },
        data: e.data,
    });
    currentEdgeId.value = "edge" + id;
    const edge = getNearestEdge([...(nodes.value as GraphNode[])]);
    edges.value.push({
        id: currentEdgeId.value,
        type: DEFAULT_EDGE_TYPE,
        style: { stroke: "var(--color-accent)", opacity: 0.5 },
        ...edge,
    });
}

let getEdge: typeof getNearestEdge | null;

function onMoveOnVueFlow(e: MoveEvent) {
    const id = (nodes.value.length - 1).toString();
    updateNode(id, {
        position: {
            x: (e.position.x - viewport.value.x) / viewport.value.zoom,
            y: (e.position.y - viewport.value.y) / viewport.value.zoom,
        },
    });
    if (!getEdge) {
        getEdge = throttleWithCache(getNearestEdge, 100);
    }
    const edge = getEdge([...nodes.value] as GraphNode[]);

    edges.value = edges.value.map((e) => {
        if (e.id === currentEdgeId.value) {
            return { ...e, ...edge, animated: true };
        }
        return e;
    });
}

function onLeaveVueFlow() {
    getEdge = null;
    nodes.value.pop();
    edges.value.pop();
}

function onDropOnVueFlow({ data }: DropEvent<DraggableCardData>) {
    getEdge = null;
    nodes.value = nodes.value
        .map((node) => {
            if (node.id === currentNodeId.value) {
                node.type = "workflow";
            }
            return node;
        })
        .filter(({ type }) => type !== "dragging-placeholder");
    edges.value = edges.value.map((edge) => {
        if (edge.id === currentEdgeId.value) {
            const item: LinkItem = {
                name: currentNodeId.value,
                id: data.lit ? data.data.id : data.card.name,
            };

            (indexMap[edge.source].next ??= {})[edge.sourceHandle!] = [
                edge,
                item,
            ];
            (item.previous ??= {})[edge.sourceHandle!] = [
                edge,
                indexMap[edge.source],
            ];
            indexMap[item.name] = item;
            edge.animated = false;
            delete edge.style;
        }
        return edge;
    });
    console.log(linkMap);
    commit();
}
//#endregion

//#region create task
// 构建服务需要的任务表

function finish() {
    let currentNode = linkMap;
    if (!currentNode) return;

    // while (currentNode) {
    //   currentNode.name;

    //   Object.entries(currentNode).forEach(() => {});
    // }
    const map = createTaskMap(currentNode);
    console.log(map);
}

function createTaskMap(node: LinkItem, map: TaskMap = {}) {
    const preWid = node.name; // w-id 0
    if (!node.next) return;
    Object.entries(node.next).forEach(([branch, [, node]]) => {
        const wid = node.name;
        if (node.id)
            map[`${preWid}:${branch}`] = node.id.match(/\d+$/)
                ? { LitRef: { wid, id: node.id } }
                : {
                      Inline: {
                          uid: `${wid}:inline`,
                          data: toRaw(inlineNodeData.value[wid]),
                          type: node.id,
                      },
                  };
        createTaskMap(node, map);
    });
    return map;
}

//#endregion

//#region toolbar
import { getNearestEdge, useLayout, useTransition } from "./Workflow.utils";
import ActionButtonGroup, {
    ActionButton,
} from "../components/ActionButtonGroup.vue";
import {
    mdiChartSankeyVariant,
    mdiCheck,
    mdiClose,
    mdiFlashAlertOutline,
    mdiFlashOutline,
    mdiPencilOutline,
    mdiPlus,
    mdiPowerPlugOutline,
    mdiRedoVariant,
    mdiReload,
    mdiUndoVariant,
    mdiUnfoldMoreVertical,
} from "@mdi/js";
import { dbg } from "../utils/debug";

const { layout } = useLayout();
const { startTransition } = useTransition();
const {
    undo: n_undo,
    canUndo,
    redo: n_redo,
    canRedo,
    commit: n_commit,
    clear,
} = useManualRefHistory(nodes, {
    clone: deepClone,
});
const {
    undo: e_undo,
    redo: e_redo,
    commit: e_commit,
} = useManualRefHistory(edges, {
    clone: deepClone,
});

const commit = () => (n_commit(), e_commit());
const undo = () => (n_undo(), e_undo());
const redo = () => (n_redo(), e_redo());

// toolbar定义
const btns: ActionButton[] = [
    {
        type: "accent",
        tooltip: "布局",
        callback: reLayout,
        icon: mdiChartSankeyVariant,
        shortcutKey: ["Shift", "Alt", "f"],
    },
    {
        // 仅用于测试
        type: "secondary",
        tooltip: "流程",
        disable: () => !flow.value.length,
        callback() {
            startTransition(flow.value);
        },
        icon: mdiChartSankeyVariant,
        show: inDevMode(),
    },
    {
        type: "primary",
        style: "outline",
        tooltip: "撤销",
        callback: undo,
        disable: () => !canUndo.value,
        icon: mdiUndoVariant,
        shortcutKey: ["Control", "z"],
    },
    {
        type: "primary",
        style: "outline",
        tooltip: "恢复",
        callback: redo,
        disable: () => !canRedo.value,
        icon: mdiRedoVariant,
        shortcutKey: ["Control", "y"],
    },
    {
        type: "error",
        style: "soft",
        tooltip: "重置",
        callback: reset,
        icon: mdiReload,
        shortcutKey: ["Shift", "Alt", "r"],
    },
    {
        type: "accent",
        style: "solid",
        tooltip: "完成",
        disable: () => !edges.value.length,
        callback: finish,
        icon: mdiCheck,
        shortcutKey: ["Control", "E"],
    },
];

function reLayout() {
    // 左右排布
    // TODO: 偏好设置，排布方向
    // ? 由于handle是卡片实现时定义的，可能需要旋转handle的位置
    nodes.value = layout(nodes.value, edges.value, "LR");
    nextTick(() => {
        fitView();
    });
    commit();
}

function reset() {
    while (canUndo.value) undo();
    clear();
}

//#endregion

//#region float card
const currentFocusedNode = ref<GraphNode<DraggableCardData> | null>();

const showCardInfo = computed({
    get: () => !!currentFocusedNode.value,
    set: (val) => {
        if (!val) currentFocusedNode.value = null;
    },
});

const statBind = ref();

function onNodeClick({ node }: NodeMouseEvent) {
    let current = indexMap[node.id];
    const line = [current];
    const connectEdges: Edge[] = [];
    while (current.previous) {
        const [[_, [edge, value]]] = Object.entries(current.previous);
        current = value;
        connectEdges.push(edge);
        line.push(current);
    }

    flow.value = line.reverse().map(({ name: id }) => id);
    let branch = "";
    nodes.value = nodes.value.reduceRight((list, node) => {
        if (node.id === "trigger") {
            list.unshift(node);
            return list;
        }
        const currentNode = line.find((l) => l.name === node.id);
        if (currentNode) {
            const b = branch;
            branch = Object.keys(currentNode.previous!)[0];
            list.unshift({ ...node, data: { ...node.data, branch: b } });
        } else {
            list.unshift({ ...node, data: { ...node.data, branch: false } });
        }
        return list;
    }, [] as Node[]);

    edges.value = edges.value.map((edge) => {
        if (connectEdges.findIndex((e) => e.id === edge.id) !== -1) {
            return { ...edge, animated: true, type: "transition" };
        }
        return { ...edge, animated: false, type: DEFAULT_EDGE_TYPE };
    });

    currentFocusedNode.value = node;
}

watchEffect(() => {
    if (!currentFocusedNode.value) return;
    const cardData: DraggableCardData = currentFocusedNode.value.data;
    const inlineData = inlineNodeData.value[currentFocusedNode.value.id];
    // debugger;
    if (cardData.lit || inlineData) {
        const { card, data: litCard } = {
            data: { data: inlineData },
            ...cardData,
        };
        const i18n = createI18nWithUtils({
            messages: card.i18n,
            locale: locale.value,
        });
        const statProps = card.litCardView({
            ...i18n.global,
            litCardInfo: {
                ...litCard,
                type: card.name,
                data: parseData(litCard.data),
            },
            cardInfo: card,
        } as litCardViewProps);
        statBind.value = {
            stat: statProps.map((stat) => {
                return normalizeStatProps(
                    stat,
                    parseData(litCard.data),
                    i18n.global,
                    card.args,
                );
            }),
            onVnodeUnmounted() {
                i18n.dispose();
            },
        };
    } else {
        const { card } = cardData;
        const useI18n = createScopeI18n({
            locale: locale.value,
            messages: card.i18n,
        });
        currentCardForm.value = normalizeRenderable(card.view, {
            useI18n,
            cardInfo: card,
        });
    }
});

watch(currentFocusedNode, (val) => {
    if (!val) {
        edges.value = edges.value.map((edge) => ({
            ...edge,
            type: DEFAULT_EDGE_TYPE,
            animated: false,
        }));
        flow.value = [];
    }
});

const currentCardForm = ref();

function selectCardMate(data: DraggableCardData) {
    if (data.card.parent.startsWith("action.")) {
        activeViewName.value = "action";
        actionView.value?.expose?.selectLitCardById(data.card.name);
    } else {
        activeViewName.value = "trigger";
        triggerView.value?.expose?.selectLitCardById(data.card.name);
    }
}

function activateInlineNode() {
    const id = currentFocusedNode.value!.id;
    inlineNodeData.value[id] = form.value!.getData();
}
const form = useTemplateRef<{ getData: () => Data }>("form");

const inlineNodeData: Ref<{ [wid: string]: Data }> = ref({});

/* function selectLitCard(data: DraggableCardData) {
  if (!data.lit) return;
  if (data.card.parent.startsWith("action.")) {
    activeViewName.value = "action";
    actionView.value?.expose?.selectLitCardById(data.data.id);
  } else {
    activeViewName.value = "trigger";
    triggerView.value?.expose?.selectLitCardById(data.data.id);
  }
}
 */
const hideLayer = computed(() => {
    const data = currentFocusedNode.value?.data;
    if (!data) return true;
    if (!data.card.parent.startsWith(activeViewName.value)) return false;
    const currentValue = activeViewRef.value?.expose?.currentValue;
    if (!currentValue) return;
    if (currentValue.lit !== data.lit) return false;
    if (currentValue.name !== (data.lit ? data.data.id : data.card.name))
        return false;
    return true;
});

//#endregion

//#region show
const show = defineModel("show", { default: false });

watchEffect(() => {
    if (!show.value) {
        currentFocusedNode.value = null;
    }
});

//#endregion

//#region window
// import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";

// onBeforeMount(async () => {
//   await getCurrentWindow().setSize(new LogicalSize(1200, 900));
//   await getCurrentWindow().center();
// });

// onBeforeUnmount(async () => {
//   await getCurrentWindow().setSize(new LogicalSize(800, 600));
//   await getCurrentWindow().center();
// });

//#endregion
</script>
<style>
/* import the necessary styles for Vue Flow to work */
@import "@vue-flow/core/dist/style.css";

/* import the default theme, this is optional but generally recommended */
@import "@vue-flow/core/dist/theme-default.css";
</style>

<i18n lang="yaml">
zh-CN:
    trigger:
        tip: 触发器，支持传入多个触发器
        no: 没有触发器
    inlineNode:
        require: 内联节点需要激活

en:
    trigger:
        tip: Trigger, supports multiple triggers passed in,
        no: No Trigger
    inlineNode:
        require: Inline node need to be activated
</i18n>
