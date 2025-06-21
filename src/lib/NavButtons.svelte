<script lang="ts">
  import { tabStore, routes } from "../store/tabs.svelte";
  $inspect(tabStore.current);
</script>

<div>
  {#each routes as route}
    <button
      class={route.name === tabStore.current.name ? "active" : ""}
      onclick={() => {
        tabStore.current = route;
      }}>{route.name}</button
    >
  {/each}
</div>

<style>
  /* your styles go here */
  .active {
    background-color: #4caf50;
    color: white;
  }
</style>
