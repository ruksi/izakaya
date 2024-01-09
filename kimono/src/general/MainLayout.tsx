import viteLogo from "/vite.svg"
import Stack from "react-bootstrap/esm/Stack";
import {Link, Outlet} from "react-router-dom";
import reactLogo from "../assets/react.svg"

export default function MainLayout() {
    return (
        <>
            <header className="bg-body-tertiary">
                <Stack direction="horizontal" gap={2} className="p-3">
                    <Link to={`/`}>Home</Link>
                    <Link to={`/login`} className="ms-auto">Login</Link>
                    <Link to={`/sign-up`}>Sign Up</Link>
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
