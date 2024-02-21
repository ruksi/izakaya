import {getBackendUrl} from "@/urls";

const backend = getBackendUrl();

export async function query(uri: string) {
    let response = await fetch(`${backend}${uri}`, {credentials: "include"});
    return await handleResponse(response);
}

export async function mutation([uri, method]: any[], {arg}: {arg: any}) {
    const headers: Record<string, string> = {};
    const token = getCookie("Tatami-CSRF");
    if (token) {
        headers["CSRF-Token"] = token;
    }

    let body;
    if (arg) {
        body = JSON.stringify(arg);
        headers["Content-Type"] = "application/json";
    }

    let response = await fetch(`${backend}${uri}`, {
        credentials: "include",
        method,
        headers,
        body,
    });
    return await handleResponse(response);
}

function getCookie(name: string): string | null {
    const valueStartsAt = name.length + 1;
    return (
        document.cookie
            .split(";")
            .map((c) => c.trim())
            .filter((cookie) => {
                return cookie.substring(0, valueStartsAt) === `${name}=`;
            })
            .map((cookie) => {
                return decodeURIComponent(cookie.substring(valueStartsAt));
            })[0] || null
    );
}

async function handleResponse(response: Response): Promise<any> {
    let data = {};

    // invalid JSON parse _could_ be fine too if response has an empty body
    try {
        data = await response.json();
    } catch (e) {
        console.debug(e);
    }

    // turn 4xx and 5xx into errors,
    // then you can have valid data in `data` and error details in `error.data`
    if (!response.ok) {
        const error = new Error(response.statusText);
        // @ts-ignore
        error.data = data;
        // @ts-ignore
        error.status = response.status;
        throw error;
    }

    return data;
}
