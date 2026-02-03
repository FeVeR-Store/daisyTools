import {
  Edge,
  GraphEdge,
  GraphNode,
  Node,
  useVueFlow,
} from "@vue-flow/core";
import { ref, type Ref, provide, InjectionKey } from "vue";

export function getNearestEdge(nodes: GraphNode[]) {
  const currentNode = nodes.pop();
  const getDestence = (x: number, y: number) =>
    (currentNode!.position.x - x) ** 2 + (currentNode!.position.y - y) ** 2;
  let dest = Infinity;
  let edge: Pick<GraphEdge<any, any>, "source" | "target"> &
    Partial<GraphEdge> = {
    target: currentNode!.id,
    source: "",
    targetHandle: currentNode?.handleBounds?.target?.[0]?.id,
    sourceHandle: "",
  };
  nodes.forEach((node) => {
    const { x: node_x, y: node_y } = node.position;
    node.handleBounds.source?.forEach(({ x, y, id }) => {
      const cur = getDestence(node_x + x, node_y + y);
      if (cur < dest) {
        dest = cur;
        edge.sourceHandle = id ?? node.id;
        edge.source = node.id;
      }
    });
  });
  return edge;
}
import * as dagre from "@dagrejs/dagre";
export function useLayout() {
  const { findNode } = useVueFlow();

  const graph = ref(new dagre.graphlib.Graph());

  const previousDirection = ref("LR");

  function layout(nodes: Node[], edges: Edge[], direction: "LR" | "TB") {
    // we create a new graph instance, in case some nodes/edges were removed, otherwise dagre would act as if they were still there
    const dagreGraph = new dagre.graphlib.Graph();

    graph.value = dagreGraph;

    dagreGraph.setDefaultEdgeLabel(() => ({}));

    dagreGraph.setGraph({ rankdir: direction });

    previousDirection.value = direction;

    for (const node of nodes) {
      // if you need width+height of nodes for your layout, you can use the dimensions property of the internal node (`GraphNode` type)
      const graphNode = findNode(node.id)!;

      dagreGraph.setNode(node.id, {
        width: graphNode.dimensions.width*1.2 || 150,
        height: graphNode.dimensions.height*1.2 || 50,
      });
    }

    for (const edge of edges) {
      dagreGraph.setEdge(edge.source, edge.target);
    }

    dagre.layout(dagreGraph);

    // set nodes with updated positions
    return nodes.map((node) => {
      const nodeWithPosition = dagreGraph.node(node.id);

      return {
        ...node,
        // targetPosition: isHorizontal ? Position.Left : Position.Top,
        // sourcePosition: isHorizontal ? Position.Right : Position.Bottom,
        position: { x: nodeWithPosition.x, y: nodeWithPosition.y },
      };
    });
  }

  return { graph, layout, previousDirection };
}

export const transitionKey = Symbol("transition") as InjectionKey<{
  currentNode: Ref<Node | null>;
  targetNode: Ref<Node | null>;
  finishTransition: () => void;
  // onFinished: (nextNodeId: string) => void;
}>;

export function useTransition() {
  const { findNode } = useVueFlow();
  const nodes = ref<Node[]>([]);
  const currentNode = ref<Node | null>(null);
  const targetNode = ref<Node | null>(null);
  provide(transitionKey, {
    currentNode,
    targetNode,
    finishTransition() {
      [currentNode.value, targetNode.value] = [
        targetNode.value!,
        nodes.value.shift()!,
      ];
    },
    // onFinished(nextNodeId: string){
    //   const nextNode = findNode(nextNodeId)!;

    // }
  });

  return {
    startTransition(idList: string[]) {
      nodes.value = idList.map((id) => findNode(id)!);
      [currentNode.value, targetNode.value] = [
        nodes.value.shift()!,
        nodes.value.shift()!,
      ];
    },
  };
}
