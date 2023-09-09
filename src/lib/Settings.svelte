<script lang="ts">
  import SettingEntry from "./SettingEntry.svelte";
  import { settings, settingList } from "../store/settings";
  import FaCog from "svelte-icons/fa/FaCog.svelte";

  let values = { ...$settings };

  let modalOpen = false;
  const closeModal = () => {
    modalOpen = false;
    values = { ...$settings };
  };

  const saveChanges = () => {
    console.log("save");
    settings.set(values);
  };
</script>

<button
  class="settings"
  on:click={() => {
    modalOpen = true;
  }}
>
  <FaCog />
</button>
{#if modalOpen}
  <div class="modal" on:click={closeModal} />
  <div class="modal-content">
    <h1>Settings</h1>
    {#each settingList as setting}
      <SettingEntry entry={setting} bind:value={values[setting.key]} />
    {/each}
    <button class="close" on:click={closeModal}>Close</button>
    <button class="save" on:click={saveChanges}>Save</button>
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
    background-color: #2f2f2f;
    /* overflow-y: scroll; */
  }
</style>
