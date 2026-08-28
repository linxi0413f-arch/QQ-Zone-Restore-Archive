<script setup lang="ts">
import { computed, ref } from "vue";
import { storeToRefs } from "pinia";
import { useRoute, useRouter } from "vue-router";
import { platform } from "@tauri-apps/plugin-os";
import { openUrl } from "@tauri-apps/plugin-opener";
import Button from "primevue/button";
import Drawer from "primevue/drawer";
import Popover from "primevue/popover";
import LoginDialog from "../components/LoginDialog.vue";
import { useAppStore } from "../stores/app";
import { useAuthStore } from "../stores/auth";

defineProps<{ pageTitle: string }>();
const appStore = useAppStore();
const authStore = useAuthStore();
const router = useRouter();
const route = useRoute();
const accountPopover = ref<InstanceType<typeof Popover>>();
const moreVisible = ref(false);
const logoutLoading = ref(false);
const { darkMode, sidebarCollapsed, themeIcon } = storeToRefs(appStore);
const { loggedIn, user } = storeToRefs(authStore);
const showQzoneButton = computed(() => {
  const currentPlatform = platform();
  return currentPlatform !== "android" && currentPlatform !== "ios";
});
const navigation = [
  { label: "概览", icon: "pi pi-home", to: "/" },
  { label: "归档", icon: "pi pi-inbox", to: "/archives" },
  { label: "联系人", icon: "pi pi-users", to: "/contacts" },
  { label: "媒体", icon: "pi pi-images", to: "/media" },
  { label: "任务", icon: "pi pi-sync", to: "/tasks" },
  { label: "设置", icon: "pi pi-cog", to: "/settings" },
];
const mobileNavigation = [navigation[0], navigation[1], navigation[3], navigation[4]];
const mobileMoreNavigation = [navigation[2], navigation[5]];
const moreActive = computed(() => mobileMoreNavigation.some((item) => item.to === route.path));
function qzoneUrl() {
  const uin = user.value?.uin;
  return uin ? `https://user.qzone.qq.com/${uin}` : "https://qzone.qq.com";
}
async function openQzoneWindow() {
  await openUrl(qzoneUrl());
}

function handleAccountClick(event: MouseEvent) {
  if (loggedIn.value) accountPopover.value?.toggle(event);
  else authStore.openLogin();
}
async function logout() {
  if (logoutLoading.value) return;
  logoutLoading.value = true;
  try {
    accountPopover.value?.hide();
    await authStore.logout();
    await router.push("/");
  } finally { logoutLoading.value = false; }
}
</script>

<template>
  <div class="app-shell" :class="{ 'app-dark': darkMode, 'sidebar-collapsed': sidebarCollapsed }">
    <aside class="desktop-sidebar">
      <div class="brand">
        <div class="brand-mark"><i class="pi pi-box" /></div>
        <div class="brand-copy"><strong>恢复归档</strong><span>QQ Zone Restore Archive</span></div>
      </div>
      <nav class="side-navigation" aria-label="主要导航">
        <RouterLink v-for="item in navigation" :key="item.to" :to="item.to">
          <i :class="item.icon" /><span>{{ item.label }}</span>
        </RouterLink>
      </nav>
      <div class="sidebar-footer">
        <Button :icon="sidebarCollapsed ? 'pi pi-angle-right' : 'pi pi-angle-left'" severity="secondary" text rounded aria-label="折叠侧边栏" @click="appStore.toggleSidebar" />
      </div>
    </aside>
    <div class="app-workspace">
      <header class="topbar">
        <div><p class="topbar-eyebrow">QQ ZONE RESTORE ARCHIVE</p><h1>{{ pageTitle }}</h1></div>
        <div class="topbar-actions">
          <Button :icon="themeIcon" severity="secondary" text rounded aria-label="切换主题" @click="appStore.toggleTheme" />
          <button class="account-chip" type="button" :aria-label="loggedIn ? '打开账号菜单' : '登录 QQ 空间'" @click="handleAccountClick">
            <span class="account-avatar">
              <img v-if="user?.avatarImage" :src="user.avatarImage" alt="" />
              <i v-else class="pi pi-user" />
            </span>
            <span class="account-copy"><strong>{{ user?.nickname ?? '尚未登录' }}</strong><small>{{ user ? `QQ ${user.uin}` : '登录 QQ 空间' }}</small></span>
            <span v-if="loggedIn" class="account-menu-indicator"><i class="pi pi-angle-down account-menu-arrow" /><i class="pi pi-sign-out account-menu-logout" /></span>
          </button>
          <Popover ref="accountPopover" class="account-popover">
            <div class="account-popover-profile"><span class="account-popover-avatar"><img v-if="user?.avatarImage" :src="user.avatarImage" alt="" /><i v-else class="pi pi-user" /></span><div><strong>{{ user?.nickname }}</strong><span>QQ {{ user?.uin }}</span></div></div>
            <div class="account-popover-divider" />
            <Button v-if="showQzoneButton" label="QQ 空间（系统浏览器）" icon="pi pi-globe" severity="secondary" text @click="openQzoneWindow(); accountPopover?.hide()" />
            <Button label="退出登录" icon="pi pi-sign-out" severity="danger" text :loading="logoutLoading" @click="logout" />
          </Popover>
        </div>
      </header>
      <main class="page-content"><slot /></main>
    </div>
    <nav class="mobile-navigation" aria-label="移动端导航">
      <RouterLink v-for="item in mobileNavigation" :key="item.to" :to="item.to">
        <i :class="item.icon" /><span>{{ item.label }}</span>
      </RouterLink>
      <button class="mobile-navigation-item" :class="{ 'is-active': moreActive }" type="button" aria-label="打开更多页面" @click="moreVisible = true">
        <i class="pi pi-ellipsis-h" /><span>更多</span>
      </button>
    </nav>
    <Drawer v-model:visible="moreVisible" position="bottom" header="更多" class="mobile-more-drawer">
      <nav class="mobile-more-navigation" aria-label="更多页面">
        <RouterLink v-for="item in mobileMoreNavigation" :key="item.to" :to="item.to" @click="moreVisible = false">
          <span class="mobile-more-icon"><i :class="item.icon" /></span>
          <span><strong>{{ item.label }}</strong><small>{{ item.to === '/contacts' ? '查看与你互动过的联系人' : '归档频率、主题与数据管理' }}</small></span>
          <i class="pi pi-angle-right" />
        </RouterLink>
      </nav>
    </Drawer>
    <LoginDialog />
  </div>
</template>
