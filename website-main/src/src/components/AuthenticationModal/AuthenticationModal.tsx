import React, { useState } from "react";
import { Modal, Button, Alert, Spinner } from "react-bootstrap";
import "./authenticationModal.scss";
import { language } from "../../main";
import { SignInCredentials, SignUpCredentials, signIn, signUpConfirm, ConfirmationCode, signUp } from "../../services/accountService";
import ConfirmationCodeModal from "../ConfirmationCodeModal/ConfirmationCodeModal";
import SignInForm from "./components/SignInForm";
import SignUpForm from "./components/SignUpForm";

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
  const [loading, setLoading] = useState(false);
  const [email, setEmail] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const [confirmationCodeModalLoading, setConfirmationCodeModalLoading] = useState(false);
  const [showConfirmationCodeModal, setShowConfirmationCodeModal] = useState(false);
  const [showConfirmationCodeModalErrorMessage, setShowConfirmationCodeModalErrorMessage] = useState(false);

  const handleTabChange = (tab: string) => {
    setActiveTab(tab);
    setShowErrorMessage(false);
    setShowSuccessMessage(false);
  };

  const handleSubmitButtonClick = async () => {
    setShowErrorMessage(false);
    setShowSuccessMessage(false);
    setLoading(true);

    if (activeTab == "sign-in") {
      let signInCredentials: SignInCredentials = {
        email: email,
        password: password
      };

      let response = await signIn(signInCredentials);
      if (response.statusCode == 200) {
        window.location.reload();
      }
      else {
        setErrorMessage(`Error code: ${response.statusCode}`);
        setShowErrorMessage(true);
      }
    }
    else {
      let signUpCredentials: SignUpCredentials = {
        username: username,
        email: email,
        password: password,
      };

      let response = await signUp(signUpCredentials);
      if (response.statusCode == 200) {
        setShowConfirmationCodeModal(true);
      }
      else {
        setErrorMessage(`Error code: ${response.statusCode}`);
        setShowErrorMessage(true);
      }
    }

    setLoading(false);
  }

  const onConfirmationCodeConfirm = async (code: string) => {
    setShowConfirmationCodeModalErrorMessage(false);
    setConfirmationCodeModalLoading(true);

    let confirmationCode: ConfirmationCode = {
      confirmationCode: code
    };

    let response = await signUpConfirm(confirmationCode);
    if (response.statusCode === 200) {
      setShowConfirmationCodeModal(false);
      window.location.reload();
      return;
    }
    setShowConfirmationCodeModalErrorMessage(true);
    setConfirmationCodeModalLoading(false);
  }

  return (
    <div>
      <Modal show={showConfirmationCodeModal ? false : show} onHide={onHide} >
        <Modal.Header closeButton>
          <Modal.Title>
            {(activeTab === "sign-in") ?
              `${language.dictionary.signIn}`
              :
              `${language.dictionary.signUp}`
            }
          </Modal.Title>
        </Modal.Header>
        <Modal.Body>

          {(activeTab === "sign-in") ?
            <SignInForm
              email={email} onEmailChange={(e) => setEmail(e.currentTarget.value)}
              password={password} onPasswordChange={(e) => setPassword(e.currentTarget.value)}
            />
            :
            <SignUpForm
              email={email} onEmailChange={(e) => setEmail(e.currentTarget.value)}
              username={username} onUsernameChange={(e) => setUsername(e.currentTarget.value)}
              password={password} onPasswordChange={(e) => setPassword(e.currentTarget.value)}
            />
          }

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
          </div>

          <Alert className="alert-danger" hidden={!showErrorMessage}>{errorMessage}</Alert>
          <Alert className="alert-success" hidden={!showSuccessMessage}>{language.dictionary.confirmationEmailSent}</Alert>

        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={onHide}>
            {language.dictionary.close}
          </Button>
          <Button variant="primary" disabled={loading} onClick={handleSubmitButtonClick}>
            {loading ?
              <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }} />
              :
              (activeTab === "sign-in") ? `${language.dictionary.signIn}` : `${language.dictionary.signUp}`
            }
          </Button>
        </Modal.Footer>
      </Modal >
      <ConfirmationCodeModal
        show={showConfirmationCodeModal}
        showErrorMessage={showConfirmationCodeModalErrorMessage}
        onHide={() => setShowConfirmationCodeModal(false)}
        onConfirm={(value) => onConfirmationCodeConfirm(value)}
        message={language.dictionary.confirmationEmailSent}
        loading={confirmationCodeModalLoading}
      />
    </div>
  );
};

export default AuthenticationModal;
