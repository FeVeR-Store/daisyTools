/* ! Effect! */

// reflect-metadata
import "reflect-metadata/lite";

// process.env
import "virtual:env";

// fetch
import { fetch } from "@tauri-apps/plugin-http";
declare global {
  /**
   * 原fetch
   */
  function _fetch(
    input: string | URL | globalThis.Request,
    init?: RequestInit
  ): Promise<Response>;
}

const _fetch = globalThis.fetch;

globalThis._fetch = _fetch;

globalThis.fetch = (...args) => {
  // 正常转发ipc
  if (
    typeof args[0] === "string" &&
    args[0].startsWith("http://ipc.localhost/")
  ) {
    return _fetch(...args);
  }
  // 其他网络请求使用@tauri-apps/plugin-http
  return fetch(...args);
};
