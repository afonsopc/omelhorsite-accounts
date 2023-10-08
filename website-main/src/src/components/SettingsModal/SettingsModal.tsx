import React, { useState } from "react";
import { Modal, Accordion, Alert } from "react-bootstrap";
import { language } from "../../main"; // Certifique-se de importar o dicionário de idiomas
import "./settingsModal.scss"
import EmailChangeForm from "./components/EmailChangeForm";
import PasswordChangeForm from "./components/PasswordChangeForm";
import LanguageChangeForm from "./components/LanguageChangeForm";
import DeleteAccountForm from "./components/DeleteAccountForm";
import LogoutButton from "./components/LogoutButton";
import { deleteAccount, deleteAccountConfirm, ConfirmationCode } from "../../services/accountService";
import ConfirmationCodeModal from "../ConfirmationCodeModal/ConfirmationCodeModal";

interface SettingsModalProps {
  show: boolean;
  onHide: () => void;
}

enum Actions {
  Email = "email",
  Username = "username",
  Password = "password",
  Delete = "delete",
}

const SettingsModal: React.FC<SettingsModalProps> = ({ show, onHide }) => {
  const [deleteAccountErrorMessage, setDeleteAccountErrorMessage] = useState("");
  const [loadingDeleteAccount, setLoadingDeleteAccount] = useState(false);
  const [showConfirmationCodeModal, setShowConfirmationCodeModal] = useState(false);
  const [showConfirmationCodeModalErrorMessage, setShowConfirmationCodeModalErrorMessage] = useState(false);
  const [currentAction, setCurrentAction] = useState<Actions>(Actions.Email);

  const handleDeleteAccount = async () => {
    setLoadingDeleteAccount(true);
    let statusCode = await deleteAccount();
    if (statusCode === 200) {
      setCurrentAction(Actions.Delete);
      setShowConfirmationCodeModal(true);
    }
    else {
      setDeleteAccountErrorMessage(`Error code: ${statusCode}`);
    }
    setLoadingDeleteAccount(false);
  };

  const handleConfirmationCodeModalConfirm = async (code: string) => {
    let confirmationCode: ConfirmationCode = {
      confirmationCode: code
    };

    if (currentAction === Actions.Delete) {
      let status_code = await deleteAccountConfirm(confirmationCode);
      if (status_code === 200) {
        window.location.reload();
      }
    }
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
              <EmailChangeForm />
            </Accordion.Body>
          </Accordion.Item>

          <Accordion.Item eventKey="1">
            <Accordion.Header>{language.dictionary.changePassword}</Accordion.Header>
            <Accordion.Body>
              <PasswordChangeForm />
            </Accordion.Body>
          </Accordion.Item>

          <Accordion.Item eventKey="2">
            <Accordion.Header>{language.dictionary.changeLanguage}</Accordion.Header>
            <Accordion.Body>
              <LanguageChangeForm />
            </Accordion.Body>
          </Accordion.Item>

          <Accordion.Item eventKey="3">
            <Accordion.Header>{language.dictionary.deleteAccount}</Accordion.Header>
            <Accordion.Body>
              <DeleteAccountForm
                loading={loadingDeleteAccount}
                onDelete={() => handleDeleteAccount()}
              />
              <br />
              <Alert variant="danger" hidden={!deleteAccountErrorMessage}>{deleteAccountErrorMessage}</Alert>
            </Accordion.Body>
          </Accordion.Item>

        </Accordion>
        <LogoutButton />
      </Modal.Body>
      <ConfirmationCodeModal
        show={showConfirmationCodeModal}
        showErrorMessage={showConfirmationCodeModalErrorMessage}
        onHide={() => setShowConfirmationCodeModal(false)}
        onConfirm={(value) => handleConfirmationCodeModalConfirm(value)}
      />
    </Modal>
  );
};

export default SettingsModal;
