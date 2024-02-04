import {createAsyncThunk} from "@reduxjs/toolkit";
import {tatamiUrl} from "../utils";

const baseUrl = tatamiUrl();

interface VerifyPayload {
    is_authenticated: boolean;
    user_id: string | null;
}

export const authVerify = createAsyncThunk("auth/verify", async () => {
    const response = await fetch(`${baseUrl}/verify`, {credentials: "include"});
    if (!response.ok) {
        throw Error();
    }
    const data = await response.json();
    return {
        is_authenticated: data.is_authenticated,
        user_id: data.user_id,
    } as VerifyPayload;
});


export const authLogOut = createAsyncThunk(
    "auth/log-out",
    async () => {
        const response = await fetch(`${baseUrl}/log-out`, {method: "POST", credentials: "include"});
        if (!response.ok) {
            throw Error();
        }
        return;
    },
)
