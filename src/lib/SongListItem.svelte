<script lang="ts">
    import type { OpenFileEntry } from "../store/files";
    import { createEventDispatcher } from "svelte";
    export let song: OpenFileEntry;
    export let index: number;

    const dispatch = createEventDispatcher();

    const select = () => {
        dispatch("select", song);
    };
</script>

<div>
    <li on:click={select}>
        <div class="li-num">
            {index + 1}
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
        width: 2em;
        margin-right: 1em;
    }
</style>
