export default function getCookie(name: string): string | null {
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
