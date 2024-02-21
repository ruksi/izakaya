import FetchError from "$lib/FetchError";
import getCookie from "$lib/getCookie";
import {backend} from "$lib/urls";

const safeMethods = ["GET", "HEAD", "OPTIONS", "TRACE"];

type HandleFetchParameters = {
    url: string;
    options?: RequestInit;
    _fetch: typeof fetch;
};

export default async function handleFetch({url, options, _fetch}: HandleFetchParameters) {
    const _options: RequestInit = options || {};
    if (url.startsWith(backend)) {
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

    let data = {};
    try {
        data = await response.json();
    } catch (e) {
        // invalid JSON parse _could_ be fine too if response has an empty body
        console.debug(e);
    }

    // turn non-2xx status codes into errors
    if (!response.ok) {
        throw new FetchError(response.status, data);
    }
    return data;
}
