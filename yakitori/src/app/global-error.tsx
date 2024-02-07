"use client";

export default function GlobalError({
    _error,
    reset,
}: {
    _error: Error & {digest?: string};
    reset: () => void;
}) {
    return (
        <html lang="en">
            <body>
                <h1>Sorry, something went wrong</h1>
                <button onClick={() => reset()}>Try again</button>
            </body>
        </html>
    );
}
