import {Container, Stack} from "react-bootstrap";
import {AccessControl} from "./AccessControl";
import {EmailControl} from "./EmailControl";
import {UserControl} from "./UserControl";

export const metadata = {
    title: "Settings",
};

export default function SettingsPage() {
    return (
        <Container className="py-3">
            <h1>Settings</h1>
            <Stack gap={3}>
                <div>
                    <UserControl />
                </div>
                <div>
                    <EmailControl />
                </div>
                <div>
                    <AccessControl />
                </div>
            </Stack>
        </Container>
    );
}
