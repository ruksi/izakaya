// TODO: not sure we really need this yet
// import type {HandleFetch} from "@sveltejs/kit";
//
// export const handleFetch: HandleFetch = async ({event, request, fetch}) => {
//     if (request.url.startsWith("http://localhost:8080")) {
//         // pass user cookies from SvelteKit backend to the API backend
//         const cookies = event.request.headers.get("Cookie");
//         if (cookies) {
//             request.headers.set("Cookie", cookies);
//         }
//     }
//     return fetch(request);
// };
