import {createAsyncThunk} from "@reduxjs/toolkit";
import {getBackendUrl} from "../urls.ts";

const backend = getBackendUrl();

interface VerifyPayload {
    is_authenticated: boolean;
    user_id: string | null;
}

export const authVerify = createAsyncThunk("auth/verify", async () => {
    const response = await fetch(`${backend}/verify`, {credentials: "include"});
    if (!response.ok) {
        throw Error();
    }
    const data = await response.json();
    return {
        is_authenticated: data.is_authenticated,
        user_id: data.user_id,
    } as VerifyPayload;
});
