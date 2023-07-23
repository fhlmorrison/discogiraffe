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

    const selectSong = (song: OpenFileEntry) => {
        selected = song;
    };
</script>

<div class="col">
    <div class="row">
        <button on:click={openFolder}>Open Folder</button>
    </div>

    <SongPlayer song={selected} />

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
