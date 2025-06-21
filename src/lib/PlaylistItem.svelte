<script lang="ts">
  import FaPlay from "svelte-icons/fa/FaPlay.svelte";
  import type { dbSong } from "../store/playlist.svelte";

  interface Props {
    item: dbSong;
    selected?: boolean;
  }

  let { item, selected = $bindable(false) }: Props = $props();

  const fmtMSS = (s: number) => {
    return (s - (s %= 60)) / 60 + (9 < s ? ":" : ":0") + s;
  };

  // if downloaded, do not bind
</script>

<div class="item">
  <div class="thumbnail">
    <img src={item.thumbnail} alt={item.id} height="90px" loading="lazy" />
    <a class="link" target="_blank" href={item.url}>
      <FaPlay />
    </a>
    <div class="duration">{fmtMSS(item.duration)}</div>
  </div>
  <div class="metadata">
    <div class="title">
      {item.title}
    </div>
    <div class="channel">
      {item.channel}
    </div>
  </div>
  <div class="selector">
    {#if item.downloaded}
      <input class="checkbox" type="checkbox" checked disabled />
    {:else}
      <input class="checkbox" type="checkbox" bind:checked={selected} />
    {/if}
  </div>
</div>

<!-- <iframe
    width="560"
    height="315"
    src="https://www.youtube.com/embed/{item.id}"
    title="YouTube video player"
    frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    allowfullscreen
/> -->

<style>
  .item {
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: left;
    align-items: center;
    border-radius: 15px;
    padding: 10px;
  }

  .item:hover {
    background-color: #ddd;
  }

  .item:hover .link {
    opacity: 1;
  }

  .thumbnail {
    border-radius: 10px;
    position: relative;
    height: 90px;
  }

  .thumbnail img {
    aspect-ratio: 16 / 9;
    border-radius: 10px;
    padding: 0;
  }

  .link {
    position: absolute;
    top: 50%;
    left: 50%;
    opacity: 0;
    color: rgb(255, 255, 255, 0.85);
    height: 40%;
    padding: 2px;
    border-radius: 2px;
    transform: translate(-50%, -50%);
  }

  .duration {
    position: absolute;
    bottom: 5px;
    right: 5px;
    background-color: #00000080;
    color: #fff;
    padding: 0.1em 0.5em;
    border-radius: 0.5em;
  }

  .metadata {
    display: flex;
    flex-direction: column;
    align-self: start;
    margin-left: 1em;
    text-align: left;
  }
  .title {
    font-size: 1em;
    font-weight: bold;
  }
  .channel {
    font-size: 0.9em;
  }
  .selector {
    margin-left: auto;
  }
  .checkbox {
    width: 1.5em;
    height: 1.5em;
    border-radius: 0.5em;
  }

  .checkbox[disabled] {
    filter: invert(25%);
  }
</style>
