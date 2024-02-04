import {isBackendErrorResponse} from "@/form/types";

export function isErroneous(field: string, error: any): boolean {
    if (!isBackendErrorResponse(error)) {
        return false;
    }
    if (!error.data.details) {
        return false;
    }
    const issues = error.data.details[field];
    return !!issues;
}
