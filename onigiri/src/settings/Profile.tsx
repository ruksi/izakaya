import Placeholder from "react-bootstrap/Placeholder";
import backend from "../services/backend.ts";

export function Profile() {
    const {data: user} = backend.endpoints.getMyUser.useQuery();
    return <>
        <h2>Profile</h2>
        <div>
            <label className="text-body-emphasis me-1">User ID:</label>
            {user?.user_id
                ? <code>{user.user_id}</code>
                : <Placeholder aria-hidden="true" bg="secondary" style={{width: 300}}/>}
        </div>
        <div>
            <label className="text-body-emphasis me-1">Username:</label>
            {user?.username
                ? user.username
                : <Placeholder aria-hidden="true" bg="secondary" style={{width: 60}}/>}
        </div>
    </>;
}
