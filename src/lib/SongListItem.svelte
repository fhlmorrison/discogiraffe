<script lang="ts">
  import type { OpenFileEntry } from "../store/files.svelte";
  interface Props {
    song: OpenFileEntry;
    index: number;
    onSelect: (index: number) => void;
  }

  let { song, index, onSelect }: Props = $props();

  const select = () => {
    onSelect(index);
  };
</script>

<div>
  <li onclick={select}>
    <div class="li-num">
      {index + 1} |
    </div>
    {song.name}
  </li>
</div>

<style>
  li {
    display: flex;
    align-items: left;
    text-align: left;
    width: 100%;
    cursor: default;
  }
  li:hover {
    background-color: #aaa;
  }
  .li-num {
    text-align: right;
    width: 5ch;
    margin-right: 1ch;
  }
</style>
