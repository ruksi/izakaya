export function isBackendError(error: Error | null): error is BackendError {
    return (error as BackendError)?.data?.message !== undefined;
}

export interface BackendError {
    name: string;
    message: string;
    status: number;
    data: BackendErrorData;
}

export interface BackendErrorData {
    message: string;            // exists for all "controlled" errors
    issues?: ValidationIssues;  // exists if validation error
}

export interface ValidationIssues {
    [fieldName: string]: Issue[];
}

export interface Issue {
    code: string;
    message?: string;
    details: IssueDetailMap;
}

export interface IssueDetailMap {
    // almost always, the value that caused the issue
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    value?: any;

    // length
    min?: number;
    max?: number;
    equal?: number;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    [key: string]: any;
}
