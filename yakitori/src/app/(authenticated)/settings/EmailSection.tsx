"use client";

import {Email, useEmails} from "@/services/backend";
import {Col, ListGroup, Placeholder, Row} from "react-bootstrap";

export function EmailSection() {
    const {emails} = useEmails();
    return (
        <Row>
            <Col sm={8} md={7} lg={5} xl={4}>
                <ListGroup>
                    {emails &&
                        emails.map((email) => (
                            <EmailDisplay key={email.email_id} email={email} />
                        ))}
                    {!emails && <EmailDisplayPlaceholder />}
                </ListGroup>
            </Col>
        </Row>
    );
}

function EmailDisplay({email}: {email: Email}) {
    return (
        <ListGroup.Item>
            <div>
                {email.email}
                {email.is_primary && <PrimaryLabel />}
            </div>
            {email.is_primary && <PrimaryNotice />}
        </ListGroup.Item>
    );
}

function EmailDisplayPlaceholder() {
    return (
        <ListGroup.Item>
            <div>
                <Placeholder
                    aria-hidden="true"
                    bg="secondary"
                    style={{width: 150}}
                />
            </div>
            <Placeholder
                aria-hidden="true"
                bg="secondary"
                style={{width: 240}}
            />
        </ListGroup.Item>
    );
}

function PrimaryLabel() {
    return (
        <span className="ms-1">
            – <span className="text-success-emphasis">Primary</span>
        </span>
    );
}

function PrimaryNotice() {
    return (
        <div>
            <small className="text-secondary">
                This email is used for password resets.
            </small>
        </div>
    );
}
