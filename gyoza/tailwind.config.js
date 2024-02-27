/** @type {import("tailwindcss").Config} */
export default {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {},
    corePlugins: {
        container: false,
    },
    daisyui: {
        logs: false,
        themes: [{
            izakaya: {
                "--rounded-box": "0.25rem",
                "--rounded-btn": "0.25rem",
                "--rounded-badge": "1.9rem",

                "base-100": "#18181b",        // zinc-900
                "base-content": "#d4d4d4",    // neutral-300
                "neutral": "#44403c",         // stone-700

                "primary": "#9333ea",           // purple-600
                "primary-content": "#fafafa",   // neutral-50
                "secondary": "#94a3b8",         // slate-400
                "secondary-content": "#fafafa", // neutral-50
                "accent": "#db2777",            // pink-600
                "accent-content": "#fafafa",    // neutral-50

                "info": "#0284c7",            // sky-600
                "info-content": "#f0f9ff",    // sky-50
                "success": "#16a34a",         // green-600
                "success-content": "#f0fdf4", // green-50
                "warning": "#ca8a04",         // yellow-600
                "warning-content": "#fefce8", // yellow-50
                "error": "#dc2626",           // red-600
                "error-content": "#fef2f2",   // red-50
            },
        }],
    },
    plugins: [
        require("@tailwindcss/typography"),
        require("tailwind-scrollbar"),
        require("daisyui"),
    ]
};
