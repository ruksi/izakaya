import {configureStore} from "@reduxjs/toolkit"
import {setupListeners} from "@reduxjs/toolkit/query";
import counterSlice from "./Counter/slice.ts";
import tatamiApi from "./services/tatami.ts";

export const store = configureStore({
    reducer: {
        counter: counterSlice.reducer,
        [tatamiApi.reducerPath]: tatamiApi.reducer,
    },
    middleware: (defaults) => (
        defaults().concat(tatamiApi.middleware)
    ),
})
setupListeners(store.dispatch)

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
