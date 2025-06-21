<script lang="ts">
  import { settingsStore } from "./../store/settings.svelte";
  import type { dbSong } from "../store/playlist.svelte";
  import PlaylistItem from "./PlaylistItem.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { playlistStore } from "../store/playlist.svelte";
  import { mapWithConcurrency } from "./concurrentMap.js";
  import { downloadDir } from "@tauri-apps/api/path";
  import { openFiles } from "../store/files";
  import { loadSongsFromPath, loadSongsFromPlaylist } from "./loadAssets";
  import { tabStore } from "../store/tabs.svelte";

  //TODO: Add shift click multi select

  let downloadDirPath: string;

  let downloadPromise: Promise<any> | undefined = $state();

  const setDownloadDir = async (option?: string) =>
    (downloadDirPath = option || (await downloadDir()));

  let selected: boolean[] = $state(
    playlistStore.playlist?.songs.map((item) => false) ?? []
  );

  const initSelected = () => {
    selected = playlistStore.playlist?.songs.map((item) => false) ?? [];
  };

  const selectAll = () => {
    selected =
      playlistStore.playlist?.songs.map((item) => !item.downloaded) ?? [];
  };

  const download = async (item: dbSong): Promise<string> =>
    invoke("download_song", {
      song: item,
      downloadPath: downloadDirPath,
    });

  const downloadSelected = () => {
    const selectedItems =
      playlistStore.playlist?.songs.filter((item, i) => selected[i]) ?? [];
    downloadPromise = mapWithConcurrency(selectedItems, download).then(
      async (results) => {
        console.log(results);
        // Open the newly downloaded files in song reader
        openFiles.set(await loadSongsFromPath(results));
        // TODO: open reader tab
        tabStore.select("/songreader");
      }
    );
  };

  const openDownloaded = async () => {
    openFiles.set(await loadSongsFromPlaylist(playlistStore.playlist));
    // Open reader tab
    tabStore.select("/songreader");
  };

  $effect.pre(() => {
    setDownloadDir(settingsStore.settings.downloadPath);
    if (playlistStore.playlist) {
      initSelected();
    }
  });
</script>

{#if playlistStore.playlist}
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
    {#each playlistStore.playlist.songs as item, i}
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
