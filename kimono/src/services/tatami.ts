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

        // api...
        getMyUser: build.query<User, void>({query: () => "/api/users/me"}),
        getUsers: build.query({query: () => "/api/users"}),
        getUser: build.query({query: userId => `/api/users/${userId}`}),
    }),
});

// dunno where this belongs yet 🤷
interface User {
    user_id: string;
    username: string;
}

// export const {
//     useGetUsersQuery,
//     useGetUserQuery,
// } = tatami;

export default tatami;
