<script lang="ts">
  // import { open } from "@tauri-apps/plugin-dialog";
  import { settings } from "./../store/settings";
  import type { dbSong } from "../store/playlist";
  import PlaylistItem from "./PlaylistItem.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { playlist } from "../store/playlist";
  import { mapWithConcurrency } from "./concurrentMap.js";
  import { downloadDir } from "@tauri-apps/api/path";
  import { openFiles } from "../store/files";
  import { loadSongsFromPath, loadSongsFromPlaylist } from "./loadAssets";
  import { currentTab } from "../store/tabs";

  //TODO: Add shift click multi select
  console.log("Playlist", $playlist);

  let downloadDirPath: string;

  let downloadPromise: Promise<any> | undefined = $state();

  const setDownloadDir = async (option?: string) =>
    (downloadDirPath = option || (await downloadDir()));

  let selected: boolean[] = $state($playlist?.songs.map((item) => false));

  const initSelected = () => {
    selected = $playlist?.songs.map((item) => false);
  };

  const selectAll = () => {
    selected = $playlist?.songs.map((item) => !item.downloaded);
  };

  const download = async (item: dbSong): Promise<string> =>
    invoke("download_song", {
      song: item,
      downloadPath: downloadDirPath,
    });

  const downloadSelected = () => {
    const selectedItems = $playlist?.songs.filter((item, i) => selected[i]);
    downloadPromise = mapWithConcurrency(selectedItems, download).then(
      async (results) => {
        console.log(results);
        // Open the newly downloaded files in song reader
        openFiles.set(await loadSongsFromPath(results));
        // TODO: open reader tab
        currentTab.select("/songreader");
      }
    );
  };

  const openDownloaded = async () => {
    openFiles.set(await loadSongsFromPlaylist($playlist));
    // Open reader tab
    currentTab.select("/songreader");
  };

  $effect.pre(() => {
    setDownloadDir($settings?.downloadPath);
    if ($playlist) {
      initSelected();
    }
  });
</script>

{#if $playlist}
  <div class="button-row">
    {#await downloadPromise}
      <button class="download loading" disabled onclick={downloadSelected}
        >Downloading...</button
      >
    {:then}
      <button class="download" onclick={downloadSelected}
        >Download Selected</button
      >
    {:catch error}
      <button class="download error" onclick={downloadSelected}
        >Download Selected</button
      >
    {/await}
    <button class="open" onclick={openDownloaded}>Open Songs</button>
    <button onclick={initSelected}>Deselect All</button>
    <button onclick={selectAll}>Select All</button>
  </div>
  <div class="list">
    {#each $playlist.songs as item, i}
      <PlaylistItem {item} bind:selected={selected[i]} />
    {/each}
  </div>
{:else}
  <p>No playlist loaded</p>
{/if}

<style>
  .button-row {
    display: flex;
    flex-direction: row;
    justify-content: right;
    align-items: center;
    margin: 10px 0;
    gap: 10px;
  }

  .download {
    background-color: #4caf50;
    border: none;
    color: white;
    padding: 10px 20px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    margin-right: auto;
  }

  .open {
    /* background-color: #4caf50; */
    /* color: white; */
    /* border: none; */
    padding: 10px 20px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    margin-right: auto;
  }

  .list {
    display: flex;
    flex-direction: column;
    justify-content: left;
    align-items: center;
    overflow-y: scroll;
    overflow-x: hidden;
  }

  .loading {
    border-color: #ffe600;
  }

  .error {
    border-color: #ff0000;
  }
</style>
