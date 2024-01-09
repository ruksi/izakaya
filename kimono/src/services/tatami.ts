import {createApi, fetchBaseQuery} from "@reduxjs/toolkit/query/react"


const createBaseQuery = () => {
    let url = import.meta.env.VITE_TATAMI_URL ?? "";
    url = url.trim();
    if (url.endsWith("/")) {
        url = url.slice(0, -1);
    }
    if (url === "") {
        throw new Error("VITE_TATAMI_URL is not set");
    }
    const baseUrl = `${url}/api/`;
    return fetchBaseQuery({baseUrl});
}

const tatamiApi = createApi({
    baseQuery: createBaseQuery(),
    endpoints: (build) => ({
        getUsers: build.query({query: () => "/users"}),
        getUser: build.query({query: userId => `/users/${userId}`}),
    }),
});

export const {
    useGetUsersQuery,
    useGetUserQuery,
} = tatamiApi;

export default tatamiApi;
