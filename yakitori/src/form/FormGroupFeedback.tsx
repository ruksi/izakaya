import {formatMessage} from "@/form/messages";
import {isBackendError} from "@/general/errors";

export function FormGroupFeedback({field, error}: {field: string; error: any}) {
    if (!isBackendError(error)) {
        return null;
    }
    if (!error.data.issues) {
        return null;
    }
    const field_issues = error.data.issues[field];
    if (!field_issues) {
        return null;
    }
    return (
        <div className="text-danger small ps-1">
            {field_issues.map((issue: any) => (
                <div key={issue.code}>{formatMessage(issue)}</div>
            ))}
        </div>
    );
}
