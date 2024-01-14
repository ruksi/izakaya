import viteLogo from "/vite.svg"
import Button from "react-bootstrap/Button";
import Stack from "react-bootstrap/esm/Stack";
import {useSelector} from "react-redux";
import {Link, Outlet} from "react-router-dom";
import reactLogo from "../assets/react.svg"
import {selectIsAuthenticated} from "../auth/slice.ts";
import {authLogOut, authVerify} from "../auth/thunks.ts";
import {store} from "../store.ts";

export default function MainLayout() {

    const verify = () => {
        store.dispatch(authVerify());
    }
    const logOut = () => {
        store.dispatch(authLogOut());
    }

    const isAuthenticated = useSelector(selectIsAuthenticated);

    return (
        <>
            <header className="bg-body-tertiary">
                <Stack direction="horizontal" gap={2} className="p-3">
                    <Link to={`/`}>Home</Link>
                    <Button onClick={verify} className="ms-auto">Verify</Button>
                    {
                        isAuthenticated &&
                        <>
                            <Button onClick={logOut}>Log Out</Button>
                        </>
                    }
                    {
                        !isAuthenticated &&
                        <>
                            <Link to={`/log-in`}>Log In</Link>
                            <Link to={`/sign-up`}>Sign Up</Link>
                        </>
                    }

                </Stack>
            </header>

            <div className="outlet">
                <Outlet/>
            </div>

            <footer className="bg-body-tertiary text-center fixed-bottom">
                <div className="text-center p-3">
                    <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
                        <img src={viteLogo} className="logo" alt="Vite logo"/>
                    </a>
                    <a href="https://react.dev" target="_blank" rel="noreferrer">
                        <img src={reactLogo} className="logo react" alt="React logo"/>
                    </a>
                </div>
            </footer>
        </>
    )
}
