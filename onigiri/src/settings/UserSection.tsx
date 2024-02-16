import {Placeholder} from "react-bootstrap";
import backend from "../services/backend.ts";

export function UserSection() {
    const {data: user} = backend.endpoints.getCurrentUser.useQuery();
    return (
        <div>
            <label className="text-body-emphasis me-1">Username:</label>
            {user?.username ? (
                user.username
            ) : (
                <Placeholder
                    aria-hidden="true"
                    bg="secondary"
                    style={{width: 60}}
                />
            )}
        </div>
    );
}
