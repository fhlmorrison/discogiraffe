<script lang="ts">
  import { addPlaylist } from "../store/playlist.svelte";

  let url = $state("");
  let errorMessage = $state("");
  let loading = $state();

  interface Props {
    onClose: () => void;
    onLoad: () => void;
  }

  let { onClose, onLoad }: Props = $props();

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
      onLoad();
      return true;
    } catch (e) {
      error(e.massage);
      return false;
    }
  }

  const closeModal = () => {
    onClose();
  };
</script>

<div class="modal" onclick={closeModal}></div>
<div class="content">
  <div>
    <input
      class={errorMessage ? "error" : ""}
      placeholder={errorMessage || "Enter a YouTube playlist URL..."}
      bind:value={url}
    />
    <button
      onclick={() => {
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
    background-color: var(--background-color);
    padding: 20px;
    border-radius: 25px;
    transform: translate(-50%, -50%);
  }
</style>
