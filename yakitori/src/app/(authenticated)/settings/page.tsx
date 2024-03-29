import React from "react";
import {Container, Stack} from "react-bootstrap";
import {AccessSection} from "./AccessSection";
import {EmailSection} from "./EmailSection";
import {UserSection} from "./UserSection";

export const metadata = {
    title: "Settings",
};

export default function SettingsPage() {
    return (
        <Container className="py-3">
            <h1>Settings</h1>
            <Stack gap={3}>
                <div>
                    <h4 className="mt-3">Profile</h4>
                    <UserSection />
                </div>
                <div>
                    <h4 className="mt-3">Emails</h4>
                    <EmailSection />
                </div>
                <div>
                    <h4 className="mt-3">Access</h4>
                    <AccessSection />
                </div>
            </Stack>
        </Container>
    );
}
