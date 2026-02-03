import { useMagicKeys, whenever } from "@vueuse/core";
import { onUnmounted, WatchHandle } from "vue";

type Keys =
  | Letter
  | Uppercase<Letter>
  | Number
  | Special
  | FunctionKeys
  | NavigationKeys
  | ModifierKeys
  | "Enter";

//#region Type: key

type Special = "!" | "@" | "#" | "$" | "%" | "^" | "&" | "*" | "(" | ")";

type FunctionKeys =
  | "F1"
  | "F2"
  | "F3"
  | "F4"
  | "F5"
  | "F6"
  | "F7"
  | "F8"
  | "F9"
  | "F10"
  | "F11"
  | "F12";

type NavigationKeys =
  | "ArrowUp"
  | "ArrowDown"
  | "ArrowLeft"
  | "ArrowRight"
  | "Home"
  | "End"
  | "PageUp"
  | "PageDown";

type ModifierKeys = "Shift" | "Control" | "Alt" | "Meta";

type Number = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

type Letter =
  | "a"
  | "b"
  | "c"
  | "d"
  | "e"
  | "f"
  | "g"
  | "h"
  | "i"
  | "j"
  | "k"
  | "l"
  | "m"
  | "n"
  | "o"
  | "p"
  | "q"
  | "r"
  | "s"
  | "t"
  | "u"
  | "v"
  | "w"
  | "x"
  | "y"
  | "z";
//#endregion

//#region helper
export type ShortcutKeyType = Keys[];
//#endregion

export function useShortcutKeys() {
  const keys = useMagicKeys();
  const watcher: WatchHandle[] = [];
  const stop = () => watcher.forEach((unwatch) => unwatch());
  onUnmounted(stop);
  return {
    on(keyList: ShortcutKeyType, listener: () => void) {
      watcher.push(whenever(keys[keyList.join("+")], listener));
    },
    stop,
    current: keys.current
  };
}
