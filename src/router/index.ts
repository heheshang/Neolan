import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Home",
    redirect: "/chat",
  },
  {
    path: "/peers",
    name: "Peers",
    component: () => import("../views/PeersView.vue"),
  },
  {
    path: "/chat",
    name: "Chat",
    component: () => import("../views/ChatView.vue"),
  },
  {
    path: "/settings",
    name: "Settings",
    component: () => import("../views/SettingsView.vue"),
  },
  {
    path: "/transfers",
    name: "FileTransfers",
    component: () => import("../views/FileTransfersView.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
