import {mutation, query} from "@/services/fetchers";
import useSWR from "swr";
import useSWRMutation from "swr/mutation";

type Verify = {is_authenticated: boolean};

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
