import { convertFileSrc } from "@tauri-apps/api/core";

export function getFileUrl(path: string) {
  return convertFileSrc(path);
}
