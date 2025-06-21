import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { get, writable } from "svelte/store";

export type OpenFileEntry = { name: string; path: string; url: string };

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
const rename = (i: number, name: string, path: string = "") => {
  update((files) => {
    files[i].name = name;
    files[i].path = path || files[i].path;
    files[i].url = convertFileSrc(files[i].path);
    return files;
  });
};

export const openFiles = {
  subscribe,
  set: (files: OpenFileEntry[] | undefined) => {
    set(files || []);
    select(0);
  },
  update,
  save,
  next,
  prev,
  select,
  rename,
};
