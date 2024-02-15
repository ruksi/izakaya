import {useSelector} from "react-redux";
import {Navigate, Outlet} from "react-router-dom";
import {selectIsAuthenticated} from "./slice.ts";

function AuthenticatedOnly() {
    const isAuthenticated = useSelector(selectIsAuthenticated);
    return isAuthenticated ? <Outlet /> : <Navigate to="/log-in" />;
}

export default AuthenticatedOnly;
