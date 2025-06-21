<script lang="ts">
  import SongListItem from "./SongListItem.svelte";
  import MetaDataViewer from "./MetaDataViewer.svelte";
  import { change_filename, loadSongsFromFolder } from "./loadAssets";
  import { openFileStore } from "../store/files.svelte";

  // let openFiles: OpenFileEntry[];

  const openFolder = () =>
    loadSongsFromFolder().then((res) => {
      console.log(res);
      openFileStore.openFiles = res || [];
    });

  const saveSongs = () => {
    openFileStore.save();
  };

  const selectSong = (index: number) => {
    openFileStore.select(index);
  };

  const changeFilename = (fileName: string) => {
    openFileStore.renameSelected(fileName);
  };
</script>

<div class="col">
  <div class="row">
    <div>
      <button onclick={openFolder}>Open Folder</button>
      {#if openFileStore.openFiles && openFileStore.openFiles.length > 0}
        <button onclick={saveSongs}>Save songs to db</button>
      {/if}
    </div>
  </div>

  <MetaDataViewer
    song={openFileStore.selectedFile}
    onChangeFilename={changeFilename}
  />
</div>

{#if openFileStore.openFiles && openFileStore.openFiles.length > 0}
  <div class="list">
    <ol>
      {#each openFileStore.openFiles as file, i}
        <SongListItem
          song={file}
          index={i}
          onSelect={(idx) => selectSong(idx)}
        />
      {/each}
    </ol>
  </div>
{/if}

<style>
  .list {
    display: flex;
    flex-direction: column;
    justify-content: left;
    overflow-y: scroll;
    overflow-x: hidden;
  }
</style>
