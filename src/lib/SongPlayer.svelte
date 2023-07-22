<script lang="ts">
    import FaPlayCircle from "svelte-icons/fa/FaPlayCircle.svelte";
    import FaPauseCircle from "svelte-icons/fa/FaPauseCircle.svelte";
    // import { currentTime } from "./../store/player";
    let time = 0;
    export let url: string = "";
    let audioPlayer: HTMLAudioElement;
    // audioPlayer.controls = false;
    // $: audioPlayer.src = url;
    // let currentTime = 0;
    let duration = 0;
    let paused = true;

    const cleanTime = (seconds) => {
        const hours = Math.floor(seconds / 3600);
        const minutes = Math.floor((seconds / 60) % 60);
        const secs = Math.floor(seconds % 60);
        return `${minutes}:${secs < 10 ? "0" : ""}${secs}`;
    };

    const jumpto = (e) => {
        const rect = e.target.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const percent = x / rect.width;
        audioPlayer.currentTime = percent * duration;
    };
</script>

<div class="song-player">
    <div class="controls">
        {#if paused}
            <div class="button" on:click={() => audioPlayer.play()}>
                <FaPlayCircle />
            </div>
        {:else}
            <div class="button" on:click={() => audioPlayer.pause()}>
                <FaPauseCircle />
            </div>
        {/if}
    </div>
    <div class="progress" on:click={jumpto}>
        <div
            class="progress-bar"
            style={`--progress: ${(time / duration) * 100}%`}
        />
    </div>
    {cleanTime(time)} / {cleanTime(duration)}
    <audio
        src={url}
        bind:this={audioPlayer}
        bind:paused
        bind:currentTime={time}
        bind:duration
    />
</div>

<style>
    * {
        --progress-bar-radius: 5px;
    }

    .progress {
        width: 100%;
        height: 5px;
        background-color: rgba(200, 200, 200, 50);
        position: relative;
        border-radius: var(--progress-bar-radius);
        overflow: hidden;
    }
    .progress-bar {
        height: 100%;
        width: var(--progress);
        /* transform: translateX(calc(-100% + var(--progress))); */
        position: absolute;
        /* left: 0; */
        z-index: 1;
        width: var(--progress);
        background-color: #400;
        border-radius: var(--progress-bar-radius);
    }
    audio {
        display: none !important;
    }
    .controls {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        grid-gap: 1em;
    }
    .button {
        background-color: transparent;
        border: none;
        cursor: pointer;
        border-radius: 50%;
        padding: 0;
        height: 60px;
        width: 60px;
        align-self: center;
        justify-self: center;
        grid-area: 1 / 2 / 3 / 3;
    }
</style>
