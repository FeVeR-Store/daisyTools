import mitt from "mitt";
import { ToastOption } from "../utils/components/ToastProvider.vue";

export const bus = mitt<{
  "toast:show": [string, ToastOption];
  "component:available": [string];
  "component:call": [string, string, any[]];
}>();
