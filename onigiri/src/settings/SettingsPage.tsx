import {Container, Stack} from "react-bootstrap";
import useTitle from "../general/useTitle.ts";
import {AccessSection} from "./AccessSection.tsx";
import {EmailSection} from "./EmailSection.tsx";
import {UserSection} from "./UserSection.tsx";

export function SettingsPage() {
    useTitle("Settings");
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
