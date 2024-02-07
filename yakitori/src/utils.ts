export function yakitoriUrl(): string {
    let url = process.env.NEXT_PUBLIC_YAKITORI_URL ?? "";
    url = url.trim();
    if (url.endsWith("/")) {
        url = url.slice(0, -1);
    }
    if (url === "") {
        throw new Error("NEXT_PUBLIC_YAKITORI_URL is not set");
    }
    return url;
}

export function backendUrl(): string {
    let url = process.env.NEXT_PUBLIC_BACKEND_URL ?? "";
    url = url.trim();
    if (url.endsWith("/")) {
        url = url.slice(0, -1);
    }
    if (url === "") {
        throw new Error("NEXT_PUBLIC_BACKEND_URL is not set");
    }
    return url;
}
