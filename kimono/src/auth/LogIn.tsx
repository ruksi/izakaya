import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import React, {useCallback, useState} from "react";
import Button from "react-bootstrap/Button";
import Card from "react-bootstrap/Card";
import Col from "react-bootstrap/Col";
import Container from "react-bootstrap/Container";
import Stack from "react-bootstrap/esm/Stack";
import Form from "react-bootstrap/Form";
import Row from "react-bootstrap/Row";
import {Link} from "react-router-dom";
import {FormAlert} from "../forms/FormAlert.tsx";
import ButtonSpinnerIf from "../general/ButtonSpinnerIf.tsx";
import useTitle from "../general/useTitle.ts";
import tatami from "../services/tatami.ts";

export default function LogIn() {
    useTitle("Log In");

    const [usernameOrEmail, setUsernameOrEmail] = useState("");
    const [password, setPassword] = useState("");

    const [logIn, {isLoading, isError, error}] = tatami.endpoints.logIn.useLazyQuery();

    const submit = useCallback((e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        logIn({username_or_email: usernameOrEmail, password: password});
    }, [logIn, usernameOrEmail, password]);

    return (
        <Container>

            <Row className="justify-content-center mt-lg-3">
                <Col xs md="8" lg="6" xl="5" xxl="4">
                    <Form className="pt-2 p-sm-2" noValidate onSubmit={submit}>

                        {isError || (isLoading && error)
                            ? <FormAlert title="Login Failed" error={error} isLoading={isLoading}/>
                            : null}

                        <Card className="my-2 my-sm-4" bg="dark">

                            <Card.Header>
                                <Card.Title>
                                    Log In
                                </Card.Title>
                            </Card.Header>

                            <Card.Body>
                                <div className="p-sm-2">

                                    <Form.Group className="mb-3" controlId="username_or_email">
                                        <Form.Label id="identity-label">
                                            Username <span className="text-secondary small">or Email</span>
                                        </Form.Label>
                                        <Form.Control
                                            arial-describedby="identity-label"
                                            disabled={isLoading}
                                            type="text"
                                            value={usernameOrEmail}
                                            onChange={(e) => setUsernameOrEmail(e.target.value)}
                                            autoFocus={true}
                                        />
                                    </Form.Group>

                                    <Form.Group className="mb-4" controlId="password">
                                        <Form.Label id="password-label">Password</Form.Label>
                                        <Form.Control
                                            arial-describedby="password-label"
                                            disabled={isLoading}
                                            type="password"
                                            value={password}
                                            onChange={(e) => setPassword(e.target.value)}
                                        />
                                    </Form.Group>

                                </div>
                            </Card.Body>

                            <Card.Footer>
                                <Stack gap={2} direction="horizontal">
                                    <Button
                                        arial-label="Log In"
                                        type="submit"
                                        variant="primary"
                                        className="ms-auto"
                                        disabled={isLoading}
                                    >
                                        <ButtonSpinnerIf isLoading={isLoading}>
                                            <FontAwesomeIcon icon="right-from-bracket" className="me-1"/>Log in
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
                    New to Ryokan? <Link to="/sign-up">Create an account</Link>
                </Col>
            </Row>

        </Container>
    )
        ;
}
