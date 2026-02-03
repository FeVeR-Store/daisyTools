import { api } from "../invoke";
import "ses";
import { ModuleSource } from "@endo/module-source";
import { createActionInvokeMap } from "../invoke/helper";
import { actions } from "../invoke/actions";
const url = new URL(document.URL);

// 启用ses
lockdown();

const ModuleMap: { [key: string]: object | string } = {
  daisy: {
    abc: 123,
  },
  ...createActionInvokeMap(actions),
};

const compartment = new Compartment({
  globals: {
    console,
  },
  resolveHook: (specifier: string) => {
    return specifier;
  },
  importHook: (specifier: string) => {
    if (typeof ModuleMap[specifier] == "object") {
      return {
        namespace: ModuleMap[specifier],
      };
    } else {
      return new ModuleSource(ModuleMap[specifier]);
    }
  },
  __options__: true,
});

let action_id = url.searchParams.get("action_id");

(window as any).run = (code: string) => {
  let id = "script" + Date.now();
  ModuleMap[id] = code;
  compartment.import(id);
};

if (action_id) {
  const content = api.getScriptById(action_id);
  const script = await content;
  if (script) {
    (window as any).run(script);
  }
}
