import {isTatamiError} from "../general/errors.ts";

export function isErroneous(field: string, error: any): boolean {
    if (!isTatamiError(error)) {
        return false;
    }
    if (!error.data.details) {
        return false;
    }
    const issues = error.data.details[field];
    return !!issues;
}
