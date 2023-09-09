<script lang="ts">
  import Router from "./lib/Router.svelte";
  import Settings from "./lib/Settings.svelte";
</script>

<header>
  <Settings />
</header>

<main class="container">
  <Router />
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
</main>

<style>
  .playlist-section {
    width: clamp(400px, 100%, 800px);
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
</style>
