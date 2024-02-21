import {backendUrl} from "$lib/urls";
import type {HandleFetch} from "@sveltejs/kit";

export const handleFetch: HandleFetch = async ({event, request, fetch}) => {
    // TODO: is it bad that this can cause some cookies to be send to ether? 😅
    if (request.url.startsWith(backendUrl)) {
        // pass user cookies from SvelteKit backend to the API backend
        const cookies = event.request.headers.get("Cookie");
        if (cookies) {
            request.headers.set("Cookie", cookies);
        }
    }
    return fetch(request);
};
