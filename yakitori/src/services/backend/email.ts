import {query} from "@/services/fetchers";
import useSWR from "swr";

export interface Email {
    email_id: string;
    email: string;
    is_primary: boolean;
}

export function useEmails() {
    const {data, error, isLoading} = useSWR<Email[]>("/api/emails", query);
    return {emails: data, isLoading, isError: !!error, error};
}
