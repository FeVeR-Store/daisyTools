import { Config } from "../pages/Settings.vue";
import { createInvoke } from "./helper";
import { CardMeta, Data, LitCard } from "./type";
import { ServiceState } from "./serviceState";

export const invokeMap = {
  registerAction: {
    args: ["actionType", "name", "args"] as {} as [
      actionType: CardMeta["name"],
      name: string,
      args: CardMeta["args"]
    ],
    return: "" as string,
  },
  registerTrigger: {
    args: ["triggerType", "name", "args"] as {} as [
      triggerType: CardMeta["name"],
      name: string,
      args: CardMeta["args"]
    ],
    return: "" as string,
  },
  getLitTrigger: {
    args: [] as unknown[],
    return: {} as LitCard[],
  },
  getLitAction: {
    args: [] as unknown[],
    return: {} as LitCard[],
  },
  updateActionPlug: {
    args: ["id", "plug"] as {} as [id: string, plug: Record<string, any>],
    return: undefined as void,
  },
  getScriptById: {
    args: ["actionId"] as {} as [actionId: string],
    return: "" as string,
  },
  getServiceState: {
    args: [] as unknown[],
    return: "" as ServiceState,
  },
  launchService: {
    args: [] as unknown[],
    return: undefined as void,
  },
  getServiceStateFile: {
    args: [] as unknown[],
    return: "" as string,
  },
  createTask: {
    args: ["triggerId", "name", "workflow"] as {} as [
      trigger_id: string,
      name: string,
      workflow: string[]
    ],
    return: "" as string,
  },
  runActionById: {
    args: ["id"] as {} as [id: string],
    return: undefined as void,
  },
  runAction: {
    args: ["actionType", "args"] as {} as [actionType: string, args: Data],
    return: undefined as void,
  },
  removeAction: {
    args: ["id"] as {} as [id: string],
    return: undefined as void,
  },
  removeTrigger: {
    args: ["id"] as {} as [id: string],
    return: undefined as void,
  },
  removeTask: {
    args: ["id"] as {} as [id: string],
    return: undefined as void,
  },
  isCronExpressionVaild: {
    args: ["expression"] as {} as [expression: string],
    return: {} as boolean,
  },
  get_config: {
    args: [] as unknown[],
    return: {} as Config,
  },
  openWindow: {
    args: [] as unknown[],
    return: {} as unknown,
  },
} as const;

export const api = createInvoke(invokeMap);
