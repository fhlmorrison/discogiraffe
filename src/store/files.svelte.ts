import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { change_filename } from "../lib/loadAssets";

export type OpenFileEntry = { name: string; path: string; url: string };

const save = async (filePaths: string[]) => {
  return await invoke<void>("save_folder", { filePaths });
};

interface OpenFileStore {
  openFiles: OpenFileEntry[];
  selectedFile: OpenFileEntry;

  save: () => Promise<void>;
  next: () => void;
  prev: () => void;
  select: (i: number) => void;
  renameSelected: (name: string) => void;
}

class OpenFileStoreClass implements OpenFileStore {
  openFiles = $state<OpenFileEntry[]>([]);
  private selectedIndex = $state(0);
  selectedFile = $derived(this.openFiles[this.selectedIndex]);

  next = () => {
    this.selectedIndex = (this.selectedIndex + 1) % this.openFiles.length;
  };

  prev = () => {
    this.selectedIndex = (this.selectedIndex - 1) % this.openFiles.length;
  };

  select = (i: number) => {
    this.selectedIndex = i;
  };

  renameSelected = (newName: string) => {
    const selectedFile = this.openFiles[this.selectedIndex];
    change_filename(selectedFile.path, newName).then((resultingPath) => {
      console.log(`"${selectedFile.name}"\nrenamed to\n"${newName}"`);
      this.openFiles[this.selectedIndex].name = newName;
      this.openFiles[this.selectedIndex].path = resultingPath;
      this.openFiles[this.selectedIndex].url = convertFileSrc(resultingPath);
    });
  };

  save = async () => await save(this.openFiles.map((file) => file.path));
}

export const openFileStore = new OpenFileStoreClass();
