import viteLogo from "/vite.svg"
import Button from "react-bootstrap/Button";
import Col from "react-bootstrap/Col";
import Container from "react-bootstrap/Container";
import Form from "react-bootstrap/Form";
import InputGroup from "react-bootstrap/InputGroup";
import Row from "react-bootstrap/Row";
import {useSelector} from "react-redux";
import {useNavigate} from "react-router-dom";
import reactLogo from "../assets/react.svg"
import {selectIsAuthenticated} from "../auth/slice.ts";

export default function Home() {
    const isAuthenticated = useSelector(selectIsAuthenticated);
    return isAuthenticated ? <Dashboard/> : <Landing/>;
}

function Landing() {

    const navigate = useNavigate();

    return (
        <Container className="py-lg-5 mt-lg-5 py-md-3 my-md-3 py-2 my-2">

            <div className="pt-lg-5 mt-lg-5 pt-md-3 mt-md-3 pt-2 mt-2">
                <div className="display-3 fw-semibold">
                    Very&nbsp;innovative, such&nbsp;wow&nbsp;🐶
                </div>
                <div className="fs-4 text-secondary mt-1 mt-md-2">
                    You probably want to have a static marketing site for&nbsp;SEO,&nbsp;though.
                </div>
            </div>

            <div className="
            pt-lg-5 pt-md-4 pt-3
            pb-lg-5 mb-lg-5 pb-md-4 mb-md-4 pb-3 mb-3
            ">
                <Row>
                    <Col xs={10} md={6} lg={5} xl={4}>
                        <InputGroup>
                            <Form.Control placeholder="email@example.com"/>
                            <Button onClick={() => navigate(`/sign-up`)}>Sign up</Button>
                        </InputGroup>
                    </Col>
                </Row>
            </div>

            <div className="py-lg-5 my-lg-5 py-md-4 my-md-4 py-3 my-3">
                <figure className="text-center">
                    <blockquote className="blockquote">
                        <p>Social proof is excellent for reassurance.</p>
                    </blockquote>
                    <figcaption className="blockquote-footer text-secondary-emphasis">
                        Someone Famous in <cite title="Some Company">Some Company</cite>
                    </figcaption>
                </figure>
            </div>

            <div className="pt-lg-5 mt-lg-5 pt-3 mt-3">
                <div>
                    <div className="display-5 text-info-emphasis mb-0 mb-md-2">
                        Core Message
                    </div>
                    <p className="display-5">
                        This is why you should use this service or product,
                        and how it will change your life, like, forever.
                    </p>
                </div>
            </div>

            <div className="py-lg-5 my-lg-5 py-3 my-3">
                <div className="p-3 bg-body-tertiary border rounded-2">
                    <span className="fs-4 text-secondary">
                        <span className="text-body-emphasis">Core Feature</span> helps you do thing we were just talking about above.
                    </span>
                </div>
                <div className="d-flex gap-4 pt-4">
                    <div className="p-3 bg-body-tertiary border rounded-2">
                        <span className="fs-4 text-secondary">
                            <span className="text-body-emphasis">Core Feature</span> helps you do thing we were just talking about above.
                        </span>
                    </div>
                    <div className="p-3 bg-body-tertiary border rounded-2">
                        <span className="fs-4 text-secondary">
                            <span className="text-body-emphasis">Core Feature</span> helps you do thing we were just talking about above.
                        </span>
                    </div>
                </div>
            </div>

            <div className="pt-lg-5 mt-lg-5 pt-3 mt-3">
                <div>
                    <div className=" display-5 text-success-emphasis mb-0 mb-md-2">
                        Additional Selling Point
                    </div>
                    <p className=" display-5">
                        You would probably have some inspirational images
                        and photos all around.
                    </p>
                </div>
            </div>

            <div className="py-lg-5 my-lg-5 py-3 my-3">
                <div className="d-flex gap-4 pb-4">
                    <div className="p-3 bg-body-tertiary border rounded-2">
                        <span className="fs-4 text-secondary">
                            <span className="text-body-emphasis">Core Feature</span> helps you do thing we were just talking about above.
                        </span>
                    </div>
                    <div className="p-3 bg-body-tertiary border rounded-2">
                        <span className="fs-4 text-secondary">
                            <span className="text-body-emphasis">Core Feature</span> helps you do thing we were just talking about above.
                        </span>
                    </div>
                </div>
                <div className="p-3 bg-body-tertiary border rounded-2">
                    <span className="fs-4 text-secondary">
                        <span className="text-body-emphasis">Core Feature</span> helps you do thing we were just talking about above.
                    </span>
                </div>
            </div>

            <div className="pt-lg-5 mt-lg-5 pt-3 mt-3">
                <div>
                    <div className=" display-5 text-warning-emphasis mb-0 mb-md-2">
                        More Calls to Action
                    </div>
                    <p className=" display-5">
                        You should give a path forward on each view,
                        most likely to sign up or to subscribe.
                    </p>
                </div>
            </div>

            <div className="py-lg-5 my-lg-5 py-3 my-3">
                <div className="p-3 bg-body-tertiary border rounded-2">
                        <span className="fs-4 text-secondary">
                            <span className="text-body-emphasis">Core Feature</span> helps you do thing we were just talking about above.
                        </span>
                </div>
            </div>

            <div className=" py-lg-5 my-lg-5 py-md-3 my-md-3 py-2 my-2">
                <Row>
                    <Col sm={12} md={6} lg={3}>
                        <div className="pb-4 fs-3 fst-italic">Ryokan</div>
                        <p className="fs-6 text-secondary-emphasis">
                            Register for the service to get a closer look at all the awesome features.
                        </p>
                        <p className="fst-italic text-secondary">
                            You don&apos;t really need to use your real email.
                        </p>
                        <Button onClick={() => navigate(`/sign-up`)}>Sign up</Button>
                    </Col>
                    <Col sm={12} md={6} lg={3}>
                    </Col>
                    <Col sm={12} md={6} lg={3}>
                    </Col>
                    <Col sm={12} md={6} lg={3}>
                    </Col>
                </Row>
            </div>

            <div className="text-center">
                <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
                    <img src={viteLogo} className="logo" alt="Vite logo"/>
                </a>
                <a href="https://react.dev" target="_blank" rel="noreferrer">
                    <img src={reactLogo} className="logo react" alt="React logo"/>
                </a>
            </div>

        </Container>
    );
}

function Dashboard() {
    return (
        <Container className=" my-3">
            <h1>Dashboard</h1>
            <div>
                <code># TODO: some boxes</code>
            </div>
        </Container>
    );
}