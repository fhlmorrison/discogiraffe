import { get, writable } from "svelte/store";
import {
  BaseDirectory,
  exists,
  readTextFile,
  writeTextFile,
} from "@tauri-apps/plugin-fs";

type SettingValue = string | number | boolean;

// Types: [dropdown, text field, number field, boolean toggle]

// Folder Path
type FolderPathSettingEntry = {
  key: string;
  default: string;
  name: string;
  type: "folderPath";
};

// Dropdown\
type DropdownSettingEntry = {
  key: string;
  default: string;
  options: DropdownSettingEntryOption[];
  name: string;
  type: "dropdown";
};

type DropdownSettingEntryOption = {
  key: string;
  name: string;
};

// Boolean

type BooleanSettingEntry = {
  key: string;
  default: boolean;
  name: string;
  type: "boolean";
};

export type SettingEntry =
  | DropdownSettingEntry
  | BooleanSettingEntry
  | FolderPathSettingEntry;

export const settingList: SettingEntry[] = [
  {
    key: "theme",
    default: "light",
    name: "Theme",
    type: "dropdown",
    options: [
      {
        key: "light",
        name: "Light",
      },
      {
        key: "dark",
        name: "Dark",
      },
    ],
  },
  {
    key: "downloadPath",
    default: "",
    name: "Download Path",
    type: "folderPath",
  },
  {
    key: "autoPlay",
    default: false,
    name: "Auto Play",
    type: "boolean",
  },
];

type Settings = {
  [key: string]: SettingValue;
};

const defaults = {};

const { subscribe, set, update } = writable<Settings>({});

const load = async () => {
  // Check if settings file exists
  if (!(await exists("settings.json", { baseDir: BaseDirectory.AppData }))) {
    console.log("Settings file does not exist");
    return {};
  }
  // Read settings file
  const settingsFile = await readTextFile("settings.json", {
    baseDir: BaseDirectory.AppData,
  });
  // Parse settings file
  const settingsObject = JSON.parse(settingsFile) as Settings;
  set(settingsObject);
};
const save = async () => {
  // Get settings
  const settingsToWrite = get(settings);
  // Convert to string
  const settingsToWriteString = JSON.stringify(settingsToWrite);
  // Write to settings file
  await writeTextFile("settings.json", settingsToWriteString, {
    baseDir: BaseDirectory.AppData,
  });
};

const getValue = (e: SettingEntry) => {
  return get(settings[e.key]) ?? e.default;
};

export const settings = {
  subscribe,
  set,
  update,
  load,
  save,
};
