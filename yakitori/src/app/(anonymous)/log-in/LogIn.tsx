"use client";

import {FormAlert} from "@/form/FormAlert";
import SpinnerIf from "@/general/SpinnerIf";
import {useLogIn} from "@/services/backend";
import {faRightToBracket} from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import Link from "next/link";
import React, {useCallback, useState} from "react";
import {Button, Card, Col, Container, Form, Row, Stack} from "react-bootstrap";

export default function LogIn() {
    const [usernameOrEmail, setUsernameOrEmail] = useState("");
    const [password, setPassword] = useState("");

    const {logIn, isLoading, isError, error} = useLogIn();

    const submit = useCallback(
        (e: React.FormEvent<HTMLFormElement>) => {
            e.preventDefault();
            logIn({username_or_email: usernameOrEmail, password: password});
        },
        [logIn, usernameOrEmail, password]
    );

    return (
        <Container>
            <Row className="justify-content-center mt-lg-3">
                <Col xs md="8" lg="6" xl="5" xxl="4">
                    <Form className="pt-2 p-sm-2" noValidate onSubmit={submit}>
                        {isError || (isLoading && error) ? (
                            <FormAlert
                                title="Login Failed"
                                error={error}
                                isLoading={isLoading}
                            />
                        ) : null}

                        <Card className="my-2 my-sm-4" bg="dark">
                            <Card.Header>
                                <Card.Title>Log In</Card.Title>
                            </Card.Header>

                            <Card.Body>
                                <div className="p-sm-2">
                                    <Form.Group
                                        className="mb-3"
                                        controlId="username_or_email"
                                    >
                                        <Form.Label id="identity-label">
                                            Username{" "}
                                            <span className="text-secondary small">
                                                or Email
                                            </span>
                                        </Form.Label>
                                        <Form.Control
                                            arial-describedby="identity-label"
                                            disabled={isLoading}
                                            type="text"
                                            value={usernameOrEmail}
                                            onChange={(e) =>
                                                setUsernameOrEmail(
                                                    e.target.value
                                                )
                                            }
                                            autoFocus={true}
                                        />
                                    </Form.Group>

                                    <Form.Group
                                        className="mb-4"
                                        controlId="password"
                                    >
                                        <Form.Label id="password-label">
                                            Password
                                        </Form.Label>
                                        <Form.Control
                                            arial-describedby="password-label"
                                            disabled={isLoading}
                                            type="password"
                                            value={password}
                                            onChange={(e) =>
                                                setPassword(e.target.value)
                                            }
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
                                        <SpinnerIf isLoading={isLoading}>
                                            <FontAwesomeIcon
                                                icon={faRightToBracket}
                                                className="me-1"
                                            />
                                            Log in
                                        </SpinnerIf>
                                    </Button>
                                </Stack>
                            </Card.Footer>
                        </Card>
                    </Form>
                </Col>
            </Row>

            <Row className="justify-content-center mt-3 mb-5">
                <Col className="text-center text-secondary">
                    New to Izakaya?{" "}
                    <Link href="/sign-up">Create an account</Link>
                </Col>
            </Row>
        </Container>
    );
}
