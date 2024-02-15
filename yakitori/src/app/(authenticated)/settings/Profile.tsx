import {Email, useCurrentUser, useEmails} from "@/services/backend";
import Col from "react-bootstrap/Col";
import ListGroup from "react-bootstrap/ListGroup";
import Placeholder from "react-bootstrap/Placeholder";
import Row from "react-bootstrap/Row";

export function Profile() {
    const {user} = useCurrentUser();
    const {emails} = useEmails();
    return (
        <>
            <h4 className="mt-3">Profile</h4>
            <div>
                <label className="text-body-emphasis me-1">Username:</label>
                {user ? (
                    user.username
                ) : (
                    <Placeholder
                        aria-hidden="true"
                        bg="secondary"
                        style={{width: 60}}
                    />
                )}
            </div>
            <h4 className="mt-4">Emails</h4>
            <Row>
                <Col sm={8} md={7} lg={5} xl={4}>
                    <ListGroup>
                        {emails &&
                            emails.map((email) => (
                                <EmailDisplay
                                    key={email.email_id}
                                    email={email}
                                />
                            ))}
                        {!emails && <EmailDisplayPlaceholder />}
                    </ListGroup>
                </Col>
            </Row>
        </>
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
