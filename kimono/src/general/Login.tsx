import Button from "react-bootstrap/Button";
import Stack from "react-bootstrap/esm/Stack";
import Form from "react-bootstrap/Form";

export default function Login() {
    return (
        <Stack gap={2} className="col-sm-6 col-md-5 col-lg-4 col-xl-3 mx-auto">
            <Form className="p-3">
                <Form.Group className="mb-3" controlId="username-or-email">
                    <Form.Label>Username or Email</Form.Label>
                    <Form.Control type="text"/>
                </Form.Group>

                <Form.Group className="mb-4" controlId="password">
                    <Form.Label>Password</Form.Label>
                    <Form.Control type="password" placeholder="Password"/>
                </Form.Group>

                <Button variant="primary" type="submit">
                    Login
                </Button>
            </Form>
        </Stack>
    );
}
