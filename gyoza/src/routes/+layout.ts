import {browser} from "$app/environment";
import {api} from "$lib/backend";
import {redirect} from "@sveltejs/kit";
import {QueryClient} from "@tanstack/svelte-query";

export const load = async ({fetch, route}) => {
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

    // this both checks and pre-caches if the user is authenticated,
    // note this does not refresh on log in or log out, only on a page load
    // a `load` function the root layout
    const verify = await queryClient.fetchQuery({
        queryKey: ["verify"],
        queryFn: api(fetch).getVerify,
    });
    if (verify?.is_authenticated == false) {
        if (route.id && (route.id as string).startsWith("/(authenticated)/")) {
            redirect(307, "/");
        }
    }
    if (verify?.is_authenticated == true) {
        if (route.id && (route.id as string).startsWith("/(anonymous)/")) {
            redirect(307, "/");
        }
    }

    return {queryClient};
};
