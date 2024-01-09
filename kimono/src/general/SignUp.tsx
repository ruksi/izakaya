import Button from "react-bootstrap/Button";
import Stack from "react-bootstrap/esm/Stack";
import Form from "react-bootstrap/Form";

export default function SignUp() {
    return (
        <Stack gap={2} className="col-sm-6 col-md-5 col-lg-4 col-xl-3 mx-auto">
            <Form className="p-3">
                <Form.Group className="mb-3" controlId="email">
                    <Form.Label>Email</Form.Label>
                    <Form.Control type="email"/>
                </Form.Group>

                <Form.Group className="mb-3" controlId="username">
                    <Form.Label>Username</Form.Label>
                    <Form.Control type="text"/>
                </Form.Group>

                <Form.Group className="mb-4" controlId="password">
                    <Form.Label>Password</Form.Label>
                    <Form.Control type="password" placeholder="Password"/>
                </Form.Group>

                <Button variant="primary" type="submit">
                    Sign Up
                </Button>
            </Form>
        </Stack>
    );
}
