import type { FileEntry } from "@tauri-apps/api/fs";
import { writable } from "svelte/store";

export type OpenFileEntry = FileEntry & { url: string }

export const openFiles = writable<OpenFileEntry[]>([])