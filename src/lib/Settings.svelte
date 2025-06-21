<script lang="ts">
  import SettingEntry from "./SettingEntry.svelte";
  import { settingsStore, settingList } from "../store/settings.svelte";
  import FaCog from "svelte-icons/fa/FaCog.svelte";
  import { onMount } from "svelte";

  onMount(() => {
    settingsStore.load();
  });

  let modalOpen = $state(false);
  const closeModal = () => {
    modalOpen = false;
  };

  const saveChanges = () => {
    settingsStore.save();
    console.log("saved settings");
  };
</script>

<button
  class="settings"
  onclick={() => {
    modalOpen = true;
  }}
>
  <FaCog />
</button>
{#if modalOpen}
  <div class="modal" onclick={closeModal}></div>
  <div class="modal-content">
    <h1>Settings</h1>
    {#each settingList as setting}
      <SettingEntry
        entry={setting}
        bind:value={settingsStore.settings[setting.key]}
      />
    {/each}
    <button class="close" onclick={closeModal}>Close</button>
    <button class="save" onclick={saveChanges}>Save</button>
  </div>
{/if}

<style>
  .settings {
    width: 40px;
    height: 40px;
    cursor: pointer;
    border-radius: 50%;
    right: 8px;
    top: 8px;
    padding: 5px;
    position: fixed;
  }
  .modal {
    z-index: 2;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    opacity: 0.5;
    background-color: black;
  }
  .modal-content {
    z-index: 3;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    border-radius: 10px;
    padding: 20px;
    width: 60%;
    height: 60%;
    background-color: var(--background-color);
    /* overflow-y: scroll; */
  }
</style>
