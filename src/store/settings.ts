import { writable } from "svelte/store";

type settings = {
    theme?: "light" | "dark";
};

export const settings = writable<settings>({});

const loadSettings = () => { };
const saveSettings = () => { };