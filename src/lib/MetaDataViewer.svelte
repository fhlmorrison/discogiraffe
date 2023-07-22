<script lang="ts">
    import type { MetadataEntry } from "./loadAssets";
    import type { FileEntry } from "@tauri-apps/api/fs";
    import { getMetadata, getPictureData, writeMetadata } from "./loadAssets";
    import { onMount } from "svelte";
    export let song: FileEntry & { url: string };

    let DEFAULT_IMAGE = "./record.png";

    // $: audioFile = new Audio(song?.url || "");
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

    const flags = {
        title: false,
        artist: false,
        album: false,
        audioSourceUrl: false,
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
        } catch (e) {
            error = e;
        }
        loadMetadata(song?.path);
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

    const newKey = (key) => {
        switch (key) {
            case "title":
                return "Title";
            case "artist":
                return "Artist";
            case "album":
                return "Album";
            case "audioSourceUrl":
                return "Audio Source Url";
            default:
                return key;
        }
    };

    let pictureData: string = DEFAULT_IMAGE;

    const getPicture = async () => {
        pictureData = (await getPictureData(song?.path)) || DEFAULT_IMAGE;
    };

    $: song && getPicture();
</script>

{song?.name || "No song selected"}
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
    {#each metadata as entry}
        <!-- content here -->
        <div class="field">
            {entry.key}
            <input readonly={!editMode} type="text" bind:value={entry.value} />
        </div>
    {/each}
</div>
<button on:click={getPicture}> get pic </button>

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
        grid-template-rows: 1fr 1fr;
        grid-gap: 1em;
    }
    .field {
        display: flex;
        flex-direction: column;
    }
    .cover-art {
        grid-area: 1 / 1 / 3 / 2;
    }
    .cover-art img {
        height: 256px;
        aspect-ratio: 1/1;
        object-fit: cover;
        border-radius: 10px;
        align-content: center;
    }
</style>
