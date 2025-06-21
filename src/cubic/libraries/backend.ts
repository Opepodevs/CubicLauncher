import { invoke } from "@tauri-apps/api/core";

export function closeLauncher() {
  invoke("close_window");
}

export function hideLauncher() {
  console.log("xd");
  invoke("minimize_window");
}
export function maximizeLauncher() {
  invoke("maximize_window");
}
