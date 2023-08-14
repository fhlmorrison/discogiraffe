import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";

type Thumbnail = {
    height: number;
    url: string;
    width: number;
};

type PlaylistEntry = {
    availability: string | null;
    channel: string;
    channel_id: string;
    description: string | null;
    duration: number;
    id: string;
    ie_key: string;
    live_status: string | null;
    release_timestamp: number | null;
    thumbnails: Thumbnail[];
    title: string;
    url: string;
    view_count: number | null;
    __x_forwarded_for_ip: string | null;
    _type: string;
};

type PlaylistInfo = {
    availability: string | null;
    channel: string;
    channel_id: string;
    channel_url: string;
    description: string | null;
    entries: PlaylistEntry[];
    epoch: number;
    extractor: string;
    extractor_key: string;
    id: string;
    modified_date: string | null;
    original_url: string;
    playlist_count: number;
    tags: string[];
    thumbnails: Thumbnail[];
    title: string;
    uploader: string;
    uploader_id: string;
    uploader_url: string;
    view_count: number | null;
    webpage_url: string;
    webpage_url_basename: string;
    webpage_url_domain: string;
    __files_to_move: {};
    version: {
        current_git_head: string | null;
        release_git_head: string;
        repository: string;
        version: string;
    };
};

export type dbSong = {
    id: string;
    title: string;
    url: string;
    thumbnail?: string;
    path?: string;
    downloaded: boolean;
    artist?: string;
    album?: string;
    audio_source_url?: string;
    channel: string;
    duration: number;
};


export type dbPlaylist = {
    id: string;
    title: string;
    description: string;
    url: string;
    thumbnail?: string;
    path?: string;
    downloaded: boolean;
}

export type dbPlaylistFull = {
    playlist: dbPlaylist;
    songs: dbSong[];
}


async function savePlaylist(playlist: dbPlaylistFull) {
    // TODO: error handling
    const result = await invoke("savePlaylist", playlist);
}

export async function addPlaylist(url: string) {
    const result = await invoke("add_playlist", { url });
    return result;
}

export async function loadPlaylists() {
    const playlists = await invoke<dbPlaylist[]>("load_playlists");
    playlistLibrary.set(playlists);
    // console.log(playlists);
    return playlists;
}

export async function loadPlaylist(id: string) {
    const data = await invoke<dbPlaylistFull>("load_playlist", { id });
    console.log(data);
    return data;
}

export async function deletePlaylist(id: string) {
    // TODO
}

export async function selectPlaylist(id: string) {
    playlist.set(await loadPlaylist(id));
}

const URL_FORMAT = "https://www.youtube.com/playlist?list=";
export async function getYTPlaylist(url: string) {
    // TODO: error handling
    if (!url.startsWith(URL_FORMAT)) {
        // Error
        return;
    }
    playlist.set(await invoke<dbPlaylistFull>("get_playlist_info", { url }));
    return true;
}

export const playlist = writable<dbPlaylistFull>();

export const playlistLibrary = writable<dbPlaylist[]>([]);