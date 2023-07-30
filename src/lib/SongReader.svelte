<script lang="ts">
    import SongListItem from "./SongListItem.svelte";
    import SongPlayer from "./MetaDataViewer.svelte";
    import { loadSongsFromFolder } from "./loadAssets";
    import type { OpenFileEntry } from "src/store/files";
    import { openFiles } from "../store/files";

    // let openFiles: OpenFileEntry[];

    const openFolder = () =>
        loadSongsFromFolder().then((res) => {
            console.log(res);
            $openFiles = res;
        });

    let selected: OpenFileEntry;
    let selectedIndex = 0;

    const selectSong = (index: number) => {
        selectedIndex = index;
    };

    $: $openFiles && $openFiles.length > 0 && (selectedIndex = 0);
    $: selected = $openFiles[selectedIndex];
</script>

<div class="col">
    <div class="row">
        <button on:click={openFolder}>Open Folder</button>
    </div>

    <SongPlayer
        song={selected}
        on:next={() => {
            console.log("next received");
            selectedIndex = (selectedIndex + 1) % $openFiles.length;
        }}
        on:prev={() => {
            console.log("prev received");
            selectedIndex =
                (selectedIndex - 1 + $openFiles.length) % $openFiles.length;
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
