import {query} from "@/services/fetchers";
import useSWR from "swr";

export interface User {
    user_id: string;
    username: string;
}

export function useCurrentUser() {
    const {data, error, isLoading} = useSWR<User>("/api/users/me", query);
    return {user: data, isLoading, isError: !!error, error};
}
