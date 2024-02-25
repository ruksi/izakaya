import handleFetch from "$lib/handleFetch";
import {backend} from "$lib/urls";
import {createMutation, createQuery, type QueryClient} from "@tanstack/svelte-query";

export function verifyQuery() {
    return createQuery({
        queryFn: api().getVerify,
        queryKey: ["verify"],
    });
}

export function signUpMutation(client: QueryClient) {
    return createMutation({
        mutationFn: api().postSignUp,
        onMutate: async () => {
            await client.cancelQueries({queryKey: ["verify"]});
            return null;
        },
        onSettled: async () => {
            await client.invalidateQueries({queryKey: ["verify"]});
        },
    });
}

export function logInMutation(client: QueryClient) {
    return createMutation({
        mutationFn: api().postLogIn,
        onMutate: async () => {
            await client.cancelQueries({queryKey: ["verify"]});
            return null;
        },
        onSettled: async () => {
            await client.invalidateQueries({queryKey: ["verify"]});
        },
    });
}

export function logOutMutation(client: QueryClient) {
    return createMutation({
        mutationFn: api().postLogOut,
        onMutate: async () => {
            await client.cancelQueries({queryKey: ["verify"]});
            return null;
        },
        onSuccess: async () => {
            // noinspection ES6MissingAwait
            client.invalidateQueries();
        },
    });
}

export function currentUserQuery() {
    return createQuery({
        queryFn: api().getUsersMe,
        queryKey: ["current-user"],
    });
}

export function emailsQuery() {
    return createQuery({
        queryFn: api().getEmails,
        queryKey: ["email"],
    });
}

export function sessionsQuery() {
    return createQuery({
        queryFn: api().getSessions,
        queryKey: ["session"],
    });
}

export function newSessionMutation(client: QueryClient) {
    return createMutation({
        mutationFn: api().postSessions,
        onSuccess: async () => {
            await client.invalidateQueries({queryKey: ["session"]});
        },
    });
}

export function revokeSessionMutation(client: QueryClient) {
    return createMutation({
        mutationFn: api().deleteSessions,
        onSuccess: async () => {
            await client.invalidateQueries({queryKey: ["session"]});
        },
    });
}

export type Status = {status: string};
export type Verify = {is_authenticated: boolean};
export type User = {user_id: string; username: string;}
export type Email = {email_id: string; email: string; is_primary: boolean;};
export type Session = {access_token_prefix: string; used_at?: string;};
export type NewSession = {access_token: string;};

type SignUpArgs = {username: string; email: string; password: string;};
type LogInArgs = {username_or_email: string; password: string;};
type PostSessionsArgs = {password: string;};
type DeleteSessionsArgs = {access_token_prefix: string;};

// In server `load`, the SvelteKit `fetch` is not yet injected, so
// we sometimes need to pass it as an argument to the API function.
export function api(_fetch = fetch) {
    return {
        getVerify: async (): Promise<Verify> => {
            const data = await handleFetch({
                url: `${backend}/verify`,
                _fetch,
            });
            return data as Verify;
        },
        postSignUp: async ({username, email, password}: SignUpArgs): Promise<Status> => {
            const data = await handleFetch({
                url: `${backend}/sign-up`,
                options: {
                    method: "POST",
                    body: JSON.stringify({username, email, password}),
                },
                _fetch,
            });
            return data as Status;
        },
        postLogIn: async ({username_or_email, password}: LogInArgs): Promise<Status> => {
            const data = await handleFetch({
                url: `${backend}/log-in`,
                options: {
                    method: "POST",
                    body: JSON.stringify({username_or_email, password}),
                },
                _fetch,
            });
            return data as Status;
        },
        postLogOut: async (): Promise<Status> => {
            const data = await handleFetch({
                url: `${backend}/log-out`,
                options: {
                    method: "POST",
                },
                _fetch,
            });
            return data as Status;
        },
        getUsersMe: async (): Promise<User> => {
            const data = await handleFetch({
                url: `${backend}/api/users/me`,
                _fetch,
            });
            return data as User;
        },
        getEmails: async (): Promise<Email[]> => {
            const data = await handleFetch({
                url: `${backend}/api/emails`,
                _fetch,
            });
            return data as Email[];
        },
        getSessions: async (): Promise<Session[]> => {
            const data = await handleFetch({
                url: `${backend}/api/sessions`,
                _fetch,
            });
            return data as Session[];
        },
        postSessions: async ({password}: PostSessionsArgs): Promise<NewSession> => {
            const data = await handleFetch({
                url: `${backend}/api/sessions`,
                options: {
                    method: "POST",
                    body: JSON.stringify({password}),
                },
                _fetch,
            });
            return data as NewSession;
        },
        deleteSessions: async ({access_token_prefix}: DeleteSessionsArgs): Promise<Status> => {
            const data = await handleFetch({
                url: `${backend}/api/sessions/${access_token_prefix}`,
                options: {
                    method: "DELETE",
                },
                _fetch,
            });
            return data as Status;
        },
    };
}
