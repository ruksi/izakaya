export function backendUrl(): string {
    let url = import.meta.env.VITE_BACKEND_URL ?? "";
    url = url.trim();
    if (url.endsWith("/")) {
        url = url.slice(0, -1);
    }
    if (url === "") {
        throw new Error("VITE_BACKEND_URL is not set");
    }
    return url;
}

export function getCookie(name: string): string | null {
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
