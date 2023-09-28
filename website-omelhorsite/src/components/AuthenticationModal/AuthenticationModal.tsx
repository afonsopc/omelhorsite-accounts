import React, { useState } from "react";
import { Modal, Button, Form, Alert, InputGroup, Spinner } from "react-bootstrap";
import "./authenticationModal.scss";
import { authenticateAccount, createAccount, language } from "../../main";

interface AuthenticationModalProps {
  show: boolean;
  onHide: () => void;
  startingTab?: string;
}

const AuthenticationModal: React.FC<AuthenticationModalProps> = ({ show, onHide, startingTab }) => {
  const [activeTab, setActiveTab] = useState(startingTab || "sign-up");
  const [errorMessage, setErrorMessage] = useState("");
  const [showErrorMessage, setShowErrorMessage] = useState(false);
  const [showSuccessMessage, setShowSuccessMessage] = useState(false);
  const [email, setEmail] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [loading, setLoading] = useState(false);
  const [password, setPassword] = useState("");

  const handleTabChange = (tab: string) => {
    setActiveTab(tab);
    setShowErrorMessage(false);
    setShowSuccessMessage(false);
  };

  const handleAuthentication = async () => {
    setShowSuccessMessage(false);
    setShowErrorMessage(false);
    setLoading(true);
    if (activeTab == "sign-up") {
      try {
        const response = await createAccount(email, password);
        if (response.status === 200) {
          setShowSuccessMessage(true);
          setShowErrorMessage(false);
        }
        else {
          setShowErrorMessage(true);
          setShowSuccessMessage(false);
          setErrorMessage(`${response.status} ${response.statusText}`);
        }

      }
      catch (error) {
        setShowErrorMessage(true);
        setErrorMessage(language.dictionary.unknownError);

      }
    }
    else {
      try {
        const response = await authenticateAccount(email, password);
        if (response.status === 200) {
          setShowErrorMessage(false);
          localStorage.setItem("token", response.data);
          onHide();
          window.location.reload();
        }
        else {
          setShowErrorMessage(true);
          setShowSuccessMessage(false);
          setErrorMessage(`${response.status} ${response.statusText}`);
        }
      }
      catch (error) {
        setShowErrorMessage(true);
        setErrorMessage(language.dictionary.unknownError);
      }
    }
    setLoading(false);
  };


  const togglePasswordVisibility = () => {
    setShowPassword(!showPassword);
  };

  return (
    <Modal show={show} onHide={onHide}>
      <Modal.Header closeButton>
        <Modal.Title>{activeTab === "sign-in" ? `${language.dictionary.signIn}` : `${language.dictionary.signUp}`}</Modal.Title>
      </Modal.Header>
      <Modal.Body>
        <Form className="form">
          <Form.Group controlId="formBasicEmail">
            <Form.Label>{language.dictionary.email}</Form.Label>
            <Form.Control
              type="email"
              placeholder={language.dictionary.enterEmail}
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            />
          </Form.Group>

          <Form.Group controlId="formBasicPassword">
            <Form.Label>{language.dictionary.password}</Form.Label>
            <InputGroup>
              <Form.Control
                type={showPassword ? "text" : "password"}
                placeholder={language.dictionary.enterPassword}
                required
                value={password}
                onChange={(e) => setPassword(e.target.value)}
              />
              <Button
                variant="outline-secondary"
                onClick={togglePasswordVisibility}
              >
                {showPassword ? "Hide" : "Show"}
              </Button>
            </InputGroup>
          </Form.Group>

          <div className="extra-text-container">
            <p className="change-tab-text">
              {activeTab === "sign-in" ? `${language.dictionary.dont_have_account_yet}` : `${language.dictionary.already_have_account}`}
              <a
                href="#"
                className="auth-link"
                onClick={() => handleTabChange(activeTab === "sign-in" ? "sign-up" : "sign-in")}
              >
                {activeTab === "sign-in" ? `${language.dictionary.signUp}` : `${language.dictionary.signIn}`}
              </a>
            </p>
            <Alert className="alert-danger" hidden={!showErrorMessage}>{errorMessage}</Alert>
            <Alert className="alert-success" hidden={!showSuccessMessage}>{language.dictionary.confirmationEmailSent}</Alert>
          </div>
        </Form>
      </Modal.Body>
      <Modal.Footer>
        <Button variant="secondary" onClick={onHide}>
          {language.dictionary.close}
        </Button>
        <Button variant="primary" onClick={handleAuthentication} disabled={loading}>
          {loading ? 
            <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }}/>
            :
            (activeTab === "sign-in") ? `${language.dictionary.signIn}` : `${language.dictionary.signUp}`
          }
        </Button>
      </Modal.Footer>
    </Modal>
  );
};

export default AuthenticationModal;
