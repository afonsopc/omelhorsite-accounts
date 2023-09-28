import React, { useState } from "react";
import { Modal, Button, Form, InputGroup, Accordion, Alert, Spinner } from "react-bootstrap";
import { changeEmail, changePassword, deleteAccount, language, logout } from "../../main"; // Certifique-se de importar o dicionário de idiomas
import "./settingsModal.scss"
import LanguageSelector from "../LanguageSelector/LanguageSelector";
import { Language, languages } from "../../translations";

interface SettingsModalProps {
  show: boolean;
  onHide: () => void;
}

const SettingsModal: React.FC<SettingsModalProps> = ({ show, onHide }) => {
  const [newEmail, setNewEmail] = useState<string>("");
  const [newPassword, setNewPassword] = useState<string>("");
  const [showNewPassword, setShowNewPassword] = useState(false);
  const [selectedLanguage, setSelectedLanguage] = useState<Language>(language); // Estado para armazenar o idioma selecionado
  const [showNewEmailErrorMessage, setShowNewEmailErrorMessage] = useState<boolean>(false);
  const [showNewEmailSuccessMessage, setShowNewEmailSuccessMessage] = useState<boolean>(false);
  const [loadingNewEmail, setLoadingNewEmail] = useState<boolean>(false);
  const [newEmailErrorMessage, setNewEmailErrorMessage] = useState<string>("");
  const [showNewPasswordErrorMessage, setShowNewPasswordErrorMessage] = useState<boolean>(false);
  const [showNewPasswordSuccessMessage, setShowNewPasswordSuccessMessage] = useState<boolean>(false);
  const [loadingNewPassword, setLoadingNewPassword] = useState<boolean>(false);
  const [newPasswordErrorMessage, setNewPasswordErrorMessage] = useState<string>("");
  const [showDeleteAccountErrorMessage, setShowDeleteAccountErrorMessage] = useState<boolean>(false);
  const [showDeleteAccountSuccessMessage, setShowDeleteAccountSuccessMessage] = useState<boolean>(false);
  const [deleteAccountErrorMessage, setDeleteAccountErrorMessage] = useState<string>("");
  const [loadingDeleteAccount, setLoadingDeleteAccount] = useState<boolean>(false);

  const handleEmailChange = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoadingNewEmail(true);
    if (!newEmail) { return }
    try {
      const response = await changeEmail(newEmail);
      if (response.status === 200) {
        setShowNewEmailErrorMessage(false);
        setShowNewEmailSuccessMessage(true)
      }
      else {
        setShowNewEmailErrorMessage(true);
        setShowNewEmailSuccessMessage(false);
        setNewEmailErrorMessage(`${response.status} ${response.statusText}`);
      }
    }
    catch (error) {
      setShowNewEmailErrorMessage(true);
      setNewEmailErrorMessage(language.dictionary.unknownError);
    }
    setLoadingNewEmail(false);
  };

  const handlePasswordChange = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoadingNewPassword(true);
    if (!newPassword) { return }
    try {
      const response = await changePassword(newPassword);
      if (response.status === 200) {
        setShowNewPasswordErrorMessage(false);
        setShowNewPasswordSuccessMessage(true)
      }
      else {
        setShowNewPasswordErrorMessage(true);
        setShowNewPasswordSuccessMessage(false);
        setNewPasswordErrorMessage(`${response.status} ${response.statusText}`);
      }
    }
    catch (error) {
      setShowNewPasswordErrorMessage(true);
      setNewPasswordErrorMessage(language.dictionary.unknownError);
    }
    setLoadingNewPassword(false);
  };

  const handleDeleteAccount = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoadingDeleteAccount(true);
    if (!window.confirm(language.dictionary.deleteAccountConfirmation)) { 
      setShowDeleteAccountErrorMessage(false);
      setShowDeleteAccountSuccessMessage(false);
      setLoadingDeleteAccount(false);
      return;
     };
    
    try {
      const response = await deleteAccount();
      if (response.status === 200) {
        setShowDeleteAccountErrorMessage(false);
        setShowDeleteAccountSuccessMessage(true)
      }
      else {
        setShowDeleteAccountErrorMessage(true);
        setShowDeleteAccountSuccessMessage(false);
        setDeleteAccountErrorMessage(`${response.status} ${response.statusText}`);
      }
    }
    catch (error) {
      setShowDeleteAccountErrorMessage(true);
      setDeleteAccountErrorMessage(language.dictionary.unknownError);
    }
    setLoadingDeleteAccount(true);
  };

  const handleLanguageChange = () => {
    localStorage.setItem("language", selectedLanguage.code);
    window.location.reload();
  };

  const toggleNewPasswordVisibility = () => {
    setShowNewPassword(!showNewPassword);
  };

  return (
    <Modal show={show} onHide={onHide}>
      <Modal.Header closeButton>
        <Modal.Title>{language.dictionary.settings}</Modal.Title>
      </Modal.Header>

      <Modal.Body className="forms-container">

        <Accordion defaultActiveKey="0">
            <Accordion.Item eventKey="0">
              <Accordion.Header>{language.dictionary.changeEmail}</Accordion.Header>
              <Accordion.Body>
                <Form onSubmit={handleEmailChange} className="form-container">
                  <Form.Group controlId="email">
                    <Form.Label>{language.dictionary.newEmail}</Form.Label>
                    
                    <Form.Control
                    type="email"
                    placeholder={language.dictionary.enterNewEmail}
                    value={newEmail}
                    onChange={(e) => setNewEmail(e.target.value)}
                    autoComplete="email"
                  />
                </Form.Group>

                <Alert className="alert-danger" hidden={!showNewEmailErrorMessage}>{newEmailErrorMessage}</Alert>
                <Alert className="alert-success" hidden={!showNewEmailSuccessMessage}>{language.dictionary.confirmationEmailSent}</Alert>

                <Button variant="primary" type="submit" disabled={!newEmail || loadingNewEmail}>
                  {loadingNewEmail ? 
                      <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }}/>
                    :
                      language.dictionary.change
                  }
                </Button>
              </Form>
            </Accordion.Body>
          </Accordion.Item>
          <Accordion.Item eventKey="1">
            <Accordion.Header>{language.dictionary.changePassword}</Accordion.Header>
            <Accordion.Body>
              <Form onSubmit={handlePasswordChange} className="form-container">
                <Form.Group controlId="newPassword" className="form-container">
                  <Form.Label>{language.dictionary.newPassword}</Form.Label>
                  <InputGroup>
                    <Form.Control
                      type={showNewPassword ? "text" : "password"}
                      placeholder={language.dictionary.enterNewPassword}
                      required
                      value={newPassword}
                      onChange={(e) => setNewPassword(e.target.value)}
                      autoComplete="new-password"
                    />
                    <Button
                      variant="outline-secondary"
                      onClick={toggleNewPasswordVisibility}
                    >
                      {showNewPassword ? `${language.dictionary.hide}` : `${language.dictionary.show}`}
                    </Button>
                  </InputGroup>
                </Form.Group>
                <Alert className="alert-danger" hidden={!showNewPasswordErrorMessage}>{newPasswordErrorMessage}</Alert>
                <Alert className="alert-success" hidden={!showNewPasswordSuccessMessage}>{language.dictionary.confirmationEmailSent}</Alert>
                <Button variant="primary" type="submit" disabled={!newPassword || loadingNewPassword}>
                      {loadingNewPassword ?
                          <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }}/>
                        :
                          language.dictionary.change
                      }
                </Button>
              </Form>
            </Accordion.Body>
          </Accordion.Item>
          <Accordion.Item eventKey="2">
            <Accordion.Header>{language.dictionary.changeLanguage}</Accordion.Header>
            <Accordion.Body>
              <Form.Group controlId="selectedLanguage" className="form-container">
                <Form.Label>{language.dictionary.language}</Form.Label>
                <LanguageSelector 
                  selectedLanguage={selectedLanguage}
                  onChange={(language) => setSelectedLanguage(language)} languages={languages}                />
                <Button variant="primary" onClick={handleLanguageChange} disabled={selectedLanguage === language}>
                  {language.dictionary.change}
                </Button>
              </Form.Group>
            </Accordion.Body>
          </Accordion.Item>
          <Accordion.Item eventKey="3">
            <Accordion.Header>{language.dictionary.deleteAccount}</Accordion.Header>
            <Accordion.Body>
              <Form.Group controlId="selectedLanguage" className="form-container">
                
                <Alert className="alert-danger" hidden={!showDeleteAccountErrorMessage}>{deleteAccountErrorMessage}</Alert>
                <Alert className="alert-warning" hidden={!showDeleteAccountSuccessMessage}>{language.dictionary.confirmationEmailSentSad}</Alert>
                <Button variant="danger" onClick={handleDeleteAccount} disabled={loadingDeleteAccount}>
                  {loadingDeleteAccount ? 
                      <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }}/>
                    :
                      language.dictionary.deleteAccount
                  }
                </Button>
              </Form.Group>
            </Accordion.Body>
          </Accordion.Item>
        </Accordion>
          <Button variant="outline-danger" onClick={logout}>
            {language.dictionary.logout}
          </Button>   
      </Modal.Body>
    </Modal>
  );
};

export default SettingsModal;
