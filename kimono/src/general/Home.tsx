import svgPurpleHeart from "/purple-heart.svg";
import svgDogFace from "/dog-face.svg"
import svgRedPaperLantern from "/red-paper-lantern.svg"
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import Button from "react-bootstrap/Button";
import Col from "react-bootstrap/Col";
import Container from "react-bootstrap/Container";
import Stack from "react-bootstrap/Stack";
import Form from "react-bootstrap/Form";
import InputGroup from "react-bootstrap/InputGroup";
import Row from "react-bootstrap/Row";
import {useSelector} from "react-redux";
import {useNavigate} from "react-router-dom";
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
                    Very&nbsp;innovative, such&nbsp;wow&nbsp;
                    <picture className="d-inline-block" style={{width: "5rem", height: "auto"}}>
                        <img src={svgDogFace} className="img-fluid" alt="Dog emoji"/>
                    </picture>
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
                            <Button onClick={() => navigate(`/sign-up`)}>
                                <FontAwesomeIcon icon="pen-to-square" className="me-1"/>Sign up
                            </Button>
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
                        <p className="fs-6 text-secondary">
                            Register for <span className="text-secondary-emphasis">Ryokan</span> to get
                            a closer look at all the awesome features.
                        </p>
                        <p className="fst-italic text-secondary pb-1">
                            You don&apos;t have to use your real email.
                        </p>
                        <Button onClick={() => navigate(`/sign-up`)}>
                            <FontAwesomeIcon icon="pen-to-square" className="me-1"/>Sign up
                        </Button>
                    </Col>
                    <Col sm={12} md={6} lg={3}>
                    </Col>
                    <Col sm={12} md={6} lg={3}>
                    </Col>
                    <Col sm={12} md={6} lg={3}>
                    </Col>
                </Row>
            </div>

            <Stack className="text-center" gap={1}>
                <div>
                    <picture className="d-inline-block" style={{width: "4rem", height: "auto"}}>
                        <img src={svgRedPaperLantern} className="img-fluid" alt="Red paper lantern emoji"/>
                    </picture>
                </div>
                <div>
                    Made with
                    <div className="d-inline-block mx-1" style={{width: "1.5rem", height: "auto"}}>
                        <img src={svgPurpleHeart} alt="love" title="love"/>
                    </div>
                    by <a href="https://ruk.si/">Ruksi</a>
                </div>
            </Stack>

        </Container>
    );
}

function Dashboard() {
    return (
        <Row className="me-0">
            <Col xs={12} md={4} lg={3} xl={2} className="pe-0">
                <div className="pt-3 bg-body-secondary border-end border-bottom vh-100 overflow-y-scroll">
                    <code># TODO: some actions? </code>
                </div>
            </Col>
            <Col xs={12} md={8} lg={6} xl={8}>
                <div className="pt-3">
                    <div className="fs-5">Feed</div>
                    <code># TODO: recent activity? </code>
                </div>
            </Col>
            <Col xs={12} md={12} lg={3} xl={2} className="me-0 ps-4 ps-lg-0">
                <div className="py-3 d-flex flex-column gap-3">
                    <div className="p-3 bg-body-secondary border rounded-2">
                        <div className="small">Latest</div>
                    </div>
                    <div className="p-3 bg-body-secondary border rounded-2">
                        <div className="small">Discover</div>
                    </div>
                    <div className="p-3 bg-body-secondary border rounded-2">
                        <div className="small">Explore</div>
                    </div>
                </div>
            </Col>
        </Row>
    );
}