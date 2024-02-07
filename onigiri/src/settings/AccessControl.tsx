import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {formatDistance, parseISO} from "date-fns";
import React, {useCallback, useEffect, useState} from "react";
import Button from "react-bootstrap/Button";
import Form from "react-bootstrap/Form";
import InputGroup from "react-bootstrap/InputGroup";
import Modal from "react-bootstrap/Modal";
import Placeholder from "react-bootstrap/Placeholder";
import Stack from "react-bootstrap/Stack";
import backend, {NewSession, Session} from "../services/backend.ts";

export function AccessControl() {
    const {data: sessions, isLoading} = backend.endpoints.getMySessions.useQuery();
    return <>
        <h2>Access</h2>
        <Stack gap={3}>
            {sessions
                ? sessions.map((session: Session) =>
                    <SessionDisplay key={session.access_token_prefix} session={session}/>,
                )
                : <div className="d-flex flex-wrap align-items-center">
                    <div className="d-inline-block" style={{minWidth: 350}}>
                        <div><Placeholder aria-hidden="true" bg="secondary" style={{width: 190}}/></div>
                        <div><Placeholder aria-hidden="true" bg="secondary" style={{width: 225}}/></div>
                    </div>
                    <div className="d-inline-block mt-1">
                        <Button size="sm" variant="secondary" disabled>
                            <FontAwesomeIcon icon="ban" className="me-1"/>Revoke
                        </Button>
                    </div>
                </div>
            }
            <div>
                <CreateTokenControl isLoadingSessions={isLoading}/>
            </div>
        </Stack>
    </>;
}

function SessionDisplay({session}: { session: Session }) {
    const [deleteMySession, {isLoading}] = backend.endpoints.deleteMySession.useMutation();
    const [isDeleting, setIsDeleting] = useState(false);

    const onClickRevoke = useCallback(() => {
        setIsDeleting(true);
    }, [setIsDeleting]);
    const onClickCancel = useCallback(() => {
        setIsDeleting(false);
    }, [setIsDeleting]);
    const onClickConfirm = useCallback(() => {
        deleteMySession({access_token_prefix: session.access_token_prefix});
        setIsDeleting(false);
    }, [setIsDeleting, deleteMySession, session]);

    let lastUsed = <span className="text-secondary">Never</span>;
    if (session?.used_at) {
        const at = parseISO(session.used_at);
        const ago = formatDistance(at, new Date(), {addSuffix: true})
        lastUsed = <abbr title={session.used_at}>{ago}</abbr>;
    }

    return (
        <div className="d-flex flex-wrap align-items-center">
            <div className="d-inline-block" style={{minWidth: 350}}>
                <div>
                    <label className="text-body-emphasis me-1">Access Token:</label>
                    <code>{session.access_token_prefix}...</code>
                </div>
                <div>
                    <label className="text-body-emphasis me-1">Last Use:</label>
                    {lastUsed}
                </div>
            </div>
            <div className="d-inline-block mt-1">
                {isDeleting
                    ? (
                        <Stack direction="horizontal" gap={2}>
                            <span>Are you sure?</span>
                            <Button size="sm" variant="secondary" onClick={onClickCancel}>Cancel</Button>
                            <Button size="sm" variant="danger" onClick={onClickConfirm}>Revoke</Button>
                        </Stack>
                    )
                    : (
                        <Button size="sm" variant="danger" disabled={isLoading} onClick={onClickRevoke}>
                            <FontAwesomeIcon icon="ban" className="me-1"/>Revoke
                        </Button>
                    )
                }
            </div>
        </div>
    );
}

function CreateTokenControl({isLoadingSessions}: { isLoadingSessions: boolean }) {
    const [createSession, {
        data: newSession,
        isLoading,
        isSuccess,
        reset: resetNewSession,
    }] = backend.endpoints.createSession.useMutation();

    const [isCreating, setIsCreating] = useState(false);
    const [password, setPassword] = useState("");
    const [modalShow, setModalShow] = useState(false);

    useEffect(() => {
        if (isSuccess) {
            setModalShow(true);
        }
    }, [setModalShow, isSuccess]);

    const confirm = useCallback((e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        createSession({password});
        setPassword("");
        setIsCreating(false);
    }, [createSession, password, setPassword, setIsCreating]);

    const onModalClose = useCallback(() => {
        setModalShow(false);
        resetNewSession();
    }, [setModalShow, resetNewSession]);

    return (
        <div className="d-flex flex-wrap align-items-center">
            <div className="d-inline-block" style={{minWidth: 350}}>
                <div><Placeholder aria-hidden="true" bg="secondary" style={{width: 190}}/></div>
                <div><Placeholder aria-hidden="true" bg="secondary" style={{width: 225}}/></div>
            </div>
            <div className="d-inline-block mt-1">
                {isCreating
                    ? (
                        <Form onSubmit={confirm}>
                            <InputGroup size="sm">
                                <Form.Control
                                    type="password"
                                    placeholder="Re-type your password..."
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                    autoFocus={true}
                                />
                                <Button type="submit">
                                    Confirm
                                </Button>
                            </InputGroup>
                        </Form>
                    )
                    : (
                        <Button
                            size="sm"
                            variant={isLoadingSessions ? "secondary" : "primary"}
                            className="text-nowrap"
                            disabled={isLoadingSessions || isLoading}
                            onClick={() => setIsCreating(true)}
                        >
                            <FontAwesomeIcon icon="plus" className="me-1"/>Create API token
                        </Button>
                    )
                }
            </div>
            {modalShow && newSession && (
                <NewSessionModal session={newSession} show={modalShow} onClose={onModalClose}/>)
            }
        </div>
    );
}

function NewSessionModal({session, show, onClose}: { session: NewSession, show: boolean, onClose: () => void }) {
    return (
        <Modal show={show}>
            <Modal.Header>
                <Modal.Title>
                    <FontAwesomeIcon icon="key" className="ms-1 me-3 text-warning"/>
                    Your API Token is Ready!
                </Modal.Title>
            </Modal.Header>
            <Modal.Body>
                <p>
                    Copy the following token:
                </p>
                <pre className="text-wrap text-warning-emphasis"><code>{session.access_token}</code></pre>
                <p>
                    <strong>Keep this token safe!</strong> You won&apos;t be seeing it again.
                </p>
            </Modal.Body>
            <Modal.Footer>
                <Button variant="secondary" onClick={onClose}>
                    <FontAwesomeIcon icon="thumbs-up" className="me-2"/>Yes, I copied it
                </Button>
            </Modal.Footer>
        </Modal>
    );
}
