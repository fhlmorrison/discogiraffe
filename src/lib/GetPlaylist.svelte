<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { playlist, type dbPlaylistFull } from "../store/playlist";

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
    const data = await invoke<dbPlaylistFull>("get_playlist_info", { url });
    playlist.set(data);
    return true;
  }
</script>

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

<style>
  input {
    min-width: 400px;
  }
  .error {
    border: 1px solid red;
  }
</style>
