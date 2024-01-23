import Container from "react-bootstrap/Container";
import Placeholder from "react-bootstrap/Placeholder";
import tatami from "../services/tatami.ts";

export function Settings() {

    const {data: user} = tatami.endpoints.getMyUser.useQuery();

    return (
        <Container className="py-3">
            <h1>Settings</h1>
            <div>
                <h2>Profile</h2>
                <div>
                    <label className="me-1">User ID:</label>
                    {user?.user_id
                        ? user.user_id
                        : <Placeholder aria-hidden="true" bg="secondary" style={{width: 295}}/>}
                </div>
                <div>
                    <label className="me-1">Username:</label>
                    {user?.username
                        ? user.username
                        : <Placeholder aria-hidden="true" bg="secondary" style={{width: 60}}/>}
                </div>
            </div>
        </Container>
    );
}
