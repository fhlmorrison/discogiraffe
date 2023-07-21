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

export const playlist = writable<PlaylistInfo>();