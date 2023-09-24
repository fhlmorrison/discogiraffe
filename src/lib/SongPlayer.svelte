<script lang="ts">
  // ignore this
  import FaStepBackward from "svelte-icons/fa/FaStepBackward.svelte";
  import FaStepForward from "svelte-icons/fa/FaStepForward.svelte";
  import FaPlayCircle from "svelte-icons/fa/FaPlayCircle.svelte";
  import FaPauseCircle from "svelte-icons/fa/FaPauseCircle.svelte";
  import { settings } from "../store/settings";
  import { openFiles, selectedIndex } from "../store/files";
  // import { currentTime } from "./../store/player";
  let time = 0;
  let audioPlayer: HTMLAudioElement;
  // audioPlayer.controls = false;
  // $: audioPlayer.src = url;
  // let currentTime = 0;
  let duration = 0;
  let paused = true;

  let progressElement: HTMLDivElement;

  $: url = $openFiles[$selectedIndex]?.url ?? "";

  const cleanTime = (seconds) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds / 60) % 60);
    const secs = Math.floor(seconds % 60);
    return `${hours ? hours + ":" : ""} ${minutes}:${
      secs < 10 ? "0" : ""
    }${secs}`;
  };

  const jumpto = (e) => {
    const rect = progressElement.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const percent = x / rect.width;
    audioPlayer.currentTime = percent * duration;
  };
  $: url && (paused = true);

  const prev = () => {
    openFiles.prev();
    if ($settings["autoPlay"]) {
      setTimeout(() => {
        audioPlayer.play();
      }, 100);
    }
  };

  const next = () => {
    openFiles.next();
    if ($settings["autoPlay"]) {
      setTimeout(() => {
        audioPlayer.play();
      }, 100);
    }
  };

  const songEnded = () => {
    if ($settings["autoPlay"]) {
      next();
    }
  };
</script>

<div class="song-player">
  <div class="controls-row">
    <div class="controls">
      <div class="button skip" on:click={prev}>
        <FaStepBackward />
      </div>
      {#if paused}
        <div class="button play" on:click={() => audioPlayer.play()}>
          <FaPlayCircle />
        </div>
      {:else}
        <div class="button play" on:click={() => audioPlayer.pause()}>
          <FaPauseCircle />
        </div>
      {/if}
      <div class="button skip" on:click={next}>
        <FaStepForward />
      </div>
    </div>
  </div>
  <div class="row">
    <div class="progress" on:click={jumpto} bind:this={progressElement}>
      <div
        class="progress-bar"
        style={`--progress: ${(time / duration) * 100}%`}
      />
    </div>
    <div class="duration">
      {cleanTime(time)} / {cleanTime(duration)}
    </div>
  </div>
  <audio
    src={url}
    bind:this={audioPlayer}
    bind:paused
    bind:currentTime={time}
    bind:duration
    on:ended={songEnded}
  />
</div>

<style>
  * {
    --progress-bar-radius: 5px;
  }

  .progress {
    width: clamp(500px, 33%, 100%);
    height: 5px;
    background-color: rgba(200, 200, 200, 50);
    position: relative;
    border-radius: var(--progress-bar-radius);
    overflow: hidden;
    margin: auto;
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
  .controls-row {
    display: flex;
    flex-direction: row;
    justify-content: center;
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
  }
  .skip {
    height: 40px;
    width: 40px;
  }
</style>
