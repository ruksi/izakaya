import {browser} from "$app/environment";
import {api} from "$lib/backend";
import {QueryClient} from "@tanstack/svelte-query";

export const load = async ({fetch}) => {
    // To pre-cache data on the SvelteKit backend, we construct
    // the query client here on the root layout and pass it to the pages.
    const queryClient = new QueryClient({
        defaultOptions: {
            queries: {
                enabled: browser,
                // TanStack Query marks prefetched queries as "stale" by default,
                // and any cached data will be refetch on mount anyway, so don't
                // do that, please. Only consider doing that after the following (ms).
                staleTime: 10_000,
            },
        },
    });

    await queryClient.prefetchQuery({
        queryKey: ["verify"],
        queryFn: api(fetch).getVerify,
    });

    return {queryClient};
};
