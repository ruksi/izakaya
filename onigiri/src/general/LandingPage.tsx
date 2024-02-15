import {css, cx, keyframes} from "@emotion/css";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import Button from "react-bootstrap/Button";
import Col from "react-bootstrap/Col";
import Container from "react-bootstrap/Container";
import Form from "react-bootstrap/Form";
import InputGroup from "react-bootstrap/InputGroup";
import Row from "react-bootstrap/Row";
import Stack from "react-bootstrap/Stack";
import {useNavigate} from "react-router-dom";
import svgPurpleHeart from "../assets/purple-heart.svg";
import svgRedPaperLantern from "../assets/red-paper-lantern.svg";

const swing = keyframes({
    "0%": {transform: "rotate(8deg)", transformOrigin: "top"},
    "50%": {transform: "rotate(-8deg)", transformOrigin: "top"},
    "100%": {transform: "rotate(8deg)", transformOrigin: "top"},
});

const heroLantern = css({
    width: "5rem",
    height: "auto",
    backgroundColor: "rgba(255, 183, 120, 0.7)",
    borderRadius: "50%",
    boxShadow: "0 0 0.5rem 0.25rem rgba(255, 183, 120, 0.4)",
});
const footerLantern = css({
    width: "4rem",
    height: "auto",
    backgroundColor: "rgba(255, 183, 120, 0.6)",
    borderRadius: "50%",
    boxShadow: "0 0 0.5rem 0.25rem rgba(255, 183, 120, 0.5)",
    willChange: "transform, transform-origin",
    animation: `${swing} 4s ease-in-out infinite`,
});
const footerHeart = css({width: "1.5rem", height: "auto"});

export default function LandingPage() {
    const navigate = useNavigate();

    return (
        <Container className="py-3 my-3 py-lg-5 mt-lg-5">
            <div className="pt-2 mt-2 pt-md-4 mt-md-4 pt-lg-5 mt-lg-5">
                <div className="display-3 fw-semibold d-inline-flex">
                    <picture
                        className={cx(
                            "d-inline-flex align-self-center me-4",
                            heroLantern
                        )}
                    >
                        <img
                            src={svgRedPaperLantern}
                            className="img-fluid"
                            alt="Red paper lantern emoji"
                        />
                    </picture>
                    This is Izakaya landing page
                </div>
                <div className="fs-4 text-secondary mt-2 mt-md-3">
                    with Vite + React + React Router
                </div>
            </div>

            <div
                className="
            pt-3 pt-md-4 pt-lg-5
            pb-3 mb-3 pb-lg-5 mb-lg-5
            "
            >
                <Row>
                    <Col xs={10} md={6} lg={5} xl={4}>
                        <InputGroup>
                            <Form.Control placeholder="email@example.com" />
                            <Button onClick={() => navigate(`/sign-up`)}>
                                <FontAwesomeIcon
                                    icon="pen-to-square"
                                    className="me-1"
                                />
                                Sign up
                            </Button>
                        </InputGroup>
                    </Col>
                </Row>
            </div>

            <div className="py-4 my-4">
                <figure className="text-center">
                    <blockquote className="blockquote">
                        <p>Social proof is excellent for reassurance.</p>
                    </blockquote>
                    <figcaption className="blockquote-footer text-secondary-emphasis">
                        Bob the Famous in <cite>Big Company</cite>
                    </figcaption>
                </figure>
            </div>

            <div className="pt-1 mt-1 pt-lg-5 mt-lg-5">
                <div>
                    <div className="display-5 text-info-emphasis mb-0 mb-md-2">
                        Core Message
                    </div>
                    <p className="display-5">
                        This is why you should use this service or product, and
                        how it will change your life forever.
                    </p>
                </div>
            </div>
            <div className="pb-5 mb-5 pt-2 mt-2">
                <div className="p-3 bg-body-tertiary border rounded-2">
                    <span className="fs-4 text-secondary">
                        <span className="text-body-emphasis">
                            Major Feature
                        </span>{" "}
                        helps you do thing we were just talking about above.
                    </span>
                </div>
                <div className="d-flex gap-4 pt-4">
                    <div className="p-3 bg-body-tertiary border rounded-2">
                        <span className="fs-4 text-secondary">
                            <span className="text-body-emphasis">
                                Minor Feature
                            </span>{" "}
                            helps you do thing we were just talking about above.
                        </span>
                    </div>
                    <div className="p-3 bg-body-tertiary border rounded-2">
                        <span className="fs-4 text-secondary">
                            <span className="text-body-emphasis">
                                Minor Feature
                            </span>{" "}
                            helps you do thing we were just talking about above.
                        </span>
                    </div>
                </div>
            </div>

            <div className="pt-5 mt-5">
                <div>
                    <div className=" display-5 text-success-emphasis mb-0 mb-md-2">
                        Additional Benefits
                    </div>
                    <p className=" display-5">
                        You would probably have some inspirational images or
                        photographs all around.
                    </p>
                </div>
            </div>
            <div className="pb-5 mb-5 pt-2 mt-2">
                <div className="d-flex gap-4 pb-4">
                    <div className="p-3 bg-body-tertiary border rounded-2">
                        <span className="fs-4 text-secondary">
                            <span className="text-body-emphasis">
                                Left Benefit
                            </span>{" "}
                            helps you do thing we were just talking about above.
                        </span>
                    </div>
                    <div className="p-3 bg-body-tertiary border rounded-2">
                        <span className="fs-4 text-secondary">
                            <span className="text-body-emphasis">
                                Right Benefit
                            </span>{" "}
                            helps you do thing we were just talking about above.
                        </span>
                    </div>
                </div>
                <div className="p-3 bg-body-tertiary border rounded-2">
                    <span className="fs-4 text-secondary">
                        <span className="text-body-emphasis">Big Benefit</span>{" "}
                        helps you do thing we were just talking about above.
                    </span>
                </div>
            </div>

            <div className="pt-5 mt-5">
                <div>
                    <div className=" display-5 text-warning-emphasis mb-0 mb-md-2">
                        Last Call to Action
                    </div>
                    <p className=" display-5">
                        You should give a obvious path forward on each viewport,
                        most likely to sign up.
                    </p>
                </div>
            </div>
            <div className="pb-5 mb-5 pt-2 mt-2">
                <div className="p-3 bg-body-tertiary border rounded-2">
                    <span className="fs-4 text-secondary">
                        <span className="text-body-emphasis">Motivation</span>{" "}
                        to nudge towards the call to action.
                    </span>
                </div>
            </div>

            <div className="py-4 my-4">
                <Row>
                    <Col sm={12} md={6} lg={3}>
                        <div className="pb-4 fs-3 fst-italic">Izakaya</div>
                        <p className="fs-6 text-secondary">
                            Register for{" "}
                            <span className="text-secondary-emphasis">
                                Izakaya
                            </span>{" "}
                            to get a closer look at all the awesome features.
                        </p>
                        <p className="fst-italic text-secondary pb-1">
                            You don&apos;t have to use your actual email. They
                            aren&apos;t verified.
                        </p>
                        <Button onClick={() => navigate(`/sign-up`)}>
                            <FontAwesomeIcon
                                icon="pen-to-square"
                                className="me-1"
                            />
                            Sign up
                        </Button>
                    </Col>
                    <Col sm={12} md={6} lg={3}></Col>
                    <Col sm={12} md={6} lg={3}></Col>
                    <Col sm={12} md={6} lg={3}></Col>
                </Row>
            </div>

            <Stack className="text-center pt-4 mt-4" gap={3}>
                <div>
                    <picture className={cx("d-inline-block", footerLantern)}>
                        <img
                            src={svgRedPaperLantern}
                            className="img-fluid"
                            alt="Red paper lantern emoji"
                        />
                    </picture>
                </div>
                <div>
                    Made with
                    <div className={cx("d-inline-block mx-1", footerHeart)}>
                        <img src={svgPurpleHeart} alt="love" title="love" />
                    </div>
                    by <a href="https://ruk.si/">Ruksi</a>
                </div>
            </Stack>
        </Container>
    );
}
