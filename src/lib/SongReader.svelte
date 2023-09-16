<script lang="ts">
  import SongListItem from "./SongListItem.svelte";
  import MetaDataViewer from "./MetaDataViewer.svelte";
  import { change_filename, loadSongsFromFolder } from "./loadAssets";
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

  const changeFilename = (e) => {
    const fileName = e.detail;
    change_filename(selectedFile.path, fileName).then((resultingPath) => {
      console.log(`"${selectedFile.name}"\nrenamed to\n"${fileName}"`);
      openFiles.rename($selectedIndex, fileName, resultingPath);
    });
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

  <MetaDataViewer song={selectedFile} on:changeFilename={changeFilename} />

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
