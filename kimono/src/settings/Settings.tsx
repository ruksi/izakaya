import Container from "react-bootstrap/Container";
import Placeholder from "react-bootstrap/Placeholder";
import tatami from "../services/tatami.ts";

export function Settings() {

    const {data: profile} = tatami.endpoints.getProfile.useQuery({});

    return (
        <Container className="py-3">
            <h1>Settings</h1>
            <div>
                <h2>Profile</h2>
                <div>
                    <label className="me-1">User ID:</label>
                    {profile?.userId
                        ? profile.userId
                        : <Placeholder aria-hidden="true" bg="secondary" style={{width: 295}}/>}
                </div>
                <div>
                    <label className="me-1">Username:</label>
                    {profile?.username
                        ? profile.username
                        : <Placeholder aria-hidden="true" bg="secondary" style={{width: 60}}/>}
                </div>
            </div>
        </Container>
    );
}
