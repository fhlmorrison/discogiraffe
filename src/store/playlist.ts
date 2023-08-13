import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";

type Thumbnail = {
    height: number;
    url: string;
    width: number;
};

export type PlaylistEntry = {
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

export type PlaylistInfo = {
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

type dbPlaylistEntry = {
    id?: number;
    title: string;
    url: string;
    thumbnail: string;
    path?: string;
    downloaded: boolean;
    artist?: string;
    album?: string;
    audioSourceUrl?: string;
    uploader: string;
};


type dbPlaylist = {
    id?: number;
    title: string;
    description: string;
    url: string;
    thumbnail: string;
    path?: string;
    downloaded: boolean;
}

function trimPlaylist(playlist: PlaylistInfo): [dbPlaylist, dbPlaylistEntry[]] {
    const { entries, ...rest } = playlist;

    const trimmedPlaylist = {
        title: rest.title,
        description: rest.description,
        url: rest.webpage_url,
        thumbnail: rest.thumbnails[0].url,
        downloaded: false
    }
    const trimmedEntries = entries.map(entry => ({
        title: entry.title,
        url: entry.url,
        thumbnail: entry.thumbnails[0].url,
        downloaded: false,
        uploader: entry.channel,
    }))

    return [trimmedPlaylist, trimmedEntries];
}

async function savePlaylist(playlist: PlaylistInfo) {
    const [playlistInfo, songs] = trimPlaylist(playlist);
    const result = await invoke("savePlaylist", { playlistInfo, songs });
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

export const playlist = writable<PlaylistInfo>();

export const playlistLibrary = writable<dbPlaylist[]>([]);