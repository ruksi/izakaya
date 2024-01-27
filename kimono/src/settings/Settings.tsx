import Container from "react-bootstrap/Container";
import Stack from "react-bootstrap/esm/Stack";
import useTitle from "../general/useTitle.ts";
import {AccessControl} from "./AccessControl.tsx";
import {Profile} from "./Profile.tsx";

export function Settings() {
    useTitle("Settings");
    return (
        <Container className="py-3">
            <h1>Settings</h1>
            <Stack gap={3}>
                <div>
                    <Profile/>
                </div>
                <div>
                    <AccessControl/>
                </div>
            </Stack>
        </Container>
    );
}

