import {defineConfig, loadEnv} from "vite";
import react from "@vitejs/plugin-react";

// noinspection JSUnusedGlobalSymbols
export default defineConfig(({mode}) => {
    // eslint-disable-next-line no-undef
    const env = loadEnv(mode, process.cwd(), "");
    const port = env.PORT || 5173;
    return {
        server: {port},
        plugins: [react()],
    };
});
