import {
  BaseDirectory,
  exists,
  readTextFile,
  writeTextFile,
} from "@tauri-apps/plugin-fs";

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

export type SettingEntry = (
  | DropdownSettingEntry
  | BooleanSettingEntry
  | FolderPathSettingEntry
) & { key: keyof Settings };

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
  theme: string;
  downloadPath?: string;
  autoPlay: boolean;
};

interface SettingsStore {
  settings: Settings;
  load: () => Promise<void>;
  save: () => Promise<void>;
}

class SettingsStoreClass implements SettingsStore {
  settings = Object.fromEntries(
    settingList.map((e) => [e.key, e.default])
  ) as Settings;

  load = async () => {
    // Check if settings file exists
    if (!(await exists("settings.json", { baseDir: BaseDirectory.AppData }))) {
      console.log("Settings file does not exist");
      return;
    }
    // Read settings file
    const settingsFile = await readTextFile("settings.json", {
      baseDir: BaseDirectory.AppData,
    });
    // Parse settings file
    const settingsObject = JSON.parse(settingsFile) as Settings;
    this.settings = settingsObject;
  };

  save = async () => {
    // Get settings
    const settingsToWrite = this.settings;
    // Convert to string
    const settingsToWriteString = JSON.stringify(settingsToWrite);
    // Write to settings file
    await writeTextFile("settings.json", settingsToWriteString, {
      baseDir: BaseDirectory.AppData,
    });
  };
}

export const settingsStore = new SettingsStoreClass();
