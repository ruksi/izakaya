import {mutation, query} from "@/services/fetchers";
import useSWR from "swr";
import useSWRMutation from "swr/mutation";

interface Verify {
    is_authenticated: boolean;
}

export function useVerify() {
    const {data, error, isLoading} = useSWR<Verify>("/verify", query);
    let isAuthenticated = undefined;
    if (data) {
        isAuthenticated = data.is_authenticated;
    }
    return {isAuthenticated, isLoading, isError: !!error, error};
}

interface SignUpExtra {
    arg: {
        email: string;
        username: string;
        password: string;
    };
}

export function useSignUp() {
    const {trigger, error, isMutating} = useSWRMutation(
        "/verify",
        (_, extra: SignUpExtra) => mutation(["/sign-up", "POST"], extra)
    );
    return {signUp: trigger, isLoading: isMutating, isError: !!error, error};
}

interface LogInExtra {
    arg: {
        username_or_email: string;
        password: string;
    };
}

export function useLogIn() {
    const {trigger, error, isMutating} = useSWRMutation(
        "/verify",
        (_, extra: LogInExtra) => mutation(["/log-in", "POST"], extra)
    );
    return {logIn: trigger, isLoading: isMutating, isError: !!error, error};
}

export function useLogOut() {
    const {trigger, error, isMutating} = useSWRMutation("/verify", (_, extra) =>
        mutation(["/log-out", "POST"], extra)
    );

    // TODO: fix this?
    // if (data?.status == "ok") {
    //     // clear all cached data
    //     // mutate(_key => true, undefined, { revalidate: false });
    // }
    return {logOut: trigger, isLoading: isMutating, isError: !!error, error};
}

export function useCurrentUser() {
    const {data, error, isLoading} = useSWR("/api/users/me", query);
    return {user: data, isLoading, isError: !!error, error};
}

export interface Session {
    access_token_prefix: string;
    used_at?: string;
}

export function useSessions() {
    const {data, error, isLoading} = useSWR<Session[]>("/api/sessions", query);
    return {sessions: data, isLoading, isError: !!error, error};
}

interface CreateSessionExtra {
    arg: {
        password: string;
    };
}

export interface NewSession {
    access_token: string;
}

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
