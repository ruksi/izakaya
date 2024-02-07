/// <reference types="vite/client" />

/* eslint-disable no-unused-vars */

interface ImportMetaEnv {
    readonly VITE_TATAMI_URL: string | undefined;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}
