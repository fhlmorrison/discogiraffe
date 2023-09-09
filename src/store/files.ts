import { invoke } from "@tauri-apps/api";
import type { FileEntry } from "@tauri-apps/api/fs";
import { get, writable } from "svelte/store";

export type OpenFileEntry = FileEntry & { url: string };

const { subscribe, set, update } = writable<OpenFileEntry[]>([]);

const { subscribe: subscribeIndex, update: updateIndex } = writable<number>(0);

export const selectedIndex = {
  subscribe: subscribeIndex,
};

const save = async () => {
  const files = get(openFiles).map(({ path }) => path);
  return await invoke("save_folder", { files });
};

const length = () => get(openFiles).length;
const next = () => {
  updateIndex((i) => (i + 1) % length());
};
const prev = () => {
  const len = length();
  updateIndex((i) => (i - 1 + len) % len);
};
const select = (i: number) => {
  updateIndex(() => i);
};

export const openFiles = {
  subscribe,
  set: (files: OpenFileEntry[]) => {
    set(files);
    select(0);
  },
  update,
  save,
  next,
  prev,
  select,
};
