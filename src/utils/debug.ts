import { deepClone } from "./object";

export function dbg<T extends any[]>(
  ...args: T
): T["length"] extends 1 ? T[0] : T {
  console.log("dbg: ", deepClone(args));
  return args.length === 1 ? args[0] : args;
}
