import "./home.scss"
import { language, verifyAccountToken } from "../../main";
import { Container, Row, Col, Card, Button } from "react-bootstrap";
import { useEffect, useState } from "react";
import AuthenticationModal from "../../components/AuthenticationModal/AuthenticationModal";

const Home = () => {
  const [showAuthenticationModal, setShowAuthenticationModal] = useState(false);
  const [isAccountTokenValid, setIsAccountTokenValid] = useState(false);

  useEffect(() => {
    const verify = async () => {
       const response = await verifyAccountToken();
       if (response.status === 200) {
         setIsAccountTokenValid(true);
       }
       else {
        setIsAccountTokenValid(false);
       }
    }
  
    verify();
  }, [])

  const handleShowAuthenticationModal = () => {
    setShowAuthenticationModal(true);
  };

  const handleCloseAuthenticationModal = () => {
    setShowAuthenticationModal(false);
  };
  return (
    <div>
      <Container className="px-4 pt-5 my-5 text-center">
        <Row className="justify-content-center">
          <Col lg={6}>
            <h1 className="display-4 fw-bold text-body-emphasis">{language.dictionary.homeTitle}</h1>
            <p className="lead mb-4">
              {language.dictionary.homeDescription}
            </p>
          </Col>
        </Row>
        <Row className="row-cols-1 row-cols-md-3 mb-3 text-center align-items-center justify-content-center">
          <Col>
            <Card className="mb-4 rounded-3 shadow-sm">
              <Card.Header className="py-3">
                <h4 className="my-0 fw-normal">{language.dictionary.free}</h4>
              </Card.Header>
              <Card.Body>
                <Card.Title className="pricing-card-title">{language.dictionary.freePlanPrice}€ <small className="text-body-secondary fw-light">/ {language.dictionary.monthsShort}</small></Card.Title>
                <ul className="list-unstyled mt-3 mb-4">
                  <li>{language.dictionary.freePlan1}</li>
                </ul>
                <Button onClick={handleShowAuthenticationModal} variant="outline-primary" className="w-100 btn-lg" disabled={isAccountTokenValid ? true : false}>{language.dictionary.signUp}</Button>
              </Card.Body>
            </Card>
          </Col>
          <Col>
            <Card className="mb-4 rounded-3 shadow-sm border-primary">
              <Card.Header className="py-3 text-bg-primary border-primary">
                <h4 className="my-0 fw-normal">{language.dictionary.premium}</h4>
              </Card.Header>
              <Card.Body>
                <Card.Title className="pricing-card-title">{language.dictionary.premiumPlanPrice}€ <small className="text-body-secondary fw-light">/ {language.dictionary.monthsShort}</small></Card.Title>
                <ul className="list-unstyled mt-3 mb-4">
                  <li>{language.dictionary.premiumPlan1}</li>
                </ul>
                <Button variant="primary" className="w-100 btn-lg" disabled={isAccountTokenValid ? true : false || true}>{language.dictionary.unavailable}</Button>
              </Card.Body>
            </Card>
          </Col>
        </Row>
      </Container>
      <AuthenticationModal show={showAuthenticationModal} onHide={handleCloseAuthenticationModal}  startingTab="sign-up"/>
    </div>
  )
}

export default Home