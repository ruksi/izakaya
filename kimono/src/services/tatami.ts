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

        // auth...
        signUp: build.query({
            query: (params: { username: string, email: string, password: string }) => ({
                url: "/sessions/sign-up",
                method: "POST",
                body: params,
            }),
        }),
        logIn: build.query({
            query: (params: { username_or_email: string, password: string }) => ({
                url: "/sessions/log-in",
                method: "POST",
                body: params,
            }),
        }),

        // api...
        getUsers: build.query({query: () => "/api/users"}),
        getUser: build.query({query: userId => `/api/users/${userId}`}),
    }),
});

// export const {
//     useGetUsersQuery,
//     useGetUserQuery,
// } = tatami;

export default tatami;
