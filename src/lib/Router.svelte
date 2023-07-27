<script lang="ts">
    import PlaylistDisplay from "./PlaylistDisplay.svelte";
    import GetPl from "./GetPlaylist.svelte";
    import MetaDataReader from "./SongReader.svelte";

    type Route = {
        name: string;
        path: string;
        component: any;
    };

    let currentRoute: Route;

    let routes: Route[] = [
        {
            name: "Get Playlist",
            path: "/",
            component: GetPl,
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
