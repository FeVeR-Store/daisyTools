import { Plugin } from "vite";

export default function () {
  const env: [string, string][] = [];
  process.argv.forEach((arg) => {
    const match = arg.match(/--(.*)=(.*)/);
    if (match) {
      const [, name, value] = match;
      env.push([name, value]);
    }
  });
  const virtualModuleId = "virtual:env";
  const resolvedVirtualModuleId = "\0" + virtualModuleId;

  return {
    name: "command-env-inject",
    config(_, { mode }) {
      env.push(["mode", mode]);
    },
    resolveId(id) {
      if (id === virtualModuleId) {
        return resolvedVirtualModuleId;
      }
    },
    load(id) {
      if (id === resolvedVirtualModuleId) {
        return `
        if (typeof process === 'undefined') {
          globalThis.process = { env: {} };
        }
        if (!('env' in process)) {
          process.env = {};
        }
        const entires = Object.entries(process.env);
        process.env = Object.fromEntries([...entires, ...${JSON.stringify(
          env
        )}]);`;
      }
    },
  } as Plugin;
}
