import {faPenToSquare} from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import Image from "next/image";
import Link from "next/link";
import {
    Button,
    Col,
    Container,
    Form,
    InputGroup,
    Row,
    Stack,
} from "react-bootstrap";
import PurpleHeartSVG from "../../public/purple-heart.svg";
import RedPaperLanternSVG from "../../public/red-paper-lantern.svg";
import styles from "./page.module.scss";

export default function Home() {
    return (
        <Container className="py-3 my-3 py-lg-5 mt-lg-5">
            <div className="pt-2 mt-2 pt-md-4 mt-md-4 pt-lg-5 mt-lg-5">
                <div className="display-3 fw-semibold d-inline-flex">
                    <picture
                        className={`d-inline-flex align-self-center me-4 ${styles.headerLantern}`}
                    >
                        <Image
                            src={RedPaperLanternSVG}
                            width={80}
                            height={80}
                            alt="red paper lantern emoji"
                            priority
                        />
                    </picture>
                    This is Ryokan landing page
                </div>
                <div className="fs-4 text-secondary mt-2 mt-md-3">
                    Most pages, like this one, are initially static sites for
                    SEO.
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
                            <Link href="/sign-up">
                                <Button>
                                    <FontAwesomeIcon
                                        icon={faPenToSquare}
                                        className="me-1"
                                    />
                                    Sign up
                                </Button>
                            </Link>
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
                        <div className="pb-4 fs-3 fst-italic">Ryokan</div>
                        <p className="fs-6 text-secondary">
                            Register for{" "}
                            <span className="text-secondary-emphasis">
                                Ryokan
                            </span>{" "}
                            to get a closer look at all the awesome features.
                        </p>
                        <p className="fst-italic text-secondary pb-1">
                            You don&apos;t have to use your actual email. They
                            aren&apos;t verified.
                        </p>
                        <Link href="/sign-up">
                            <Button>
                                <FontAwesomeIcon
                                    icon={faPenToSquare}
                                    className="me-1"
                                />
                                Sign up
                            </Button>
                        </Link>
                    </Col>
                    <Col sm={12} md={6} lg={3}></Col>
                    <Col sm={12} md={6} lg={3}></Col>
                    <Col sm={12} md={6} lg={3}></Col>
                </Row>
            </div>

            <Stack className="text-center pt-4 mt-4" gap={3}>
                <div>
                    <picture className={styles.footerLantern}>
                        <Image
                            src={RedPaperLanternSVG}
                            width={72}
                            height={72}
                            alt="red paper lantern emoji"
                        />
                    </picture>
                </div>
                <div>
                    Made with
                    <div className="d-inline-block mx-1">
                        <Image
                            src={PurpleHeartSVG}
                            width={32}
                            height={32}
                            alt="a purple heart"
                            title="love"
                        />
                    </div>
                    by <a href="https://ruk.si/">Ruksi</a>
                </div>
            </Stack>
        </Container>
    );
}
