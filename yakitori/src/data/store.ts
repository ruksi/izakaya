import authSlice from "@/auth/slice";
import tatami from "@/services/tatami";
import {configureStore} from "@reduxjs/toolkit";
import {setupListeners} from "@reduxjs/toolkit/query";

export const makeStore = () => {
    const store = configureStore({
        reducer: {
            auth: authSlice.reducer,
            [tatami.reducerPath]: tatami.reducer,
        },
        middleware: (defaults) => defaults().concat(tatami.middleware),
    });
    setupListeners(store.dispatch);
    return store;
};

export type AppStore = ReturnType<typeof makeStore>;
export type RootState = ReturnType<AppStore["getState"]>;
export type AppDispatch = AppStore["dispatch"];
