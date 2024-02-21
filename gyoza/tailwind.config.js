/** @type {import('tailwindcss').Config} */
export default {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
        screens: {},
        extend: {
            colors: {
                amber: {990: "rgb(40, 20, 10)"},
                green: {990: "rgb(5, 30, 10)"},
                red: {990: "rgb(40, 10, 10)"},
                yellow: {990: "rgb(30, 10, 10)"},
                zinc: {
                    750: "rgb(45, 45, 48)",
                    850: "rgb(32, 32, 35)",
                },
            },
        },
    },
    plugins: [
        require("@tailwindcss/typography"),
        // eslint-disable-next-line @typescript-eslint/no-var-requires
        require("@tailwindcss/forms")({strategy: "base"}),
        require("tailwind-scrollbar"),
    ],
};
