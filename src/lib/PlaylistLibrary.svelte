<script>
    import { loadPlaylists, playlistLibrary } from "../store/playlist";
    import AddPlaylist from "./AddPlaylist.svelte";

    playlistLibrary.set([
        {
            title: "Test Playlist",
            description: "This is a test playlist",
            thumbnail: "https://i.ytimg.com/vi/5qap5aO4i9A/hqdefault.jpg",
            url: "",
            downloaded: false,
        },
    ]);

    let adding = false;

    const addPlaylist = () => {
        adding = true;
    };
    const closeModal = () => {
        adding = false;
    };
    const selectPlaylist = (playlist) => {
        console.log(playlist);
    };
    const loadList = () => {
        loadPlaylists();
        console.log("loaded");
    };
</script>

<div class="grid">
    <div class="card add" on:click={addPlaylist}>+</div>
    {#if adding}
        <AddPlaylist
            on:close={closeModal}
            on:load={() => {
                closeModal();
                loadList();
            }}
        />
    {/if}
    {#each $playlistLibrary as playlist}
        <div class="card playlist" on:click={() => selectPlaylist(playlist)}>
            <img src={playlist.thumbnail} alt={playlist.title} />
            <div class="playlist-title">{playlist.title}</div>
            <div class="playlist-description">{playlist.description}</div>
        </div>
    {/each}
</div>

<style>
    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    }

    .card {
        border: 1px solid black;
        border-radius: 5px;
        margin: 5px;
        padding: 5px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        cursor: pointer;
    }

    .add {
        font-size: 5em;
    }

    .card:hover {
        opacity: 0.8;
        border-color: #aaa;
    }

    .card > img {
        width: 100%;
        height: auto;
        aspect-ratio: 1/1;
        object-fit: cover;
    }
</style>
