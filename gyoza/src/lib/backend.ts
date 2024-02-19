import {backendUrl, getCookie} from "$lib/utils";
import {createMutation, createQuery} from "@tanstack/svelte-query";

export type Status = {status: string};
export type Verify = {is_authenticated: boolean};

const baseUrl = backendUrl();
const safeMethods = ["GET", "HEAD", "OPTIONS", "TRACE"];

export function verifyQuery() {
    return createQuery({
        queryKey: ["verify"],
        queryFn: api().getVerify,
    });
}

export function logInMutation(client) {
    return createMutation({
        mutationFn: api().postLogIn,
        onMutate: async () => {
            await client.cancelQueries({queryKey: ["verify"]});
            return null;
        },
        onSettled: () => {
            client.invalidateQueries({queryKey: ["verify"]});
        },
    });
}

export function logOutMutation(client) {
    return createMutation({
        mutationFn: api().postLogOut,
        onMutate: async () => {
            await client.cancelQueries({queryKey: ["verify"]});
            return null;
        },
        onSuccess: () => {
            client.invalidateQueries();
        },
    });
}

// In server `load`, the SvelteKit `fetch` is not yet injected, so
// we need to pass it as an argument to the API function.
export function api(_fetch = fetch) {
    return {
        getVerify: async (): Promise<Verify> => {
            const response = await handleFetch({
                url: `${baseUrl}/verify`,
                _fetch,
            });
            return (await response.json()) as Verify;
        },
        postLogIn: async ({username_or_email, password}): Promise<Status> => {
            const response = await handleFetch({
                url: `${baseUrl}/log-in`,
                options: {
                    method: "POST",
                    body: JSON.stringify({username_or_email, password}),
                },
                _fetch,
            });
            return (await response.json()) as Status;
        },
        postLogOut: async (): Promise<Status> => {
            const response = await handleFetch({
                url: `${baseUrl}/log-out`,
                options: {
                    method: "POST",
                },
                _fetch,
            });
            return (await response.json()) as Status;
        },
    };
}

type HandleFetchParameters = {
    url: string;
    options?: RequestInit;
    _fetch: typeof fetch;
};

async function handleFetch({url, options, _fetch}: HandleFetchParameters) {
    const _options: RequestInit = options || {};
    if (url.startsWith(baseUrl)) {
        _options["credentials"] = "include";
    }

    _options.headers = _options.headers || {};
    if (_options.body) {
        _options.headers = {
            "Content-Type": "application/json",
        };
    }
    if (_options.method && !safeMethods.includes(_options.method as string)) {
        const token = getCookie("Tatami-CSRF");
        if (token) {
            _options.headers["CSRF-Token"] = token;
        }
    }

    return _fetch(url, _options);
}
