import {css, cx} from "@emotion/css";
import Alert from "react-bootstrap/Alert";
import Button from "react-bootstrap/Button";
import Container from "react-bootstrap/Container";
import {responsive} from "./breakpoints.ts";

const disco = css(
    responsive({
        textDecoration: "underline",
        color: [
            "var(--bs-blue)",   // xs aka. "default"
            "var(--bs-teal)",   // sm
            "var(--bs-yellow)", // md
            "var(--bs-orange)", // lg
            "var(--bs-red)",    // xl
            "var(--bs-pink)",   // xxl
        ],
    }),
);

export default function StyleTester() {
    return (
        <Container className="py-3">
            <div className={cx("py-3 my-3", disco)}>Disco Breakpoint!</div>
            {[
                "primary",
                "secondary",
                "success",
                "danger",
                "warning",
                "info",
                "light",
                "dark",
            ].map((variant) => (
                <div
                    key={variant}
                    className="d-flex gap-3 align-items-center py-2"
                >
                    <Button variant={variant}>Button</Button>
                    <Button variant={`outline-${variant}`}>Button</Button>
                    <Alert variant={variant} className="flex-grow-1">
                        Alert
                    </Alert>
                </div>
            ))}
        </Container>
    );
}
