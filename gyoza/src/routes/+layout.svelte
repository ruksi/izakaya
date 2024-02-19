<script lang="ts">
    import MainHeader from "$lib/MainHeader.svelte";
    import {QueryClientProvider} from "@tanstack/svelte-query";
    import type {LayoutData} from "./$types";

    export let data: LayoutData;
</script>

<QueryClientProvider client={data.queryClient}>
    <MainHeader />
    <slot />
</QueryClientProvider>
