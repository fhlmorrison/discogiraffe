<script lang="ts">
  import { currentTab } from "./store/tabs";
  import Settings from "./lib/Settings.svelte";
  import SongPlayer from "./lib/SongPlayer.svelte";
  import NavButtons from "./lib/NavButtons.svelte";

  const SvelteComponent = $derived($currentTab.component);
</script>

<header>
  <Settings />
  <NavButtons />
</header>

<main class="container">
  <!-- <Router /> -->
  <SvelteComponent />
</main>

<footer>
  <SongPlayer />
</footer>

<style>
  .playlist-section {
    width: clamp(400px, 100%, 800px);
  }

  header {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    flex: 0 1 auto;
  }

  main {
    flex: 1 1 auto;
    overflow: hidden;
    justify-content: flex-start;
  }
  footer {
    width: 100%;
  }
</style>
