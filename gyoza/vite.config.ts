import {sveltekit} from "@sveltejs/kit/vite";
import {defineConfig, loadEnv} from "vite";

// @ts-expect-error TS2769
export default defineConfig(({mode}) => {
    const env = loadEnv(mode, process.cwd(), "");
    const port = env.PORT || 5173;
    return {
        server: {port},
        plugins: [sveltekit()],
        test: {
            include: ["src/**/*.{test,spec}.{js,ts}"],
        },
    };
});
