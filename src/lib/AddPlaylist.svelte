<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { playlist } from "../store/playlist";
    import type { PlaylistInfo } from "../store/playlist";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    let url = "";
    let errorMessage = "";
    let loading;

    const urlFormat = "https://www.youtube.com/playlist?list=";
    const error = (msg: string) => {
        errorMessage = msg;
        console.error(msg);
    };

    async function get() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        if (!url.startsWith(urlFormat)) {
            error("Invalid Playlist URL");
            return;
        }
        errorMessage = "";
        playlist.set(
            JSON.parse(
                await invoke<string>("get_playlist_info", { url })
            ) as PlaylistInfo
        );
        dispatch("load");
        return true;
    }

    const closeModal = () => {
        dispatch("close");
    };
</script>

<div class="modal" on:click={closeModal} />
<div class="content">
    <div>
        <input
            class={errorMessage ? "error" : ""}
            placeholder={errorMessage || "Enter a YouTube playlist URL..."}
            bind:value={url}
        />
        <button
            on:click={() => {
                loading = get();
            }}>Get Playlist Info</button
        >
    </div>
    {#await loading}
        <p>loading...</p>
    {:then res}
        {#if res}
            <p>playlist loaded</p>
        {/if}
    {:catch error}
        <p style="color: red">{error.message}</p>
    {/await}
</div>

<style>
    input {
        min-width: 400px;
    }
    .error {
        border: 1px solid red;
    }
    .modal {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .content {
        position: fixed;
        top: 50%;
        left: 50%;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: #2f2f2f;
        padding: 20px;
        border-radius: 25px;
        transform: translate(-50%, -50%);
    }
</style>
