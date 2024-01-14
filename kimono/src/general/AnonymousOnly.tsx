import {useSelector} from "react-redux";
import {Navigate, Outlet} from "react-router-dom";
import {selectIsAuthenticated} from "../auth/slice.ts";

function AnonymousOnly() {
    const isAuthenticated = useSelector(selectIsAuthenticated);
    return !isAuthenticated ? <Outlet/> : <Navigate to="/"/>;
}

export default AnonymousOnly;
