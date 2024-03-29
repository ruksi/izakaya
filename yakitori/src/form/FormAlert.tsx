import {formatMessage} from "@/form/messages";
import {isBackendError, Issue} from "@/general/errors";
import {Alert, Spinner} from "react-bootstrap";

interface FormAlertProps {
    title: string;
    error: any;
    isLoading?: boolean;
}

export function FormAlert({title, error, isLoading = false}: FormAlertProps) {
    let loading = null;
    if (isLoading) {
        loading = <Spinner animation="border" size="sm" className="ms-2" />;
    }

    if (!isBackendError(error)) {
        return (
            <Alert variant="danger">
                <div className="text-danger">
                    {title}
                    {loading}
                </div>
                <div className="text-secondary small">
                    {error.status
                        ? error.status == "FETCH_ERROR"
                            ? "Could not connect to the server"
                            : error.status
                        : null}
                </div>
            </Alert>
        );
    }
    return (
        <Alert variant="danger">
            <div className="text-danger">
                {title}
                {loading}
            </div>
            <div className="text-secondary small">
                {`${error.status} - ${error.data.message}`}
                {error.data.issues
                    ? Object.entries(error.data.issues)
                        .sort()
                        .map(([field, field_issues]) => (
                            <div key={field} className="ps-2">
                                <FieldDisplay
                                    field={field}
                                    issues={field_issues}
                                />
                            </div>
                        ))
                    : null}
            </div>
        </Alert>
    );
}

function FieldDisplay({field, issues}: {field: string; issues: Issue[]}) {
    return (
        <div className="ps-2">
            <div>{`${field}`}</div>
            {issues.map((issue: any) => (
                <div key={issue.code} className="ps-2">
                    <IssueDisplay issue={issue} />
                </div>
            ))}
        </div>
    );
}

function IssueDisplay({issue}: {issue: Issue}) {
    const message = formatMessage(issue);
    return <div className="text-secondary-emphasis">{message}</div>;
}
