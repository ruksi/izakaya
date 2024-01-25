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

        getMyUser: build.query<User, void>({query: () => "/api/users/me"}),
        getMySessions: build.query<Session[], void>({query: () => "/api/sessions"}),

        // getUsers: build.query({query: () => "/api/users"}),
        // getUser: build.query({query: userId => `/api/users/${userId}`}),
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

// export const {
//     useGetUsersQuery,
//     useGetUserQuery,
// } = tatami;

export default tatami;
