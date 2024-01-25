import Container from "react-bootstrap/Container";
import Stack from "react-bootstrap/esm/Stack";
import Placeholder from "react-bootstrap/Placeholder";
import tatami, {Session} from "../services/tatami.ts";

export function Settings() {

    const {data: user} = tatami.endpoints.getMyUser.useQuery();
    const {data: sessions} = tatami.endpoints.getMySessions.useQuery();

    return (
        <Container className="py-3">

            <h1>Settings</h1>

            <Stack gap={3}>

                <div>
                    <h2>Profile</h2>
                    <div>
                        <label className="text-body-emphasis me-1">User ID:</label>
                        {user?.user_id
                            ? user.user_id
                            : <Placeholder aria-hidden="true" bg="secondary" style={{width: 295}}/>}
                    </div>
                    <div>
                        <label className="text-body-emphasis me-1">Username:</label>
                        {user?.username
                            ? user.username
                            : <Placeholder aria-hidden="true" bg="secondary" style={{width: 60}}/>}
                    </div>
                </div>

                <div>
                    <h2>Sessions</h2>
                    <Stack gap={2}>
                        {sessions
                            ? sessions.map((session: Session) =>
                                <SessionDisplay key={session.access_token_prefix} session={session}/>,
                            )
                            : <div>
                                <div><Placeholder aria-hidden="true" bg="secondary" style={{width: 190}}/></div>
                                <div><Placeholder aria-hidden="true" bg="secondary" style={{width: 325}}/></div>
                            </div>
                        }
                    </Stack>
                </div>
            </Stack>

        </Container>
    );
}

function SessionDisplay({session}: { session: Session }) {
    return (
        <div>
            <div>
                <label className="text-body-emphasis me-1">Access Token:</label>
                <code>{session.access_token_prefix}...</code>
            </div>
            <div>
                <label className="text-body-emphasis me-1">Last Used:</label>
                {session?.used_at
                    ? session?.used_at
                    : <>-</>}
            </div>
        </div>
    );
}
