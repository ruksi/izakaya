import {isBackendError} from "../general/errors.ts";

export function isErroneous(field: string, error: any): boolean {
    if (!isBackendError(error)) {
        return false;
    }
    if (!error.data.details) {
        return false;
    }
    const issues = error.data.details[field];
    return !!issues;
}
