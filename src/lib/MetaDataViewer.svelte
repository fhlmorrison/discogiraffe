<script lang="ts">
  import type { MetadataEntry } from "./loadAssets";
  import {
    change_filename,
    getMetadata,
    getPictureData,
    writeMetadata,
  } from "./loadAssets";
  import SongPlayer from "./SongPlayer.svelte";
  import type { OpenFileEntry } from "../store/files";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let song: OpenFileEntry;

  let DEFAULT_IMAGE = "./record.png";

  let fileName: string = "";

  $: setFilename(song?.name);

  const setFilename = (name: string) => {
    fileName = name ?? "";
  };

  $: loadMetadata(song?.path);
  let metadata: MetadataEntry[] = [
    {
      key: "Title",
      value: "",
    },
    {
      key: "Artist",
      value: "",
    },
    {
      key: "Album",
      value: "",
    },
    {
      key: "AudioSourceUrl",
      value: "",
    },
  ];
  let backup: MetadataEntry[] = [];
  let error;

  const loadMetadata = async (src) => {
    try {
      metadata = await getMetadata(src);
      backup = metadata.map((entry) => ({ ...entry }));
    } catch (e) {
      error = e;
    }
  };

  let editMode: boolean = false;

  const save = async () => {
    console.log("save");
    const diffEntries = diff(metadata, backup);
    editMode = false;
    if (diffEntries.length === 0) {
      return;
    }
    try {
      await writeMetadata(song?.path, diffEntries);
      setTimeout(() => {
        loadMetadata(song?.path);
      }, 100);
    } catch (e) {
      error = e;
    }
  };

  const cancel = () => {
    metadata = backup;
    editMode = false;
  };

  const diff = (a: MetadataEntry[], b: MetadataEntry[]) => {
    const res: MetadataEntry[] = [];
    a.forEach((entry) => {
      const match = b.find((e) => e.key === entry.key);
      if (match && match.value !== entry.value) {
        res.push(entry);
      }
    });
    return res;
  };

  let pictureData: string = DEFAULT_IMAGE;

  const getPicture = async () => {
    pictureData = (await getPictureData(song?.path)) || DEFAULT_IMAGE;
  };

  $: song && getPicture();

  $: fileName = fileName.replace(/[<>:/\\\|\?"\*\^]/g, "");

  const changeFilename = () => {
    dispatch("changeFilename", fileName);
  };
</script>

<!-- {song?.name || "No song selected"} -->
<!-- <audio src={song?.url || ""} /> -->

<div>
  {#if editMode}
    <button class="save" on:click={save}> Save </button>
    <button class="cancel" on:click={cancel}> Cancel </button>
  {:else}
    <button on:click={() => (editMode = true)}> Edit </button>
    <!-- else content here -->
  {/if}
</div>
<div class="inputs">
  <div class="cover-art">
    <img
      src={pictureData}
      alt="cover art"
      on:error={() => {
        pictureData = DEFAULT_IMAGE;
      }}
      height="256px"
    />
  </div>
  <div class="filename field">
    Filename
    <div class="filename">
      <input type="text" bind:value={fileName} maxlength="100" />
      <button on:click={changeFilename}> Save </button>
    </div>
  </div>
  {#each metadata as entry}
    <!-- content here -->
    <div class="field">
      {entry.key}
      <input readonly={!editMode} type="text" bind:value={entry.value} />
    </div>
  {/each}
</div>

<!-- <SongPlayer /> -->

<style>
  .save {
    background-color: #9d9;
  }
  .cancel {
    background-color: #d99;
  }
  .inputs {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    grid-template-rows: 1fr 1fr 1fr;
    grid-gap: 1em;
  }
  .field {
    display: flex;
    flex-direction: column;
  }
  .cover-art {
    grid-area: 1 / 1 / 4 / 2;
  }
  .cover-art img {
    height: 256px;
    aspect-ratio: 1/1;
    object-fit: cover;
    border-radius: 10px;
    align-content: center;
  }
  .filename {
    grid-area: 1 / 2 / 2 / 4;
    display: flex;
  }
  .filename input {
    flex-grow: 1;
  }
  input[readonly] {
    filter: invert(0.15);
  }
</style>
