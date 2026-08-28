<script setup lang="ts">
import { storeToRefs } from "pinia";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import ProgressSpinner from "primevue/progressspinner";
import { useAuthStore } from "../stores/auth";

const authStore = useAuthStore();
const { dialogVisible, loading, qrImage, status, message } = storeToRefs(authStore);
</script>

<template>
  <Dialog
    :visible="dialogVisible"
    modal
    :draggable="false"
    :closable="true"
    class="login-dialog"
    header="扫码登录 QQ 空间"
    @update:visible="(visible) => !visible && authStore.closeLogin()"
  >
    <div class="login-content">
      <div class="qr-frame" :class="{ 'qr-muted': status === 'expired' || status === 'error' }">
        <ProgressSpinner v-if="loading && !qrImage" stroke-width="4" />
        <img v-else-if="qrImage" :src="qrImage" alt="QQ 登录二维码" />
        <i v-else class="pi pi-qrcode" />
        <div v-if="status === 'scanned'" class="qr-confirmed"><i class="pi pi-check" /></div>
      </div>

      <div class="login-status" :class="`status-${status}`">
        <span class="status-dot" />
        <p>{{ message }}</p>
      </div>
      <p class="login-help">请使用手机 QQ 扫描。二维码和 Cookie 仅由本机 Rust 后端处理，凭证不会返回前端，也不会保存到磁盘。</p>
      <Button v-if="status === 'expired' || status === 'error'" label="刷新二维码" icon="pi pi-refresh" :loading="loading" @click="authStore.refreshQrCode" />
    </div>
  </Dialog>
</template>
