import {configureStore} from "@reduxjs/toolkit"
import {setupListeners} from "@reduxjs/toolkit/query";
import authSlice from "./auth/slice.ts";
import counterSlice from "./Counter/slice.ts";
import tatami from "./services/tatami.ts";

export const store = configureStore({
    reducer: {
        auth: authSlice.reducer,
        counter: counterSlice.reducer,
        [tatami.reducerPath]: tatami.reducer,
    },
    middleware: (defaults) => (
        defaults().concat(tatami.middleware)
    ),
})
setupListeners(store.dispatch)

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
