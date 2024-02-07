export function isBackendError(error: any): error is BackendError {
    return (error as BackendError)?.data?.message !== undefined;
}

export interface BackendError {
    status: number;
    data: ErrorData;
}

export interface ErrorData {
    message: string;
    details?: ErrorDetails;
}

export interface ErrorDetails {
    [fieldName: string]: Issue[]
}

export interface Issue {
    code: string;
    message?: string;
    params: IssueParams;
}

export interface IssueParams {
    // almost always
    value?: any;

    // length
    min?: number;
    max?: number;
    equal?: number;

    [key: string]: any;
}
