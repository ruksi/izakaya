import React, {useState} from "react";
import Button from "react-bootstrap/Button";
import Stack from "react-bootstrap/esm/Stack";
import Form from "react-bootstrap/Form";
import tatamiApi from "../services/tatami.ts";
import {store} from "../store.ts";

export default function LogIn() {

    const [identity, setIdentity] = useState("");
    const [password, setPassword] = useState("");

    const logIn = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const credentials = {username_or_email: identity, password: password};
        store.dispatch(tatamiApi.endpoints.logIn.initiate(credentials));
    }

    return (
        <Stack gap={2} className="col-sm-6 col-md-5 col-lg-4 col-xl-3 mx-auto">
            <Form className="p-3" onSubmit={logIn}>
                <Form.Group className="mb-3" controlId="username-or-email">
                    <Form.Label>Username or Email</Form.Label>
                    <Form.Control
                        type="text"
                        value={identity}
                        onChange={(e) => setIdentity(e.target.value)}
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
                    Log In
                </Button>
            </Form>
        </Stack>
    );
}
