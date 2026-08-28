import { invoke } from "@tauri-apps/api/core";

export interface LoginIdentity {
  uin: string;
}

export interface QzoneLoginUser {
  uin: string;
  nickname: string;
  avatarImage?: string;
}

export const getQzoneLoginUser = () => invoke<QzoneLoginUser>("get_login_user");
