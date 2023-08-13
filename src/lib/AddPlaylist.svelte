<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { addPlaylist } from "../store/playlist";

    const dispatch = createEventDispatcher();

    let url = "";
    let errorMessage = "";
    let loading;

    const urlFormat = "https://www.youtube.com/playlist?list=";
    const error = (msg: string) => {
        errorMessage = msg;
        console.error(msg);
    };

    async function add() {
        if (!url.startsWith(urlFormat)) {
            error("Invalid Playlist URL");
            return;
        }
        try {
            await addPlaylist(url);
            dispatch("load");
            return true;
        } catch (e) {
            error(e);
            return false;
        }
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
                loading = add();
            }}>Add to Library</button
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
