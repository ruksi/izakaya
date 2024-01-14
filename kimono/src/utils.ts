export function tatamiUrl(): string {
    let url = import.meta.env.VITE_TATAMI_URL ?? "";
    url = url.trim();
    if (url.endsWith("/")) {
        url = url.slice(0, -1);
    }
    if (url === "") {
        throw new Error("VITE_TATAMI_URL is not set");
    }
    return url;
}
