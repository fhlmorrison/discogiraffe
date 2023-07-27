<script lang="ts">
    import GetPlaylist from "./GetPlaylist.svelte";
    import PlaylistDisplay from "./PlaylistDisplay.svelte";
    import MetaDataReader from "./SongReader.svelte";
    import PlaylistLibrary from "./PlaylistLibrary.svelte";

    type Route = {
        name: string;
        path: string;
        component: any;
    };

    let routes: Route[] = [
        {
            name: "Playlist Library",
            path: "/",
            component: PlaylistLibrary,
        },
        {
            name: "Get Playlist",
            path: "/getplaylist",
            component: GetPlaylist,
        },
        {
            name: "Playlist Display",
            path: "/playlist",
            component: PlaylistDisplay,
        },
        {
            name: "Song Reader",
            path: "/songreader",
            component: MetaDataReader,
        },
    ];

    let currentRoute: Route = routes[0];
</script>

<div>
    Tabs

    {#each routes as route}
        <button
            on:click={() => {
                currentRoute = route;
            }}>{route.name}</button
        >
    {/each}
</div>
<div>
    <svelte:component this={currentRoute?.component} />
</div>

<style>
    /* your styles go here */
</style>
