<script setup lang="ts">
import { storeToRefs } from "pinia";
import { onMounted, ref, watch } from "vue";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputNumber from "primevue/inputnumber";
import { getVersion } from "@tauri-apps/api/app";
import { openPath, openUrl } from "@tauri-apps/plugin-opener";
import { useAuthStore } from "../stores/auth";
import { DEFAULT_ARCHIVE_INTERVAL, MIN_ARCHIVE_INTERVAL, getArchiveInterval, resetAppSettings, setArchiveInterval } from "../utils/appSettings";
import { deleteAllAppData, deleteCurrentAccountData, getPrivacyStatus, type PrivacyStatus } from "../utils/qzone";

const authStore = useAuthStore();
const { loggedIn, user } = storeToRefs(authStore);
const intervalMs = ref(getArchiveInterval());
const privacyVisible = ref(false);
const deleteVisible = ref(false);
const deleteCurrentVisible = ref(false);
const deleting = ref(false);
const deletingCurrent = ref(false);
const error = ref("");
const appVersion = ref("");
const privacyStatus = ref<PrivacyStatus>();

async function refreshPrivacyStatus() {
  try {
    privacyStatus.value = await getPrivacyStatus();
  } catch (reason) {
    console.warn("读取隐私状态失败", reason);
  }
}

onMounted(async () => {
  try {
    appVersion.value = await getVersion();
  } catch (reason) {
    console.warn("读取应用版本失败", reason);
  }
  await refreshPrivacyStatus();
});

watch(intervalMs, (value) => { intervalMs.value = setArchiveInterval(value); });
watch(loggedIn, () => { refreshPrivacyStatus(); });

async function deleteCurrent() {
  deletingCurrent.value = true; error.value = "";
  try {
    await deleteCurrentAccountData();
    await authStore.logout();
    await refreshPrivacyStatus();
    deleteCurrentVisible.value = false;
  } catch (reason) { error.value = String(reason); }
  finally { deletingCurrent.value = false; }
}

async function deleteEverything() {
  deleting.value = true; error.value = "";
  try {
    await deleteAllAppData();
    resetAppSettings(); intervalMs.value = DEFAULT_ARCHIVE_INTERVAL;
    await authStore.logout();
    await refreshPrivacyStatus();
    deleteVisible.value = false;
  } catch (reason) { error.value = String(reason); }
  finally { deleting.value = false; }
}
</script>

<template>
  <section class="settings-stack">
    <article class="surface-card settings-card">
      <div class="settings-copy"><span class="settings-icon tone-blue"><i class="pi pi-user" /></span><div><h3>QQ 空间账号</h3><p>{{ loggedIn ? `${user?.nickname}（QQ ${user?.uin}）` : "尚未登录 QQ 空间" }}</p></div></div>
      <Button v-if="loggedIn" label="退出登录" icon="pi pi-sign-out" severity="danger" outlined @click="authStore.logout" />
      <Button v-else label="登录" icon="pi pi-link" @click="authStore.openLogin" />
    </article>

    <article class="surface-card settings-card interval-setting">
      <div class="settings-copy"><span class="settings-icon tone-green"><i class="pi pi-clock" /></span><div><h3>单页获取间隔</h3><p>每读取一页后等待一段时间再请求下一页，间隔越久越稳定。</p></div></div>
      <div class="interval-control"><InputNumber v-model="intervalMs" :min="MIN_ARCHIVE_INTERVAL" :max="30000" :step="500" suffix=" ms" show-buttons button-layout="horizontal" decrement-button-icon="pi pi-minus" increment-button-icon="pi pi-plus" /><small>最低 2000ms，建议 3000–5000ms</small></div>
    </article>

    <article class="surface-card settings-card">
      <div class="settings-copy"><span class="settings-icon tone-purple"><i class="pi pi-shield" /></span><div><h3>本机隐私状态</h3><p v-if="privacyStatus">纯本地处理 · 无遥测 · 无云端存储 · QQ 凭证不写入磁盘</p><p v-else>正在读取隐私状态…</p><small v-if="privacyStatus">数据目录：{{ privacyStatus.appDataDir }}</small></div></div>
      <div style="display:flex;gap:8px;flex-wrap:wrap;justify-content:flex-end"><Button label="打开数据目录" icon="pi pi-folder-open" severity="secondary" outlined :disabled="!privacyStatus" @click="privacyStatus && openPath(privacyStatus.appDataDir)" /><Button label="查看说明" icon="pi pi-angle-right" icon-pos="right" severity="secondary" text @click="privacyVisible = true" /></div>
    </article>

    <article v-if="loggedIn" class="surface-card settings-card danger-settings-card">
      <div class="settings-copy"><span class="settings-icon tone-red"><i class="pi pi-user-minus" /></span><div><h3>删除当前 QQ 本地数据</h3><p>只删除当前 QQ 的动态、互动、续传和任务记录并退出登录；为避免媒体残留，共享图片/视频缓存也会清空。</p></div></div>
      <Button label="删除当前账号数据" icon="pi pi-user-minus" severity="danger" outlined @click="deleteCurrentVisible = true" />
    </article>

    <article class="surface-card settings-card danger-settings-card">
      <div class="settings-copy"><span class="settings-icon tone-red"><i class="pi pi-trash" /></span><div><h3>删除所有数据</h3><p>删除全部账号的归档、续传记录和媒体缓存，并清除内存中的 QQ 登录状态。</p></div></div>
      <Button label="删除所有数据" icon="pi pi-trash" severity="danger" outlined @click="deleteVisible = true" />
    </article>

    <p v-if="error" class="archive-error"><i class="pi pi-exclamation-circle" />{{ error }}</p>
    <article class="surface-card settings-card about-card">
      <div class="about-main">
        <div class="settings-copy"><span class="settings-icon"><i class="pi pi-info-circle" /></span><div><h3>关于</h3><p>QQ Zone Restore Archive · 跨平台空间恢复归档工具</p><p class="author-line">作者：<button class="author-link" type="button" @click="openUrl('https://github.com/xiaosu19')">https://github.com/xiaosu19 <i class="pi pi-external-link" /></button></p><p class="author-line">项目：<button class="author-link" type="button" @click="openUrl('https://github.com/linxi0413f-arch/QQ-Zone-Restore-Archive')">linxi0413f-arch/QQ-Zone-Restore-Archive <i class="pi pi-external-link" /></button></p><p class="author-line">上游：<button class="author-link" type="button" @click="openUrl('https://github.com/xiaosu19/QQ-Zone-Restore-Archive')">xiaosu19/QQ-Zone-Restore-Archive <i class="pi pi-external-link" /></button> · 基于：<button class="author-link" type="button" @click="openUrl('https://github.com/Gaoshu705/QzoneArchive')">Gaoshu705/QzoneArchive <i class="pi pi-external-link" /></button></p></div></div>
        <span class="version-badge">{{ appVersion ? `v${appVersion}` : "版本未知" }}</span>
      </div>
    </article>
  </section>

  <Dialog v-model:visible="privacyVisible" modal :draggable="false" class="privacy-dialog" header="隐私与数据处理">
    <div class="privacy-content">
      <p>本版本坚持纯本机处理：没有开发者账号系统、云端数据库、遥测、广告追踪或第三方崩溃上报。</p>
      <h4>1. QQ 登录凭证</h4><p>扫码或网页登录产生的 Cookie、p_skey、skey、g_tk 等仅保存在 Rust 后端内存中，不返回前端、不写入日志、不写入磁盘。退出程序或退出登录后会话即被清除。</p>
      <h4>2. 本地归档</h4><p>QQ 空间动态、留言、点赞、评论和归档索引保存在本机 SQLite 数据库。所有读取接口都按当前 QQ 号过滤，避免不同 QQ 账号在界面中串数据。</p>
      <h4>3. 网络请求</h4><p>登录、读取空间资料、归档内容及下载媒体会直接请求腾讯 QQ、QQ 空间及其媒体服务，不经过开发者自建中转服务器。</p>
      <h4>4. 数据清除</h4><p>可以删除当前 QQ 的本地记录，也可以删除全部本地数据。由于上游历史版本的媒体缓存目录是共享的，删除当前账号时会同时清空媒体缓存，以保证已删除账号不留下图片或视频残留。</p>
      <h4>5. 导出文件</h4><p>导出的 HTML 和保存的媒体文件由你自行保管，可能包含昵称、QQ 号、头像和空间内容，请谨慎分享。</p>
    </div>
    <template #footer><Button label="我已了解" @click="privacyVisible = false" /></template>
  </Dialog>

  <Dialog v-model:visible="deleteCurrentVisible" modal :closable="!deletingCurrent" :draggable="false" class="delete-dialog" header="删除当前 QQ 的本地数据？">
    <div class="delete-dialog-content"><span class="delete-warning"><i class="pi pi-exclamation-triangle" /></span><div><p>QQ {{ user?.uin }} 的归档、互动、续传和异常记录将被永久删除，并退出当前登录。</p><small>为确保没有媒体残留，共享图片/视频缓存也会清空；其他 QQ 的数据库记录不会删除，需要时可重新加载媒体。</small></div></div>
    <template #footer><Button label="取消" severity="secondary" text :disabled="deletingCurrent" @click="deleteCurrentVisible = false" /><Button label="确认删除" icon="pi pi-user-minus" severity="danger" :loading="deletingCurrent" @click="deleteCurrent" /></template>
  </Dialog>

  <Dialog v-model:visible="deleteVisible" modal :closable="!deleting" :draggable="false" class="delete-dialog" header="删除所有数据？">
    <div class="delete-dialog-content"><span class="delete-warning"><i class="pi pi-exclamation-triangle" /></span><div><p>所有账号的本地归档和媒体缓存都将被永久删除。</p><small>包括动态、留言、评论、点赞、续传记录、视频缓存及内存中的登录状态。此操作无法撤销。</small></div></div>
    <template #footer><Button label="取消" severity="secondary" text :disabled="deleting" @click="deleteVisible = false" /><Button label="确认全部删除" icon="pi pi-trash" severity="danger" :loading="deleting" @click="deleteEverything" /></template>
  </Dialog>
</template>
