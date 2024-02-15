import {createSlice} from "@reduxjs/toolkit";
import backend from "../services/backend.ts";
import {authVerify} from "./thunks.ts";

type AuthState = {
    isAuthenticated: boolean;
};

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
        builder.addCase(authVerify.fulfilled, (state, {payload}) => {
            state.isAuthenticated = payload.is_authenticated;
        });

        builder.addMatcher(backend.endpoints.signUp.matchFulfilled, (state) => {
            state.isAuthenticated = true;
        });

        builder.addMatcher(backend.endpoints.logIn.matchFulfilled, (state) => {
            state.isAuthenticated = true;
        });

        builder.addMatcher(backend.endpoints.logOut.matchFulfilled, (state) => {
            state.isAuthenticated = false;
        });
    },
});

export default authSlice;

export const {selectIsAuthenticated} = authSlice.selectors;
