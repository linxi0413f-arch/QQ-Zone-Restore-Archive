import { createRouter, createWebHashHistory } from "vue-router";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      name: "dashboard",
      component: () => import("../views/DashboardView.vue"),
      meta: { title: "概览" },
    },
    {
      path: "/archives",
      name: "archives",
      component: () => import("../views/ArchivesView.vue"),
      meta: { title: "归档内容" },
    },
    {
      path: "/contacts",
      name: "contacts",
      component: () => import("../views/ContactsView.vue"),
      meta: { title: "联系人" },
    },
    {
      path: "/media",
      name: "media",
      component: () => import("../views/MediaView.vue"),
      meta: { title: "媒体" },
    },
    {
      path: "/tasks",
      name: "tasks",
      component: () => import("../views/TasksView.vue"),
      meta: { title: "归档任务" },
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("../views/SettingsView.vue"),
      meta: { title: "设置" },
    },
    { path: "/:pathMatch(.*)*", redirect: "/" },
  ],
});

export default router;
