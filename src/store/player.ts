import { writable } from "svelte/store";

const audio = new Audio();

const { subscribe, set, update } = writable<HTMLAudioElement>(audio);

export const player = {
    subscribe,
    set,
    update,
};

export const currentTime = writable(0);

// player.play()
// player.pause()
// $player.currentTime
// $player.duration
// $player.volume
// $player.muted
// $player.playbackRate ??
// $player.playing
// $player.ended
// $player.loop
// $player.src
// $player.currentSrc