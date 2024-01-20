import Alert from "react-bootstrap/Alert";
import Spinner from "react-bootstrap/Spinner";
import {Issue, isTatamiError} from "./errors.ts";

export function FormAlert({title, error, isLoading = false}: { title: string, error: any, isLoading?: boolean }) {

    let loading = null;
    if (isLoading) {
        loading = (
            <Spinner animation="border" size="sm" className="ms-2"/>
        );
    }

    if (!isTatamiError(error)) {
        return (
            <Alert variant="danger">
                <div className="text-danger">
                    {title}
                    {loading}
                </div>
                <div className="text-secondary small">
                    {error.status ? (
                        error.status == "FETCH_ERROR" ? "Could not connect to the server" : error.status
                    ) : null}
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
                {error.data.details ?
                    Object.entries(error.data.details).sort().map(([field, issues]) => (
                        <div key={field} className="ps-2">
                            <FieldDisplay field={field} issues={issues}/>
                        </div>
                    ))
                    : null}
            </div>
        </Alert>
    );
}

function FieldDisplay({field, issues}: { field: string, issues: Issue[] }) {
    return (
        <div className="ps-2">
            <div>{`${field}`}</div>
            {issues.map((issue: any) => (
                <div key={issue.code} className="ps-2">
                    <IssueDisplay issue={issue}/>
                </div>
            ))}
        </div>
    );
}

function IssueDisplay({issue}: { issue: Issue }) {
    const message = formatMessage(issue);
    return (
        <div className="text-secondary-emphasis">
            {message}
        </div>
    );
}

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
        <div className="text-danger small">
            {issues.map((issue: any) => (
                <div key={issue.code}>
                    {formatMessage(issue)}
                </div>
            ))}
        </div>
    );
}

const issueFormatters = {
    "length": (issue: Issue) => {
        if (issue.params.equal) {
            return `Must be exactly ${issue.params.min} characters long`;
        }
        if (issue.params.min === 0 && issue.params.max) {
            return `Must be up to ${issue.params.max} characters long`;
        }
        if (issue.params.min && issue.params.max) {
            return `Must be ${issue.params.min} to ${issue.params.max} characters long`;
        }
        if (issue.params.min) {
            return `Must be at least ${issue.params.min} characters long`;
        }
        if (issue.params.max) {
            return `Must be up to ${issue.params.max} characters long`;
        }
        return "Must fulfill length requirements"
    },
    "range": (issue: Issue) => {
        if (issue.params.min && issue.params.max) {
            return `Must be between values ${issue.params.min} and ${issue.params.max}`;
        }
        if (issue.params.min) {
            return `Must be at least ${issue.params.min}`;
        }
        if (issue.params.max) {
            return `Must be up to ${issue.params.max}`;
        }
        return "Must fulfill range requirement"
    },
    "required": () => `Required`,
    "unique": () => `Must be unique`,
    "email": () => `Must be a valid email address`,
    "url": () => `Must be a valid URL`,
}

function formatMessage(issue: Issue) {
    if (issue.message) {
        return issue.message;
    }
    // @ts-ignore
    const formatter = issueFormatters[issue.code];
    if (formatter) {
        return formatter(issue);
    }
    return `Invalid value (${issue.code})`;
}
