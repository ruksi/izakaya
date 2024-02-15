import {AccessControl} from "@/app/(authenticated)/settings/AccessControl";
import {EmailControl} from "@/app/(authenticated)/settings/EmailControl";
import {UserControl} from "@/app/(authenticated)/settings/UserControl";
import {Container, Stack} from "react-bootstrap";

export const metadata = {
    title: "Settings",
};

export default function Settings() {
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
