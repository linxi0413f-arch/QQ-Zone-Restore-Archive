import { invoke } from "@tauri-apps/api/core";

export interface LoginIdentity {
  uin: string;
}

export interface WebLoginStatus {
  status: "success" | "error" | "webLoginOpened" | "webLoginWaiting" | "webLoginCancelled";
  message: string;
  user?: LoginIdentity;
}

export const openWebLogin = () => invoke<WebLoginStatus>("open_web_login");
export const checkWebLogin = () => invoke<WebLoginStatus>("check_web_login");

export interface QzoneLoginUser {
  uin: string;
  nickname: string;
  avatarImage?: string;
}

export const getQzoneLoginUser = () => invoke<QzoneLoginUser>("get_login_user");
