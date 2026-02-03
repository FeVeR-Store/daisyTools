import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";
import Home from "./Home.vue";
import Trigger from "./Trigger.vue";
import Action from "./Action.vue";
import Settings from "./Settings.vue";
import Workflow from "./Workflow.vue";
import Example from "./Example.vue";
import Task from "./Task.vue";
import { ref } from "vue";

const routes: RouteRecordRaw[] = [
  { name: "home", path: "/", component: Home },
  { name: "settings", path: "/settings", component: Settings },
  { name: "triggers", path: "/trigger/:name?", component: Trigger },
  { name: "actions", path: "/action/:name?", component: Action },
  { name: "task", path: "/task", component: Task },
  { name: "workflow", path: "/workflow", component: Workflow },
  { path: "/example", component: Example },
];

export const router = createRouter({
  routes,
  history: createWebHashHistory(),
});

const _push = router.push;
router.push = (to) => {
  navType.value = "link";
  return _push(to);
};
const _replace = router.replace;
router.replace = (to) => {
  navType.value = "link";
  return _replace(to);
};

const navType = ref<"link" | "back" | "forward">();
export function useRouteNavType() {
  return navType;
}

router.options.history.listen((_, __, info) => {
  navType.value = info.direction as "back" | "forward";
});

// router.afterEach(() => {
//   nav_Link = false;
//   // const currentPosition = window.history.state?.position;
//   // if (prevPosition != null && currentPosition != null) {
//   //   if (currentPosition < prevPosition) {
//   //     navType.value = "back";
//   //   } else if (currentPosition > prevPosition) {
//   //     navType.value = nav_Link ? "link" : "forward";
//   //   }
//   // }
//   // nav_Link = false;
//   // prevPosition = currentPosition;
// });
