<script lang="ts">
  import { settings } from "./../store/settings";
  import type { SettingEntry } from "../store/settings";
  import { onMount } from "svelte";
  import { openFolder } from "./loadAssets";
  interface Props {
    entry: SettingEntry;
    value: any;
  }

  let { entry, value = $bindable() }: Props = $props();
  const getValue = (e: SettingEntry) => {
    value = $settings[e.key] ?? e.default;
  };
  //   $: getValue(entry);
  onMount(() => getValue(entry));
  const chooseFolder = async () => {
    value = await openFolder();
  };
</script>

<div>
  <label for={entry.key}>{entry.name}</label>
  {#if entry.type === "boolean"}
    <input type="checkbox" id={entry.key} bind:checked={value} />
  {:else if entry.type === "dropdown"}
    <select id={entry.key} bind:value>
      {#each entry.options as option}
        <option value={option.key}>{option.name}</option>
      {/each}
    </select>
  {:else if entry.type === "folderPath"}
    <button onclick={chooseFolder}>Choose Folder</button>
    <span>{value}</span>
  {/if}
</div>

<style>
  /* your styles go here */
</style>
