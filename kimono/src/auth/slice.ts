import {createSlice} from "@reduxjs/toolkit"
import tatamiApi from "../services/tatami.ts";
import {authLogOut, authVerify} from "./thunks.ts";

type AuthState = {
    isAuthenticated: boolean
}

const authSlice = createSlice({
    name: "auth",
    initialState: {
        isAuthenticated: false,
    } as AuthState,
    reducers: {},
    selectors: {
        selectIsAuthenticated: (state: AuthState) => state.isAuthenticated,
    },
    extraReducers: (builder) => {

        builder.addCase(
            authVerify.fulfilled,
            (state, {payload}) => {
                state.isAuthenticated = payload.isAuthenticated;
            },
        );

        builder.addCase(
            authLogOut.fulfilled,
            (state) => {
                state.isAuthenticated = false;
            },
        )

        builder.addMatcher(
            tatamiApi.endpoints.signUp.matchFulfilled,
            (state) => {
                state.isAuthenticated = true;
            },
        );

        builder.addMatcher(
            tatamiApi.endpoints.logIn.matchFulfilled,
            (state) => {
                state.isAuthenticated = true;
            },
        );

    },
});

export default authSlice;

export const {
    selectIsAuthenticated,
} = authSlice.selectors;
