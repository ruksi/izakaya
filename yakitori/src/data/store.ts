import authSlice from "@/auth/slice";
import backend from "@/services/backend";
import {configureStore} from "@reduxjs/toolkit";
import {setupListeners} from "@reduxjs/toolkit/query";

export const makeStore = () => {
    const store = configureStore({
        reducer: {
            auth: authSlice.reducer,
            [backend.reducerPath]: backend.reducer,
        },
        middleware: (defaults) => defaults().concat(backend.middleware),
    });
    setupListeners(store.dispatch);
    return store;
};

export type AppStore = ReturnType<typeof makeStore>;
export type RootState = ReturnType<AppStore["getState"]>;
export type AppDispatch = AppStore["dispatch"];
