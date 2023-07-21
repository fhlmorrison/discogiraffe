<script lang="ts">
    import SongListItem from "./SongListItem.svelte";
    import SongPlayer from "./MetaDataViewer.svelte";
    import { loadAssets, loadSongsFromFolder } from "./loadAssets";
    import type { FileEntry } from "@tauri-apps/api/fs";

    let openFiles: (FileEntry & { url: string })[];

    const openFolder = () =>
        loadSongsFromFolder().then((res) => {
            console.log(res);
            openFiles = res;
        });

    let selected: FileEntry & { url: string };

    const selectSong = (song: FileEntry & { url: string }) => {
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
                {#each openFiles as file, i}
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
