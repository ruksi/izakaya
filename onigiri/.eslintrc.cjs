// noinspection JSUnresolvedReference
module.exports = {
    parser: "@typescript-eslint/parser",
    parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
    },
    settings: {
        react: {version: "18.2"},
    },
    env: {
        browser: true,
        es2020: true,
        node: false,
    },
    extends: [
        "eslint:recommended",
        "plugin:react/recommended",
        "plugin:react/jsx-runtime",
        "plugin:react-hooks/recommended",
    ],
    plugins: ["react-refresh", "@typescript-eslint", "@emotion/eslint-plugin"],
    rules: {
        "react-refresh/only-export-components": "warn",
        "@emotion/syntax-preference": [2, "object"],
    },
};
