import { get, writable } from "svelte/store";

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
];

type Settings = {
  [key: string]: SettingValue;
};

const defaults = {};

const { subscribe, set, update } = writable<Settings>({});

const load = () => {
  // Read from settings file
};
const save = () => {
  // Write to settings file
  // change locally
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
