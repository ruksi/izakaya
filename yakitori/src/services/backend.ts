import {backendUrl} from "@/utils";
import {createApi, fetchBaseQuery} from "@reduxjs/toolkit/query/react";

function getCookie(name: string): string | null {
    const valueStartsAt = (name.length + 1);
    return document.cookie
        .split(";")
        .map(c => c.trim())
        .filter(cookie => {
            return cookie.substring(0, valueStartsAt) === `${name}=`;
        })
        .map(cookie => {
            return decodeURIComponent(cookie.substring(valueStartsAt));
        })[0] || null;
}

function createBaseQuery() {
    const baseUrl = backendUrl();
    return fetchBaseQuery({
        baseUrl,
        credentials: "include",
        prepareHeaders: (headers, {type}) => {
            if (type === "mutation") {
                const token = getCookie("Tatami-CSRF");
                if (token) {
                    headers.set("CSRF-Token", token);
                }
            }
            return headers;
        },
    });
}


const backend = createApi({
    baseQuery: createBaseQuery(),
    tagTypes: ["CurrentUser", "Session"],
    endpoints: (build) => ({
        signUp: build.mutation({
            query: (params: {
                username: string;
                email: string;
                password: string;
            }) => ({
                url: "/sign-up",
                method: "POST",
                body: params,
            }),
        }),
        logIn: build.mutation({
            query: (params: { username_or_email: string; password: string }) => ({
                url: "/log-in",
                method: "POST",
                body: params,
            }),
        }),
        logOut: build.mutation({
            query: () => ({
                url: "/log-out",
                method: "POST",
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
// } = backend;

export default backend;
