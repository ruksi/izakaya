export function isBackendError(error: any): error is BackendError {
    return (error as BackendError)?.data?.message !== undefined;
}

export interface BackendError {
    status: number;
    data: BackendErrorData;
}

// prettier-ignore
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
    value?: any;

    // length
    min?: number;
    max?: number;
    equal?: number;

    [key: string]: any;
}
