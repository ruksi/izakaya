export function isBackendErrorResponse(
    error: any
): error is BackendErrorResponse {
    return (error as BackendErrorResponse)?.data?.message !== undefined;
}

export interface BackendErrorResponse {
    status: number;
    data: BackendError;
}

export interface BackendError {
    message: string;
    details?: BackendErrorDetails;
}

export interface BackendErrorDetails {
    [fieldName: string]: BackendIssue[];
}

export interface BackendIssue {
    code: string;
    message?: string;
    params: BackendIssueParameters;
}

export interface BackendIssueParameters {
    // almost always
    value?: any;

    // length
    min?: number;
    max?: number;
    equal?: number;

    [key: string]: any;
}
