import {useSelector} from "react-redux";
import {selectIsAuthenticated} from "../auth/slice.ts";
import {Dashboard} from "./Dashboard.tsx";
import {Landing} from "./Landing.tsx";

export default function Home() {
    const isAuthenticated = useSelector(selectIsAuthenticated);
    return isAuthenticated ? <Dashboard/> : <Landing/>;
}
