import {backendUrl, getCookie} from "$lib/utils";
import {createMutation, createQuery, type QueryClient} from "@tanstack/svelte-query";

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

// In server `load`, the SvelteKit `fetch` is not yet injected, so
// we need to pass it as an argument to the API function.
export function api(_fetch = fetch) {
    return {
        getVerify: async (): Promise<Verify> => {
            const data = await handleFetch({
                url: `${baseUrl}/verify`,
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
                url: `${baseUrl}/log-in`,
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
                url: `${baseUrl}/log-out`,
                options: {
                    method: "POST",
                },
                _fetch,
            });
            return data as Status;
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

    _options.headers = (_options.headers || {}) as Record<string, string>;
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

    const response = await _fetch(url, _options);

    let payload = {};
    try {
        payload = await response.json();
    } catch (e) {
        // invalid JSON parse _could_ be fine too if response has an empty body
        console.debug(e);
    }

    // turn 4xx and 5xx into errors
    if (!response.ok) {
        const error = new Error(response.statusText);
        // @ts-expect-error TS2339
        error.data = payload;
        // @ts-expect-error TS2339
        error.status = response.status;
        throw error;
    }

    return payload;
}
