import { invoke } from "@tauri-apps/api";
import type { FileEntry } from "@tauri-apps/api/fs";
import { get, writable } from "svelte/store";

export type OpenFileEntry = FileEntry & { url: string };

const { subscribe, set, update } = writable<OpenFileEntry[]>([]);

const save = async () => {
  const files = get(openFiles).map(({ path }) => path);
  return await invoke("save_folder", { files });
};

export const openFiles = {
  subscribe,
  set,
  update,
  save,
};
