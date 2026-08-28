<script setup lang="ts">
import { ref } from "vue";
import Button from "primevue/button";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useAuthStore } from "../stores/auth";

const authStore = useAuthStore();
const opening = ref(false);

function qzoneUrl() {
  const uin = authStore.user?.uin;
  return uin ? `https://user.qzone.qq.com/${uin}` : "https://qzone.qq.com";
}

async function openQzone() {
  opening.value = true;
  try {
    await openUrl(qzoneUrl());
  } finally {
    opening.value = false;
  }
}
</script>

<template>
  <div class="qzone-page qzone-external-page">
    <div class="qzone-external">
      <span class="qzone-external-icon"><i class="pi pi-globe" /></span>
      <h3>在系统浏览器中打开 QQ 空间</h3>
      <p>为避免应用 WebView 持久化 QQ Cookie，本版本不在应用内部承载 QQ 空间登录页。浏览空间时使用你的系统默认浏览器，恢复与归档仍由本机 Rust 后端直接完成。</p>
      <div class="qzone-external-actions">
        <Button label="打开 QQ 空间" icon="pi pi-external-link" :loading="opening" @click="openQzone" />
      </div>
    </div>
  </div>
</template>
