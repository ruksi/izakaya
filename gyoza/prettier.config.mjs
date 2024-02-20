/** @type {import("prettier").Config} */
const config = {
    bracketSpacing: false,
    htmlWhitespaceSensitivity: "strict",
    tabWidth: 4,
    printWidth: 100,
    trailingComma: "es5",
    plugins: ["prettier-plugin-svelte"],
    overrides: [{files: "*.svelte", options: {parser: "svelte"}}],
};

export default config;
