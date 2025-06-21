import PlaylistDisplay from "../lib/PlaylistDisplay.svelte";
import MetaDataReader from "../lib/SongReader.svelte";
import PlaylistLibrary from "../lib/PlaylistLibrary.svelte";

export const routes: Route[] = [
  {
    name: "Playlist Library",
    path: "/",
    component: PlaylistLibrary,
  },
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

interface TabStore {
  current: Route;
  select: (path: string) => void;
}

class TabStoreCLass implements TabStore {
  current: Route = $state(routes[0]);

  select = (path: string) => {
    const route = routes.find((r) => r.path === path);
    if (route) {
      this.current = route;
    }
  };
}

export const tabStore = new TabStoreCLass();
