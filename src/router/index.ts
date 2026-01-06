import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Home",
    redirect: "/peers",
  },
  {
    path: "/peers",
    name: "Peers",
    component: () => import("../views/PeersView.vue"),
  },
  {
    path: "/settings",
    name: "Settings",
    component: () => import("../views/SettingsView.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
