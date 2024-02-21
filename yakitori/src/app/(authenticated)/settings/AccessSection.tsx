"use client";

import {
    NewSession,
    Session,
    useCreateSession,
    useRevokeSession,
    useSessions,
} from "@/services/backend/session";
import {
    faBan,
    faKey,
    faPlus,
    faThumbsUp,
} from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {formatDistance, parseISO} from "date-fns";
import React, {useCallback, useEffect, useState} from "react";
import {
    Button,
    Form,
    InputGroup,
    Modal,
    Placeholder,
    Stack,
} from "react-bootstrap";

export function AccessSection() {
    const {sessions, isLoading} = useSessions();
    return (
        <Stack gap={3}>
            {sessions ? (
                sessions.map((session: Session) => (
                    <SessionDisplay
                        key={session.access_token_prefix}
                        session={session}
                    />
                ))
            ) : (
                <div className="d-flex flex-wrap align-items-center">
                    <div className="d-inline-block" style={{minWidth: 350}}>
                        <div>
                            <Placeholder
                                aria-hidden="true"
                                bg="secondary"
                                style={{width: 190}}
                            />
                        </div>
                        <div>
                            <Placeholder
                                aria-hidden="true"
                                bg="secondary"
                                style={{width: 225}}
                            />
                        </div>
                    </div>
                    <div className="d-inline-block mt-1">
                        <Button size="sm" variant="secondary" disabled>
                            <FontAwesomeIcon icon={faBan} className="me-1" />
                            Revoke
                        </Button>
                    </div>
                </div>
            )}
            <div>
                <CreateTokenControl isLoadingSessions={isLoading} />
            </div>
        </Stack>
    );
}

function SessionDisplay({session}: {session: Session}) {
    const {revokeSession, isLoading} = useRevokeSession(
        session.access_token_prefix
    );
    const [isDeleting, setIsDeleting] = useState(false);

    const onClickRevoke = useCallback(() => {
        setIsDeleting(true);
    }, [setIsDeleting]);
    const onClickCancel = useCallback(() => {
        setIsDeleting(false);
    }, [setIsDeleting]);
    const onClickConfirm = useCallback(() => {
        revokeSession();
        setIsDeleting(false);
    }, [setIsDeleting, revokeSession]);

    let lastUsed = <span className="text-secondary">Never</span>;
    if (session?.used_at) {
        const at = parseISO(session.used_at);
        const ago = formatDistance(at, new Date(), {addSuffix: true});
        lastUsed = <abbr title={session.used_at}>{ago}</abbr>;
    }

    return (
        <div className="d-flex flex-wrap align-items-center">
            <div className="d-inline-block" style={{minWidth: 350}}>
                <div>
                    <label className="text-body-emphasis me-1">
                        Access Token:
                    </label>
                    <code>{session.access_token_prefix}...</code>
                </div>
                <div>
                    <label className="text-body-emphasis me-1">Last Use:</label>
                    {lastUsed}
                </div>
            </div>
            <div className="d-inline-block mt-1">
                {isDeleting ? (
                    <Stack direction="horizontal" gap={2}>
                        <span>Are you sure?</span>
                        <Button
                            size="sm"
                            variant="secondary"
                            onClick={onClickCancel}
                        >
                            Cancel
                        </Button>
                        <Button
                            size="sm"
                            variant="danger"
                            onClick={onClickConfirm}
                        >
                            Revoke
                        </Button>
                    </Stack>
                ) : (
                    <Button
                        size="sm"
                        variant="danger"
                        disabled={isLoading}
                        onClick={onClickRevoke}
                    >
                        <FontAwesomeIcon icon={faBan} className="me-1" />
                        Revoke
                    </Button>
                )}
            </div>
        </div>
    );
}

function CreateTokenControl({isLoadingSessions}: {isLoadingSessions: boolean}) {
    const {createSession, newSession, isLoading, isSuccess, resetNewSession} =
        useCreateSession();

    const [isCreating, setIsCreating] = useState(false);
    const [password, setPassword] = useState("");
    const [modalShow, setModalShow] = useState(false);

    useEffect(() => {
        if (isSuccess) {
            setModalShow(true);
        }
    }, [setModalShow, isSuccess]);

    const confirm = useCallback(
        (e: React.FormEvent<HTMLFormElement>) => {
            e.preventDefault();
            createSession({password});
            setPassword("");
            setIsCreating(false);
        },
        [createSession, password, setPassword, setIsCreating]
    );

    const onModalClose = useCallback(() => {
        setModalShow(false);
        resetNewSession();
    }, [setModalShow, resetNewSession]);

    return (
        <div className="d-flex flex-wrap align-items-center">
            <div className="d-inline-block" style={{minWidth: 350}}>
                <div>
                    <Placeholder
                        aria-hidden="true"
                        bg="secondary"
                        style={{width: 190}}
                    />
                </div>
                <div>
                    <Placeholder
                        aria-hidden="true"
                        bg="secondary"
                        style={{width: 225}}
                    />
                </div>
            </div>
            <div className="d-inline-block mt-1">
                {isCreating ? (
                    <Form onSubmit={confirm}>
                        <InputGroup size="sm">
                            <Form.Control
                                type="password"
                                placeholder="Re-type your password..."
                                value={password}
                                onChange={(e) => setPassword(e.target.value)}
                                autoFocus={true}
                            />
                            <Button type="submit">Confirm</Button>
                        </InputGroup>
                    </Form>
                ) : (
                    <Button
                        size="sm"
                        variant={isLoadingSessions ? "secondary" : "primary"}
                        className="text-nowrap"
                        disabled={isLoadingSessions || isLoading}
                        onClick={() => setIsCreating(true)}
                    >
                        <FontAwesomeIcon icon={faPlus} className="me-1" />
                        Create API token
                    </Button>
                )}
            </div>
            {modalShow && newSession && (
                <NewSessionModal
                    session={newSession}
                    show={modalShow}
                    onClose={onModalClose}
                />
            )}
        </div>
    );
}

function NewSessionModal({
    session,
    show,
    onClose,
}: {
    session: NewSession;
    show: boolean;
    onClose: () => void;
}) {
    return (
        <Modal show={show}>
            <Modal.Header>
                <Modal.Title>
                    <FontAwesomeIcon
                        icon={faKey}
                        className="ms-1 me-3 text-warning"
                    />
                    Your API Token is Ready!
                </Modal.Title>
            </Modal.Header>
            <Modal.Body>
                <p>Copy the following token:</p>
                <pre className="text-wrap text-warning-emphasis">
                    <code>{session.access_token}</code>
                </pre>
                <p>
                    <strong>Keep this token safe!</strong> You won&apos;t be
                    seeing it again.
                </p>
            </Modal.Body>
            <Modal.Footer>
                <Button variant="secondary" onClick={onClose}>
                    <FontAwesomeIcon icon={faThumbsUp} className="me-2" />
                    Yes, I copied it
                </Button>
            </Modal.Footer>
        </Modal>
    );
}
