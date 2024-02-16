import {Issue} from "@/general/errors";

export function formatMessage(issue: Issue) {
    if (issue.message) {
        return issue.message;
    }
    const formatter = getFormatter(issue);
    if (formatter) {
        return formatter(issue);
    }
    return `Invalid value (${issue.code})`;
}

type Formatter = (issue: Issue) => string;

const formatters: {[code: string]: Formatter} = {
    length: (issue: Issue) => {
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
        return "Must fulfill length requirements";
    },
    range: (issue: Issue) => {
        if (issue.params.min && issue.params.max) {
            return `Must be between values ${issue.params.min} and ${issue.params.max}`;
        }
        if (issue.params.min) {
            return `Must be at least ${issue.params.min}`;
        }
        if (issue.params.max) {
            return `Must be up to ${issue.params.max}`;
        }
        return "Must fulfill range requirement";
    },
    required: () => `Required`,
    unique: () => `Must be unique`,
    email: () => `Must be a valid email address`,
    url: () => `Must be a valid URL`,
};

function getFormatter(issue: Issue): Formatter | undefined {
    return formatters[issue.code];
}
