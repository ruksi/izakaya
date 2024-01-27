import React, {useCallback, useState} from "react";
import Button from "react-bootstrap/Button";
import Card from "react-bootstrap/Card";
import Col from "react-bootstrap/Col";
import Container from "react-bootstrap/Container";
import Stack from "react-bootstrap/esm/Stack";
import Form from "react-bootstrap/Form";
import Row from "react-bootstrap/Row";
import {Link} from "react-router-dom";
import {isErroneous} from "../forms/checks.ts";
import {FormGroupFeedback} from "../forms/FormGroupFeedback.tsx";
import tatami from "../services/tatami.ts";
import ButtonSpinnerIf from "../general/ButtonSpinnerIf.tsx";

export default function SignUp() {

    const [email, setEmail] = useState("");
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    const [signUp, {isLoading, error}] = tatami.endpoints.signUp.useLazyQuery();

    const submit = useCallback((e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        signUp({email: email, username: username, password: password});
    }, [signUp, email, username, password]);

    const onEmailChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        setEmail(value);

        const prefix = value.split("@")[0];
        if (username == "" || prefix.startsWith(username)) {
            setUsername(prefix);
        }
    }, [setEmail, setUsername, username]);

    return (
        <Container>

            <Row className="justify-content-center mt-lg-3">
                <Col xs md="8" lg="6" xl="5" xxl="4">
                    <Form className="p-sm-2" noValidate onSubmit={submit}>

                        <Card className="my-2 my-sm-4" bg="dark">

                            <Card.Header>
                                <Card.Title>
                                    Sign Up
                                </Card.Title>
                                <Card.Subtitle className="text-secondary">
                                    Create a new account
                                </Card.Subtitle>
                            </Card.Header>

                            <Card.Body>
                                <div className="p-sm-2">

                                    <Form.Group className="mb-3" controlId="email">
                                        <Form.Label id="email-label">Email</Form.Label>
                                        <Form.Control
                                            aria-describedby="email-label"
                                            disabled={isLoading}
                                            isInvalid={isErroneous("email", error)}
                                            type="email"
                                            value={email}
                                            onChange={onEmailChange}
                                        />
                                        <FormGroupFeedback field="email" error={error}/>
                                    </Form.Group>

                                    <Form.Group className="mb-3" controlId="username">
                                        <Form.Label id="username-label">Username</Form.Label>
                                        <Form.Control
                                            aria-describedby="username-label"
                                            disabled={isLoading}
                                            isInvalid={isErroneous("username", error)}
                                            type="text"
                                            value={username}
                                            onChange={(e) => setUsername(e.target.value)}
                                        />
                                        <FormGroupFeedback field="username" error={error}/>
                                    </Form.Group>

                                    <Form.Group className="mb-4" controlId="password">
                                        <Form.Label id="password-label">Password</Form.Label>
                                        <Form.Control
                                            aria-describedby="password-label"
                                            disabled={isLoading}
                                            isInvalid={isErroneous("password", error)}
                                            type="password"
                                            value={password}
                                            onChange={(e) => setPassword(e.target.value)}
                                        />
                                        <FormGroupFeedback field="password" error={error}/>
                                    </Form.Group>

                                </div>
                            </Card.Body>

                            <Card.Footer>
                                <Stack gap={2} direction="horizontal">
                                    <Button
                                        aria-label="Create Account"
                                        disabled={isLoading}
                                        type="submit"
                                        variant="primary"
                                        className="ms-auto"
                                    >
                                        <ButtonSpinnerIf isLoading={isLoading}>
                                            Create account
                                        </ButtonSpinnerIf>
                                    </Button>
                                </Stack>
                            </Card.Footer>

                        </Card>

                    </Form>
                </Col>
            </Row>

            <Row className="justify-content-center mt-3 mb-5">
                <Col className="text-center text-secondary">
                    Already got an account? <Link to="/log-in">Log in</Link>
                </Col>
            </Row>

        </Container>
    );
}

