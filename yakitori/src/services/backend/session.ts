import {mutation, query} from "@/services/fetchers";
import useSWR from "swr";
import useSWRMutation from "swr/mutation";

export type Session = {access_token_prefix: string; used_at?: string};

export function useSessions() {
    const {data, error, isLoading} = useSWR<Session[]>("/api/sessions", query);
    return {sessions: data, isLoading, isError: !!error, error};
}

type CreateSessionExtra = {arg: {password: string}};
export type NewSession = {access_token: string};

export function useCreateSession() {
    const {trigger, data, error, isMutating, reset} = useSWRMutation(
        "/api/sessions",
        (url, extra: CreateSessionExtra) => mutation([url, "POST"], extra)
    );
    return {
        createSession: trigger,
        newSession: data,
        isLoading: isMutating,
        isError: !!error,
        isSuccess: !!data && !error,
        error,
        resetNewSession: reset,
    };
}

export function useRevokeSession(prefix: string) {
    const {trigger, isMutating} = useSWRMutation(
        "/api/sessions",
        (url, extra) => mutation([`${url}/${prefix}`, "DELETE"], extra)
    );
    return {revokeSession: trigger, isLoading: isMutating};
}
