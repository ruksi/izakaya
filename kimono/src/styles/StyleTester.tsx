import Alert from "react-bootstrap/Alert";
import Button from "react-bootstrap/Button";
import Container from "react-bootstrap/Container";

export default function StyleTester() {
    return (
        <Container className="py-3">
            {["primary", "secondary", "success", "danger", "warning", "info", "light", "dark"].map((variant) => (
                <div key={variant} className="d-flex gap-3 align-items-center py-2">
                    <Button variant={variant}>Button</Button>
                    <Button variant={`outline-${variant}`}>Button</Button>
                    <Alert variant={variant} className="flex-grow-1">Alert</Alert>
                </div>
            ))}
        </Container>
    );
}
