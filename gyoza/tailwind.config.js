/** @type {import("tailwindcss").Config} */
export default {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {},
    corePlugins: {
        container: false,
    },
    plugins: [
        require("@tailwindcss/typography"),
        // eslint-disable-next-line @typescript-eslint/no-var-requires
        require("@tailwindcss/forms")({strategy: "base"}),
        require("tailwind-scrollbar"),
    ],
};
