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
        if (issue.details.equal) {
            return `Must be exactly ${issue.details.min} characters long`;
        }
        if (issue.details.min === 0 && issue.details.max) {
            return `Must be up to ${issue.details.max} characters long`;
        }
        if (issue.details.min && issue.details.max) {
            return `Must be ${issue.details.min} to ${issue.details.max} characters long`;
        }
        if (issue.details.min) {
            return `Must be at least ${issue.details.min} characters long`;
        }
        if (issue.details.max) {
            return `Must be up to ${issue.details.max} characters long`;
        }
        return "Must fulfill length requirements";
    },
    range: (issue: Issue) => {
        if (issue.details.min && issue.details.max) {
            return `Must be between values ${issue.details.min} and ${issue.details.max}`;
        }
        if (issue.details.min) {
            return `Must be at least ${issue.details.min}`;
        }
        if (issue.details.max) {
            return `Must be up to ${issue.details.max}`;
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
