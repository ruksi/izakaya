"use client";

import {AccessControl} from "@/app/(authenticated)/settings/AccessControl";
import {useCurrentUser} from "@/services/backend";
import {Container, Placeholder, Stack} from "react-bootstrap";

export default function Settings() {
    return (
        <Container className="py-3">
            <h1>Settings</h1>
            <Stack gap={3}>
                <div>
                    <Profile />
                </div>
                <div>
                    <AccessControl />
                </div>
            </Stack>
        </Container>
    );
}

function Profile() {
    const {user} = useCurrentUser();
    return (
        <>
            <h2>Profile</h2>
            <div>
                <label className="text-body-emphasis me-1">User ID:</label>
                {user?.user_id ? (
                    <code>{user.user_id}</code>
                ) : (
                    <Placeholder
                        aria-hidden="true"
                        bg="secondary"
                        style={{width: 300}}
                    />
                )}
            </div>
            <div>
                <label className="text-body-emphasis me-1">Username:</label>
                {user?.username ? (
                    user.username
                ) : (
                    <Placeholder
                        aria-hidden="true"
                        bg="secondary"
                        style={{width: 60}}
                    />
                )}
            </div>
        </>
    );
}
