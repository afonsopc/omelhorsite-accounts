import React, { useEffect, useState } from "react";
import { Modal, Accordion, Alert } from "react-bootstrap";
import { language } from "../../main"; // Certifique-se de importar o dicionário de idiomas
import "./settingsModal.scss"
import EmailChangeForm from "./components/EmailChangeForm";
import PasswordChangeForm from "./components/PasswordChangeForm";
import DeleteAccountForm from "./components/DeleteAccountForm";
import LogoutButton from "./components/LogoutButton";
import { deleteAccount, deleteAccountConfirm, ConfirmationCode, cancelConfirmations, changeUsername, changeUsernameConfirm, changePassword, changePasswordConfirm, changeEmail, changeEmailStepOneConfirm, changeEmailStepTwoConfirm, getAccountInfo } from "../../services/accountService";
import ConfirmationCodeModal from "../ConfirmationCodeModal/ConfirmationCodeModal";
import CancelConfirmationsButton from "./components/CancelConfirmationsButton";
import UsernameChangeForm from "./components/UsernameChangeForm";
import ThemeSelector from "../ThemeSelector/ThemeSelector";

interface SettingsModalProps {
    show: boolean;
    onHide: () => void;
}

enum Actions {
    EmailStepOne = "emailStepOne",
    EmailStepTwo = "emailStepTwo",
    Username = "username",
    Password = "password",
    Delete = "delete",
    None = "none",
}

const SettingsModal: React.FC<SettingsModalProps> = ({ show, onHide }) => {
    const [deleteAccountErrorMessage, setDeleteAccountErrorMessage] = useState("");
    const [loadingDeleteAccount, setLoadingDeleteAccount] = useState(false);

    const [showConfirmationCodeModal, setShowConfirmationCodeModal] = useState(false);
    const [confirmationCodeModalLoading, setConfirmationCodeModalLoading] = useState(false);
    const [confirmationCodeModalMessage, setConfirmationCodeModalMessage] = useState("");
    const [showConfirmationCodeModalErrorMessage, setShowConfirmationCodeModalErrorMessage] = useState(false);
    const [currentAction, setCurrentAction] = useState<Actions>(Actions.None);

    const [loadingCancelConfirmations, setLoadingCancelConfirmations] = useState(false);
    const [showCancelConfirmationsSuccessMessage, setShowCancelConfirmationsSuccessMessage] = useState(false);
    const [cancelConfirmationsErrorMessage, setCancelConfirmationsErrorMessage] = useState("");

    const [loadingChangeUsername, setLoadingChangeUsername] = useState(false);
    const [changeUsernameErrorMessage, setChangeUsernameErrorMessage] = useState("");

    const [loadingChangePassword, setLoadingChangePassword] = useState(false);
    const [changePasswordErrorMessage, setChangePasswordErrorMessage] = useState("");

    const [loadingChangeEmail, setLoadingChangeEmail] = useState(false);
    const [changeEmailErrorMessage, setChangeEmailErrorMessage] = useState("");

    const [currentUsername, setCurrentUsername] = useState("");
    const [currentEmail, setCurrentEmail] = useState("");

    useEffect(() => {
        const fetchData = async () => {
            const data = await getAccountInfo();
            if (data.data && data.data.username) {
                setCurrentUsername(data.data.username);
                setCurrentEmail(data.data.email);
            }
        }

        fetchData();
    }, [])

    const handleConfirmationCodeModalConfirm = async (code: string) => {
        let confirmationCode: ConfirmationCode = {
            confirmationCode: code
        };

        setConfirmationCodeModalMessage("");
        setShowConfirmationCodeModalErrorMessage(false);
        setConfirmationCodeModalLoading(true);

        if (currentAction === Actions.Delete) {
            let response = await deleteAccountConfirm(confirmationCode);
            if (response.statusCode === 200) {
                window.location.reload();
            }
            else {
                setShowConfirmationCodeModalErrorMessage(true);
            }
        }
        else if (currentAction === Actions.Username) {
            let response = await changeUsernameConfirm(confirmationCode);
            if (response.statusCode === 200) {
                window.location.reload();
            }
            else {
                setShowConfirmationCodeModalErrorMessage(true);
            }
        }
        else if (currentAction === Actions.Password) {
            let response = await changePasswordConfirm(confirmationCode);
            if (response.statusCode === 200) {
                window.location.reload();
            }
            else {
                setShowConfirmationCodeModalErrorMessage(true);
            }
        }
        else if (currentAction === Actions.EmailStepOne) {
            let response = await changeEmailStepOneConfirm(confirmationCode);
            if (response.statusCode === 200) {
                setConfirmationCodeModalMessage(language.dictionary.confirmationEmailSentToNew);
                setCurrentAction(Actions.EmailStepTwo);
                setConfirmationCodeModalLoading(false);
            }
            else {
                setShowConfirmationCodeModalErrorMessage(true);
            }
        }
        else if (currentAction === Actions.EmailStepTwo) {
            let response = await changeEmailStepTwoConfirm(confirmationCode);
            if (response.statusCode === 200) {
                window.location.reload();
            }
            else {
                setShowConfirmationCodeModalErrorMessage(true);
            }
        }
        setConfirmationCodeModalLoading(false);
    };


    const handleCancelConfirmations = async () => {
        setLoadingCancelConfirmations(true);

        setCancelConfirmationsErrorMessage("");
        setShowCancelConfirmationsSuccessMessage(false);

        let response = await cancelConfirmations();
        if (response.statusCode === 200) {
            setShowCancelConfirmationsSuccessMessage(true);
        }
        else {
            setCancelConfirmationsErrorMessage(`Error code: ${response.statusCode}`);
        }
        setLoadingCancelConfirmations(false);
    };


    const handleDeleteAccount = async () => {
        setLoadingDeleteAccount(true);

        setDeleteAccountErrorMessage("");

        let response = await deleteAccount();
        if (response.statusCode === 200) {
            setCurrentAction(Actions.Delete);
            setConfirmationCodeModalMessage(language.dictionary.confirmationEmailSent);
            setShowConfirmationCodeModal(true);
        }
        else {
            setDeleteAccountErrorMessage(`Error code: ${response.statusCode}`);
        }
        setLoadingDeleteAccount(false);
    };

    const handleChangeUsername = async (username: string) => {
        setLoadingChangeUsername(true);

        setChangeUsernameErrorMessage("");

        let response = await changeUsername(username);
        if (response.statusCode === 200) {
            setCurrentAction(Actions.Username);
            setConfirmationCodeModalMessage(language.dictionary.confirmationEmailSent);
            setShowConfirmationCodeModal(true);
        }
        else {
            setChangeUsernameErrorMessage(`Error code: ${response.statusCode}`);
        }
        setLoadingChangeUsername(false);
    };


    const handleChangePassword = async (password: string) => {
        setLoadingChangePassword(true);

        setChangePasswordErrorMessage("");

        let response = await changePassword(password);
        if (response.statusCode === 200) {
            setCurrentAction(Actions.Password);
            setConfirmationCodeModalMessage(language.dictionary.confirmationEmailSent);
            setShowConfirmationCodeModal(true);
        }
        else {
            setChangePasswordErrorMessage(`Error code: ${response.statusCode}`);
        }
        setLoadingChangePassword(false);
    };


    const handleChangeEmail = async (email: string) => {
        setLoadingChangeEmail(true);

        setChangePasswordErrorMessage("");

        let response = await changeEmail(email);
        if (response.statusCode === 200) {
            setCurrentAction(Actions.EmailStepOne);
            setConfirmationCodeModalMessage(language.dictionary.confirmationEmailSentToOriginal);
            setShowConfirmationCodeModal(true);
        }
        else {
            setChangeEmailErrorMessage(`Error code: ${response.statusCode}`);
        }
        setLoadingChangeEmail(false);
    };

    return (
        <div>
            <Modal show={showConfirmationCodeModal ? false : show} onHide={onHide}>
                <Modal.Header closeButton>
                    <Modal.Title>{language.dictionary.settings}</Modal.Title>
                </Modal.Header>

                <Modal.Body className="forms-container">

                    <ThemeSelector />
                    <CancelConfirmationsButton loading={loadingCancelConfirmations} onCancel={() => handleCancelConfirmations()} />
                    <Alert variant="danger" hidden={!cancelConfirmationsErrorMessage}>{cancelConfirmationsErrorMessage}</Alert>
                    <Alert variant="success" hidden={!showCancelConfirmationsSuccessMessage}>{language.dictionary.pendingConfirmationsCanceledSuccessfully}</Alert>

                    <Accordion defaultActiveKey="0">

                        <Accordion.Item eventKey="0">
                            <Accordion.Header>{language.dictionary.changeEmail}</Accordion.Header>
                            <Accordion.Body className="form">
                                <EmailChangeForm
                                    currentEmail={currentEmail}
                                    loading={loadingChangeEmail}
                                    onSubmit={(email) => handleChangeEmail(email)}
                                />
                                <Alert variant="danger" hidden={!changeEmailErrorMessage}>{changeEmailErrorMessage}</Alert>
                            </Accordion.Body>
                        </Accordion.Item>

                        <Accordion.Item eventKey="1">
                            <Accordion.Header>{language.dictionary.changeUsername}</Accordion.Header>
                            <Accordion.Body className="form">
                                <UsernameChangeForm
                                    currentUsername={currentUsername}
                                    loading={loadingChangeUsername}
                                    onSubmit={(username) => handleChangeUsername(username)}
                                />
                                <Alert variant="danger" hidden={!changeUsernameErrorMessage}>{changeUsernameErrorMessage}</Alert>
                            </Accordion.Body>
                        </Accordion.Item>

                        <Accordion.Item eventKey="2">
                            <Accordion.Header>{language.dictionary.changePassword}</Accordion.Header>
                            <Accordion.Body className="form">
                                <PasswordChangeForm
                                    loading={loadingChangePassword}
                                    onSubmit={(password) => handleChangePassword(password)}
                                />
                                <Alert variant="danger" hidden={!changePasswordErrorMessage}>{changePasswordErrorMessage}</Alert>
                            </Accordion.Body>
                        </Accordion.Item>

                        <Accordion.Item eventKey="3">
                            <Accordion.Header>{language.dictionary.deleteAccount}</Accordion.Header>
                            <Accordion.Body className="form">
                                <DeleteAccountForm
                                    loading={loadingDeleteAccount}
                                    onDelete={() => handleDeleteAccount()}
                                />
                                <Alert variant="danger" hidden={!deleteAccountErrorMessage}>{deleteAccountErrorMessage}</Alert>
                            </Accordion.Body>
                        </Accordion.Item>

                    </Accordion>
                    <LogoutButton />
                </Modal.Body>
            </Modal>
            <ConfirmationCodeModal
                show={showConfirmationCodeModal}
                showErrorMessage={showConfirmationCodeModalErrorMessage}
                onHide={() => setShowConfirmationCodeModal(false)}
                onConfirm={(value) => handleConfirmationCodeModalConfirm(value)}
                message={confirmationCodeModalMessage}
                loading={confirmationCodeModalLoading}
            />

        </div>
    );
};

export default SettingsModal;