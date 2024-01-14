import React, {useState} from "react";
import Button from "react-bootstrap/Button";
import Stack from "react-bootstrap/esm/Stack";
import Form from "react-bootstrap/Form";
import tatamiApi from "../services/tatami.ts";
import {store} from "../store.ts";

export default function SignUp() {

    const [email, setEmail] = useState("");
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    const signUp = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const credentials = {email, username, password};
        store.dispatch(tatamiApi.endpoints.signUp.initiate(credentials));
    }

    return (
        <Stack gap={2} className="col-sm-6 col-md-5 col-lg-4 col-xl-3 mx-auto">
            <Form className="p-3" onSubmit={signUp}>
                <Form.Group className="mb-3" controlId="email">
                    <Form.Label>Email</Form.Label>
                    <Form.Control
                        type="email"
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                    />
                </Form.Group>

                <Form.Group className="mb-3" controlId="username">
                    <Form.Label>Username</Form.Label>
                    <Form.Control
                        type="text"
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                    />
                </Form.Group>

                <Form.Group className="mb-4" controlId="password">
                    <Form.Label>Password</Form.Label>
                    <Form.Control
                        type="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                    />
                </Form.Group>

                <Button variant="primary" type="submit">
                    Sign Up
                </Button>
            </Form>
        </Stack>
    );
}
