import handleFetch from "$lib/handleFetch";
import {backendUrl} from "$lib/urls";
import {createMutation, createQuery, type QueryClient} from "@tanstack/svelte-query";

export function verifyQuery() {
    return createQuery({
        queryFn: api().getVerify,
        queryKey: ["verify"],
    });
}

export function logInMutation(client: QueryClient) {
    return createMutation({
        mutationFn: api().postLogIn,
        onMutate: async () => {
            await client.cancelQueries({queryKey: ["verify"]});
            return null;
        },
        onSettled: async () => {
            await client.invalidateQueries({queryKey: ["verify"]});
        },
    });
}

export function logOutMutation(client: QueryClient) {
    return createMutation({
        mutationFn: api().postLogOut,
        onMutate: async () => {
            await client.cancelQueries({queryKey: ["verify"]});
            return null;
        },
        onSuccess: async () => {
            await client.invalidateQueries();
        },
    });
}

export type Status = {status: string};
export type Verify = {is_authenticated: boolean};

// In server `load`, the SvelteKit `fetch` is not yet injected, so
// we sometimes need to pass it as an argument to the API function.
export function api(_fetch = fetch) {
    return {
        getVerify: async (): Promise<Verify> => {
            const data = await handleFetch({
                url: `${backendUrl}/verify`,
                _fetch,
            });
            return data as Verify;
        },
        postLogIn: async ({
            username_or_email,
            password,
        }: {
            username_or_email: string;
            password: string;
        }): Promise<Status> => {
            const data = await handleFetch({
                url: `${backendUrl}/log-in`,
                options: {
                    method: "POST",
                    body: JSON.stringify({username_or_email, password}),
                },
                _fetch,
            });
            return data as Status;
        },
        postLogOut: async (): Promise<Status> => {
            const data = await handleFetch({
                url: `${backendUrl}/log-out`,
                options: {
                    method: "POST",
                },
                _fetch,
            });
            return data as Status;
        },
    };
}
