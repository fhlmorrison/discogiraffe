import { writable } from "svelte/store";
import GetPlaylist from "../lib/GetPlaylist.svelte";
import PlaylistDisplay from "../lib/PlaylistDisplay.svelte";
import MetaDataReader from "../lib/SongReader.svelte";
import PlaylistLibrary from "../lib/PlaylistLibrary.svelte";

export const routes: Route[] = [
  {
    name: "Playlist Library",
    path: "/",
    component: PlaylistLibrary,
  },
  // {
  //   name: "Get Playlist",
  //   path: "/getplaylist",
  //   component: GetPlaylist,
  // },
  {
    name: "Playlist Display",
    path: "/playlist",
    component: PlaylistDisplay,
  },
  {
    name: "Song Reader",
    path: "/songreader",
    component: MetaDataReader,
  },
];

export type Route = {
  name: string;
  path: string;
  component: any;
};

const { subscribe, set } = writable<Route>(routes[0]);
export const currentTab = {
  subscribe,
  set,
  select: (path: string) => {
    const route = routes.find((r) => r.path === path);
    if (route) {
      set(route);
    }
  },
};
