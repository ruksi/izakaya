import {tatamiUrl} from "@/utils";
import {createAsyncThunk} from "@reduxjs/toolkit";

const baseUrl = tatamiUrl();

interface VerifyPayload {
    isAuthenticated: boolean;
    userId?: string;
}

export const authVerify = createAsyncThunk("auth/verify", async () => {
    const response = await fetch(`${baseUrl}/verify`, {credentials: "include"});
    if (response.status === 401) {
        return {isAuthenticated: false} as VerifyPayload;
    }
    if (!response.ok) {
        throw Error();
    }
    const data = await response.json();
    return {
        isAuthenticated: true,
        userId: data?.userId,
    } as VerifyPayload;
});

export const authLogOut = createAsyncThunk("auth/log-out", async () => {
    const response = await fetch(`${baseUrl}/log-out`, {
        method: "POST",
        credentials: "include",
    });
    if (!response.ok) {
        throw Error();
    }
    return;
});
