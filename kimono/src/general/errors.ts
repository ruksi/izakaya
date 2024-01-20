export function isTatamiError(error: any): error is TatamiError {
    return (error as TatamiError)?.data?.message !== undefined;
}

export interface TatamiError {
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
