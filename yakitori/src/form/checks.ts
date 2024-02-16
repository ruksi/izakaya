import {isBackendError} from "@/general/errors";

export function isErroneous(field: string, error: any): boolean {
    if (!isBackendError(error)) {
        return false;
    }
    if (!error.data.issues) {
        return false;
    }
    const field_issues = error.data.issues[field];
    return !!field_issues;
}
