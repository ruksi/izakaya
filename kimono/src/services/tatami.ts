import {createApi, fetchBaseQuery} from "@reduxjs/toolkit/query/react"
import {tatamiUrl} from "../utils.ts";

function createBaseQuery() {
    const baseUrl = tatamiUrl();
    return fetchBaseQuery({
        baseUrl,
        credentials: "include",
    });
}

const tatami = createApi({
    baseQuery: createBaseQuery(),
    tagTypes: ["CurrentUser", "Session"],
    endpoints: (build) => ({
        signUp: build.query({
            query: (params: { username: string, email: string, password: string }) => ({
                url: "/sign-up",
                method: "POST",
                body: params,
            }),
        }),
        logIn: build.query({
            query: (params: { username_or_email: string, password: string }) => ({
                url: "/log-in",
                method: "POST",
                body: params,
            }),
        }),

        // ["CurrentUser"]
        getMyUser: build.query<User, void>({
            query: () => "/api/users/me",
            providesTags: ["CurrentUser"],
        }),

        // ["Session"]
        createSession: build.mutation<NewSession, { password: string }>({
            query: ({password}) => ({
                url: "/api/sessions",
                method: "POST",
                body: {password},
            }),
            invalidatesTags: ["Session"],
        }),
        getMySessions: build.query<Session[], void>({
            query: () => "/api/sessions",
            providesTags: ["Session"],
        }),
        deleteMySession: build.mutation({
            query: (params: { access_token_prefix: string }) => ({
                url: `/api/sessions/${params.access_token_prefix}`,
                method: "DELETE",
            }),
            invalidatesTags: ["Session"],
        }),
    }),
});

// don't know where these belong yet. 🤷
export interface User {
    user_id: string;
    username: string;
}

export interface Session {
    access_token_prefix: string;
    used_at?: string;
}

export interface NewSession {
    access_token: string;
}

// export const {
//     useCreateSessionMutation,
// } = tatami;

export default tatami;
