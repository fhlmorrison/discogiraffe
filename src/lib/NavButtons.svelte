<script lang="ts">
  import GetPlaylist from "./GetPlaylist.svelte";
  import PlaylistDisplay from "./PlaylistDisplay.svelte";
  import MetaDataReader from "./SongReader.svelte";
  import PlaylistLibrary from "./PlaylistLibrary.svelte";
  import { currentTab } from "../store/tabs";
  import type { Route } from "../store/tabs";

  let routes: Route[] = [
    {
      name: "Playlist Library",
      path: "/",
      component: PlaylistLibrary,
    },
    {
      name: "Get Playlist",
      path: "/getplaylist",
      component: GetPlaylist,
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

  currentTab.set(routes[0]);
</script>

<div>
  {#each routes as route}
    <button
      class={route === $currentTab ? "active" : ""}
      on:click={() => {
        currentTab.set(route);
      }}>{route.name}</button
    >
  {/each}
</div>

<style>
  /* your styles go here */
  .active {
    background-color: #4caf50;
    color: white;
  }
</style>
