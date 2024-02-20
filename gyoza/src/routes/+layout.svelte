<script lang="ts">
    import "../app.css";

    import MainHeader from "$lib/MainHeader.svelte";
    import {QueryClientProvider} from "@tanstack/svelte-query";
    import type {LayoutData} from "./$types";

    export let data: LayoutData;
</script>

<svelte:head>
    <!--<title>{data.title}</title>-->
    <title>Izakaya</title>
</svelte:head>

<QueryClientProvider client={data.queryClient}>
    <MainHeader />
    <slot />
</QueryClientProvider>
