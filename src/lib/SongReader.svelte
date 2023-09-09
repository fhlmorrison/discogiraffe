<script lang="ts">
  import SongListItem from "./SongListItem.svelte";
  import MetaDataViewer from "./MetaDataViewer.svelte";
  import { loadSongsFromFolder } from "./loadAssets";
  import { selectedIndex, openFiles } from "../store/files";

  // let openFiles: OpenFileEntry[];

  const openFolder = () =>
    loadSongsFromFolder().then((res) => {
      console.log(res);
      $openFiles = res;
    });

  const saveSongs = () => {
    openFiles.save();
  };

  $: selectedFile = $openFiles[$selectedIndex];

  const selectSong = (index: number) => {
    openFiles.select(index);
  };
</script>

<div class="col">
  <div class="row">
    <div>
      <button on:click={openFolder}>Open Folder</button>
      {#if $openFiles && $openFiles.length > 0}
        <button on:click={saveSongs}>Save songs to db</button>
      {/if}
    </div>
  </div>

  <MetaDataViewer
    song={selectedFile}
    on:next={() => {
      openFiles.next();
    }}
    on:prev={() => {
      openFiles.prev();
    }}
  />

  {#if openFiles}
    <div class="row">
      <ul>
        {#each $openFiles as file, i}
          <SongListItem
            song={file}
            index={i}
            on:select={(e) => selectSong(e.detail)}
          />
        {/each}
      </ul>
    </div>
  {/if}
</div>

<style>
</style>
