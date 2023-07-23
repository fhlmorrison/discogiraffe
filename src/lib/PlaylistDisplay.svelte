<script lang="ts">
    import type { PlaylistEntry } from "../store/playlist";
    import PlaylistItem from "./PlaylistItem.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { playlist } from "../store/playlist";
    import { mapWithConcurrency } from "./concurrentMap.js";
    import { downloadDir } from "@tauri-apps/api/path";
    import { onMount } from "svelte/internal";

    //TODO: Add shift click multi select

    let downloadDirPath;
    onMount(async () => (downloadDirPath = await downloadDir()));

    let selected: boolean[] = $playlist?.entries.map((item) => false);
    $: if ($playlist) {
        initSelected();
    }

    const initSelected = () => {
        selected = $playlist?.entries.map((item) => false);
    };

    const selectAll = () => {
        selected = $playlist?.entries.map((item) => true);
    };

    const download = async (item: PlaylistEntry): Promise<string> =>
        invoke("download_song", {
            url: item.url,
            downloadPath: downloadDirPath,
        });

    const downloadSelected = () => {
        const selectedItems = $playlist?.entries.filter(
            (item, i) => selected[i]
        );
        mapWithConcurrency(selectedItems, download).then((results) => {
            console.log(results);
        });
    };
</script>

{#if $playlist}
    <div class="button-row">
        <button class="download" on:click={downloadSelected}
            >Download Selected</button
        >
        <button on:click={initSelected}>Deselect All</button>
        <button on:click={selectAll}>Select All</button>
    </div>
    <div class="list">
        {#each $playlist.entries as item, i}
            <PlaylistItem {item} bind:selected={selected[i]} />
        {/each}
    </div>
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

    .list {
        display: flex;
        flex-direction: column;
        justify-content: left;
        align-items: center;
    }
</style>
