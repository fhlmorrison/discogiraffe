<script lang="ts">
  import { currentTab, routes } from "../store/tabs";
  currentTab.set(routes[0]);
</script>

<div>
  {#each routes as route}
    <button
      class={route === $currentTab ? "active" : ""}
      onclick={() => {
        currentTab.set(route);
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
