"use client";

import {useState} from "react";
import {Accordion, Alert, Button, Container, Modal} from "react-bootstrap";

export default function Design() {
    return (
        <Container className="py-5">
            <h1 className="text-primary py-3">Disco!</h1>

            <Accordion defaultActiveKey="0" className="py-3">
                <Accordion.Item eventKey="0">
                    <Accordion.Header>Item One</Accordion.Header>
                    <Accordion.Body>Lorem ipsum dolor sit amet</Accordion.Body>
                </Accordion.Item>
                <Accordion.Item eventKey="1">
                    <Accordion.Header>Item Two</Accordion.Header>
                    <Accordion.Body>
                        Lorem ipsum dolor sit amet <ExampleModalHere />
                    </Accordion.Body>
                </Accordion.Item>
            </Accordion>

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

function ExampleModalHere() {
    const [show, setShow] = useState(false);

    const handleClose = () => setShow(false);
    const handleShow = () => setShow(true);

    return (
        <>
            <Button variant="primary" onClick={handleShow}>
                Launch demo modal
            </Button>

            <Modal show={show} onHide={handleClose}>
                <Modal.Header closeButton>
                    <Modal.Title>Modal heading</Modal.Title>
                </Modal.Header>
                <Modal.Body>
                    Woo, you are reading this text in a modal!
                </Modal.Body>
                <Modal.Footer>
                    <Button variant="secondary" onClick={handleClose}>
                        Close
                    </Button>
                    <Button variant="primary" onClick={handleClose}>
                        Save Changes
                    </Button>
                </Modal.Footer>
            </Modal>
        </>
    );
}
