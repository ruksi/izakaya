"use client";

import {AccessControl} from "@/app/(authenticated)/settings/AccessControl";
import {Profile} from "@/app/(authenticated)/settings/Profile";
import {Container, Stack} from "react-bootstrap";

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
