import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { readDir } from "@tauri-apps/plugin-fs";
import type { DirEntry } from "@tauri-apps/plugin-fs";
import { basename, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { dbPlaylistFull } from "src/store/playlist";

type SongEntry = DirEntry & { path: string };

const allowedTypes = ["mp3"];

const arrayify = <T>(input: T | T[]): T[] =>
  Array.isArray(input) ? input : [input];

export async function loadAssets() {
  const result = arrayify(
    await open({
      multiple: true,
      defaultPath: "~/music",
      filters: [
        {
          name: "Audio Files",
          extensions: allowedTypes,
        },
      ],
    })
  );
  if (result === undefined) return;
  return result;
}

export async function openFolder() {
  const result = await open({
    multiple: false,
    directory: true,
    defaultPath: "~/music",
  });
  return result;
}

// Mutual recursion to get all files in a folder
const processEntries = async (dirPath: string, entries: DirEntry[]) =>
  await Promise.all(
    entries.flatMap(async (entry) => ({
      ...entry,
      path: await join(dirPath, entry.name),
    }))
  );

export async function loadAssetsFromFolder() {
  const dirPath = await openFolder();
  if (!dirPath) return;
  const entries = await readDir(dirPath);
  const files = await processEntries(dirPath, entries);
  return files.filter(({ name }) =>
    allowedTypes.some((type) => name.endsWith(type))
  );
}

export async function getUrl(path: string): Promise<string> {
  return convertFileSrc(path);
}

export async function loadSongsFromFolder() {
  const files = await loadAssetsFromFolder();
  if (files === undefined) return;
  const songs = await Promise.all(
    files.map(async ({ name, path }) => {
      const url = await getUrl(path);
      return { name, url, path };
    })
  );
  return songs;
}

export async function loadSongsFromPath(files: string[]) {
  if (files === undefined) return;
  const songs = await Promise.all(
    files.map(async (path) => {
      const url = await getUrl(path);
      const name = await basename(path);
      return { name, url, path };
    })
  );
  return songs;
}

export async function loadSongsFromPlaylist(playlist: dbPlaylistFull) {
  const localSongs = playlist.songs
    .filter((song) => song.path)
    .map((song) => song.path as string);
  return await loadSongsFromPath(localSongs);
}

export async function getMetadata(src: string) {
  // return await musicMetadata.fetchFromUrl(convertFileSrc(src));
  // console.log("getting metadata", src);
  return await invoke<MetadataEntry[]>("get_metadata", { src });
}

type MetadataKey =
  | "Title"
  | "Artist"
  | "Album"
  | "AudioSourceUrl"
  | "PictureData";

export type MetadataEntry = {
  key: MetadataKey;
  value: string;
};

type WriteMetadataEvent = {
  src: string;
  metadata: MetadataEntry[];
};

export async function writeMetadata(src: string, metadata: MetadataEntry[]) {
  const event: WriteMetadataEvent = { src, metadata };
  console.log(event);
  invoke("write_metadata", { event });
}

type CoverArt = {
  mime_type: String;
  b64: String;
};

export async function getPictureData(src: string): Promise<string> {
  const pictureString = await invoke<CoverArt>("get_cover_art", { src });
  return pictureString.b64
    ? `data:${pictureString.mime_type};base64, ${pictureString.b64}`
    : "";
}

export async function change_filename(src: string, newName: string) {
  return invoke<string>("change_filename", { src, newName });
}
