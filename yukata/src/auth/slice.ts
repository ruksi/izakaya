import {authLogOut, authVerify} from "@/auth/thunks";
import tatami from "@/services/tatami";
import {createSlice} from "@reduxjs/toolkit";

type AuthState = {
    isAuthenticated: boolean | null;
};

const authSlice = createSlice({
    name: "auth",
    initialState: {
        isAuthenticated: null,
    } as AuthState,
    reducers: {},
    selectors: {
        selectIsAuthenticated: (state: AuthState) => state.isAuthenticated,
    },
    extraReducers: (builder) => {
        builder.addCase(authVerify.fulfilled, (state, {payload}) => {
            state.isAuthenticated = payload.isAuthenticated;
        });

        builder.addCase(authLogOut.fulfilled, (state) => {
            state.isAuthenticated = false;
        });

        builder.addMatcher(tatami.endpoints.signUp.matchFulfilled, (state) => {
            state.isAuthenticated = true;
        });

        builder.addMatcher(tatami.endpoints.logIn.matchFulfilled, (state) => {
            state.isAuthenticated = true;
        });
    },
});

export default authSlice;

export const {selectIsAuthenticated} = authSlice.selectors;
