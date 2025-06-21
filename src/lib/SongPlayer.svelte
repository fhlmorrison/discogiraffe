<script lang="ts">
  // ignore this
  import FaStepBackward from "svelte-icons/fa/FaStepBackward.svelte";
  import FaStepForward from "svelte-icons/fa/FaStepForward.svelte";
  import FaPlayCircle from "svelte-icons/fa/FaPlayCircle.svelte";
  import FaPauseCircle from "svelte-icons/fa/FaPauseCircle.svelte";
  import FaVolumeUp from "svelte-icons/fa/FaVolumeUp.svelte";
  import { settings } from "../store/settings";
  import { openFiles, selectedIndex } from "../store/files";
  // import { currentTime } from "./../store/player";
  let time = $state(0);
  let audioPlayer: HTMLAudioElement | undefined = $state();
  // audioPlayer.controls = false;
  // $: audioPlayer.src = url;
  // let currentTime = 0;
  let duration = $state(0);
  let volume = $state(0.75);
  let url = $derived($openFiles[$selectedIndex]?.url ?? "");
  let paused = $state(true);
  let progressElement: HTMLDivElement | undefined = $state();

  const cleanTime = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds / 60) % 60);
    const secs = Math.floor(seconds % 60);
    return `${hours ? hours + ":" : ""} ${minutes}:${
      secs < 10 ? "0" : ""
    }${secs}`;
  };

  const jumpto = (e: MouseEvent) => {
    if (!progressElement || !audioPlayer) return;
    const rect = progressElement.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const percent = x / rect.width;
    audioPlayer.currentTime = percent * duration;
  };

  const prev = () => {
    openFiles.prev();
    if ($settings["autoPlay"]) {
      setTimeout(() => {
        audioPlayer?.play();
      }, 100);
    }
  };

  const next = () => {
    openFiles.next();
    if ($settings["autoPlay"]) {
      setTimeout(() => {
        audioPlayer?.play();
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
      <div class="button skip" onclick={prev}>
        <FaStepBackward />
      </div>
      {#if paused}
        <div class="button play" onclick={() => audioPlayer?.play()}>
          <FaPlayCircle />
        </div>
      {:else}
        <div class="button play" onclick={() => audioPlayer?.pause()}>
          <FaPauseCircle />
        </div>
      {/if}
      <div class="button skip" onclick={next}>
        <FaStepForward />
      </div>
    </div>
  </div>
  <div class="play-bar">
    <div class="duration">
      {cleanTime(time)} / {cleanTime(duration)}
    </div>
    <div class="progress" onclick={jumpto} bind:this={progressElement}>
      <div
        class="progress-bar"
        style={`--progress: ${(time / duration) * 100}%`}
      ></div>
    </div>
    <div class="volume">
      <div class="volume-icon">
        <FaVolumeUp />
      </div>
      <input
        type="range"
        min="0"
        max="1"
        step="0.01"
        bind:value={volume}
        oninput={() => {
          if (audioPlayer) {
            audioPlayer.volume = volume;
          }
        }}
      />
    </div>
  </div>
  <audio
    src={url}
    bind:this={audioPlayer}
    bind:paused
    bind:currentTime={time}
    bind:duration
    onended={songEnded}
  ></audio>
</div>

<style>
  * {
    --progress-bar-radius: 5px;
  }

  .progress {
    width: clamp(400px, 33%, 40%);
    height: 5px;
    background-color: rgba(200, 200, 200, 50);
    position: relative;
    border-radius: var(--progress-bar-radius);
    overflow: hidden;
    /* margin: auto; */
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

  .duration {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-end;
    width: 30%;
  }

  .volume {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    width: 30%;
  }
  /* .volume > * {
    outline: black solid 1px;
  } */
  .volume-icon {
    height: 20px;
    align-content: center;
    color: var(--button-color);
    /* object-fit: contain; */
  }

  input[type="range"] {
    /* -webkit-appearance: none; */
    margin: 0;
    padding: 0;
    height: 20px;
    border: none;
    background-color: transparent;
    cursor: pointer;
    box-shadow: none;
  }

  .play-bar {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    gap: 10px;
  }
  /* .play-bar > * {
    outline: black solid 1px;
  } */
</style>
