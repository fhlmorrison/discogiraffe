<script lang="ts">
  import { onMount } from "svelte";
  import {
    type dbPlaylist,
    addPlaylist,
    playlistStore,
  } from "../store/playlist.svelte";
  import AddPlaylist from "./AddPlaylist.svelte";
  import MdRefresh from "svelte-icons/md/MdRefresh.svelte";

  import { tabStore } from "../store/tabs.svelte";

  let refreshPromise = $state<Promise<void> | undefined>();

  onMount(() => {
    loadList();
  });

  let adding = $state(false);

  const openModal = () => {
    adding = true;
  };
  const closeModal = () => {
    adding = false;
  };
  const choosePlaylist = (selectedPlaylist: dbPlaylist) => {
    console.log("selected", selectedPlaylist);
    playlistStore.selectPlaylist(selectedPlaylist.id);
    // Open playlist list view
    tabStore.select("/playlist");
  };
  const loadList = () => {
    playlistStore.loadPlaylists();
    console.log("loaded");
  };
  const refreshPlaylist = async (pl: dbPlaylist) => {
    refreshPromise = addPlaylist(pl.url);
    refreshPromise
      .then(() => {
        playlistStore.loadPlaylists().then(() => {
          console.log("loaded");
        });
      })
      .catch((e) => {
        console.error(e);
      });
  };
</script>

<div class="grid">
  <div class="card add" onclick={openModal}>+</div>
  {#if adding}
    <AddPlaylist
      onClose={closeModal}
      onLoad={() => {
        closeModal();
        loadList();
      }}
    />
  {/if}
  {#each playlistStore.playlistLibrary as playlist}
    <div
      class="card playlist"
      onclick={() => choosePlaylist(playlist)}
      onkeydown={() => {}}
    >
      {#await refreshPromise}
        <button
          class="refresh loading"
          disabled
          onclick={(e) => {
            e.preventDefault();
            e.stopPropagation();
            refreshPlaylist(playlist);
          }}><MdRefresh /></button
        >
      {:then}
        <button
          class="refresh"
          onclick={(e) => {
            e.preventDefault();
            e.stopPropagation();
            refreshPlaylist(playlist);
          }}><MdRefresh /></button
        >
      {:catch error}
        <button
          class="refresh error"
          onclick={(e) => {
            e.preventDefault();
            e.stopPropagation();
            refreshPlaylist(playlist);
          }}><MdRefresh /></button
        >
      {/await}
      <img src={playlist.thumbnail} alt={playlist.title} />
      <div class="playlist-title">{playlist.title}</div>
      <div class="playlist-description">{playlist.description}</div>
    </div>
  {/each}
  {#if !playlistStore.playlistLibrary.length}
    <div class="card playlist">
      <img src={"/record.png"} alt={"test image"} />
      <div class="playlist-title">{"Example Playlist"}</div>
      <div class="playlist-description">
        {"Use the plus button to save playlist from YouTube"}
      </div>
    </div>
  {/if}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    overflow: auto;
  }

  .card {
    position: relative;
    border: 1px solid black;
    border-radius: 5px;
    margin: 5px;
    padding: 5px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    cursor: pointer;
  }

  .add {
    font-size: 5em;
  }

  .card:hover {
    opacity: 0.8;
    border-color: #aaa;
  }

  .card > img {
    width: 100%;
    height: auto;
    aspect-ratio: 1/1;
    object-fit: cover;
  }

  .refresh {
    position: absolute;
    top: 0;
    right: 0;
    background-color: #aaa;
    border: 1 px solid black;
    cursor: pointer;
    border-radius: 50%;
    padding: 0;
    height: 30px;
    width: 30px;
    align-self: center;
    justify-self: center;
  }
  /* Define the rotating animation */
  @keyframes spin {
    100% {
      transform: rotate(360deg);
    }
  }
  .loading {
    border-color: #0f0;
    animation: spin 1s linear infinite;
  }

  .error {
    border-color: #f00;
  }
</style>
