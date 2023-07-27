<script lang="ts">
  import { settings } from "./store/settings";
  import FaCog from "svelte-icons/fa/FaCog.svelte";
  import PlaylistDisplay from "./lib/PlaylistDisplay.svelte";
  import GetPl from "./lib/GetPlaylist.svelte";
  import MetaDataReader from "./lib/SongReader.svelte";
  import Router from "./lib/Router.svelte";
</script>

<header>
  <button class="settings">
    <FaCog />
  </button>
</header>

<main class="container">
  <!-- <h1>YouTube Playlist Downloader</h1>
  <div class="row">
    <div class="playlist-section">
      <div class="row">
        <GetPl />
      </div>
      <PlaylistDisplay />
    </div>
  </div>
  <div class="row">
    <MetaDataReader />
  </div> -->
  <Router />
</main>

<style>
  .playlist-section {
    width: clamp(400px, 100%, 800px);
  }

  .settings {
    width: 64px;
    cursor: pointer;
    float: left;
    justify-self: left;
    padding: 5px;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
</style>
