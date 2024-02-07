import {isTatamiError} from "../general/errors.ts";
import {formatMessage} from "./messages.ts";

export function FormGroupFeedback({field, error}: { field: string, error: any }) {
    if (!isTatamiError(error)) {
        return null;
    }
    if (!error.data.details) {
        return null;
    }
    const issues = error.data.details[field];
    if (!issues) {
        return null;
    }
    return (
        <div className="text-danger small ps-1">
            {issues.map((issue: any) => (
                <div key={issue.code}>
                    {formatMessage(issue)}
                </div>
            ))}
        </div>
    );
}
